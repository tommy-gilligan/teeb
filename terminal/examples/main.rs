#![feature(ascii_char)]
#![feature(ascii_char_variants)]
use embedded_graphics::{pixelcolor::Rgb888, prelude::Size};
use embedded_graphics_simulator::{sdl2::Keycode, SimulatorEvent, OutputSettingsBuilder, SimulatorDisplay, Window};
use std::{
    io::{Read, Write, BufReader, BufRead},
    process::{ChildStderr, ChildStdout, Command, Stdio},
    sync::mpsc::{Receiver, TryRecvError, channel},
    thread,
    time
};
use terminal::Terminal;
use terminal::c1::Parser;
use terminal::csi::Parser as CsiParser;

#[tokio::main]
async fn main() -> Result<(), core::convert::Infallible> {
    let mut bash = Command::new("bash")
        .arg("--login")
        .arg("-i")
        .arg("-m")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .env("TERM", "xterm")
        .spawn()
        .unwrap();

    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(800, 300));
    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Hello World", &output_settings);
    let mut terminal: Terminal<'_> = Terminal::new();
    terminal.clear(&mut display);

    terminal.push(
        terminal::csi::Character::ControlSequenceIntroducer(
            terminal::csi::ControlSequenceIntroducer::SelectGraphicRendition(
                terminal::csi::select_graphic_rendition::SelectGraphicRendition::SetForegroundColor4
            )
        )
    );

    let mut stdout = bash.stdout.unwrap();
    let (tx_o, stdout_reader) = channel::<u8>();
    thread::spawn(move || loop {
        let mut buffer = [0];
        stdout.read(&mut buffer).unwrap();
        tx_o.send(buffer[0]);
    });

    let mut stderr = bash.stderr.unwrap();
    let (tx_e, stderr_reader) = channel::<u8>();
    thread::spawn(move || loop {
        let mut buffer = [0];
        stderr.read(&mut buffer).unwrap();
        tx_e.send(buffer[0]);
    });

    'running: loop {
        window.update(&display);

        let mut c1_parser_buffer: Vec<u8> = Vec::new();
        let mut csi_parser_buffer: Vec<terminal::c1::Character> = Vec::new();

        let mut err_c1_parser_buffer: Vec<u8> = Vec::new();
        let mut err_csi_parser_buffer: Vec<terminal::c1::Character> = Vec::new();

        'readerr: loop {
            match stderr_reader.try_recv() {
                Ok(key) => {
                    c1_parser_buffer.push(key);
                    let mut c1_parser = Parser::new(&c1_parser_buffer);
                    if let Some(c) = c1_parser.next() {
                        csi_parser_buffer.push(c);
                        let mut csi_parser = CsiParser::new(&csi_parser_buffer);
                        if let Some(d) = csi_parser.next() {
                            println!("{:?}", &d);
                            terminal.push(d);
                            csi_parser_buffer.clear();
                        }
                        c1_parser_buffer.clear();
                    }
                },
                Err(TryRecvError::Empty) => {
                    break 'readerr;
                },
                Err(TryRecvError::Disconnected) => {
                    break 'readerr;
                },
            }
        }
        'readout: loop {
            match stdout_reader.try_recv() {
                Ok(key) => {
                    err_c1_parser_buffer.push(key);
                    let mut c1_parser = Parser::new(&err_c1_parser_buffer);
                    if let Some(c) = c1_parser.next() {
                        err_csi_parser_buffer.push(c);
                        let mut csi_parser = CsiParser::new(&err_csi_parser_buffer);
                        if let Some(d) = csi_parser.next() {
                            println!("{:?}", &d);
                            terminal.push(d);
                            err_csi_parser_buffer.clear();
                        }
                        err_c1_parser_buffer.clear();
                    }
                },
                Err(TryRecvError::Empty) => {
                    break 'readout;
                },
                Err(TryRecvError::Disconnected) => {
                    break 'readout;
                },
            }
        }

        terminal.draw(&mut display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, keymod, .. } => {
                   let b = (keycode.into_i32() & 0xff) as u8;
                   let k = match (b, keymod) {
                       (39, embedded_graphics_simulator::sdl2::Mod::LSHIFTMOD) => 34,
                       (92, embedded_graphics_simulator::sdl2::Mod::LSHIFTMOD) => 124,
                       (b'4', embedded_graphics_simulator::sdl2::Mod::LSHIFTMOD) => 36,
                       (b'a'..b'z', embedded_graphics_simulator::sdl2::Mod::LSHIFTMOD) => b - 32,
                       _ => b
                   };

                    match core::ascii::Char::from_u8(k) {
                        Some(ascii_char) => {
                            bash.stdin.as_mut().unwrap().write(&[k]);
                        },
                        None => {
                            println!("{:?}", b);
                        }
                    }
                },
                _ => {}
            }
        }
    }

    Ok(())
}
