#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    block::ImageDef,
    gpio::{Flex, Pull},
    peripherals::UART1,
    uart::{BufferedInterruptHandler, BufferedUart, Config as UartConfig, StopBits, Parity},
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel;
use embedded_io_async::Write;
use static_cell::StaticCell;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::descriptor::KeyboardUsage;
use {defmt_rtt as _, panic_probe as _};
use 

use ssmarshal::serialize;

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    UART1_IRQ => BufferedInterruptHandler<UART1>;
});

static EVENT_CHANNEL: channel::Channel<CriticalSectionRawMutex, KeyboardUsage, 10> = channel::Channel::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let (tx_pin, rx_pin, uart) = (p.PIN_4, p.PIN_5, p.UART1);


    static TX_BUF: StaticCell<[u8; 1024]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; 1024])[..];
    static RX_BUF: StaticCell<[u8; 1024]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; 1024])[..];
    let mut config = UartConfig::default();
    config.parity = Parity::ParityEven;
    config.stop_bits = StopBits::STOP2;
    let uart = BufferedUart::new(uart, Irqs, tx_pin, rx_pin, tx_buf, rx_buf, config);
    let (mut tx, rx) = uart.split();

    let mut outputs = [
        Flex::new(p.PIN_8),
        Flex::new(p.PIN_9),
        Flex::new(p.PIN_10),
        Flex::new(p.PIN_11),
        Flex::new(p.PIN_12),
        Flex::new(p.PIN_13),
        Flex::new(p.PIN_14),
        Flex::new(p.PIN_15),
        Flex::new(p.PIN_16),
        Flex::new(p.PIN_17),
        Flex::new(p.PIN_18),
        Flex::new(p.PIN_19),
        Flex::new(p.PIN_20),
        Flex::new(p.PIN_21),
    ];

    let mut inputs = [
        Flex::new(p.PIN_3),
        Flex::new(p.PIN_26),
        Flex::new(p.PIN_22),
        Flex::new(p.PIN_6),
        Flex::new(p.PIN_7),
    ];

    for input in &mut inputs {
        input.set_low();
        input.set_as_input();
        input.set_schmitt(true);
        input.set_pull(Pull::Down);
    }

    for output in &mut outputs {
        output.set_as_output();
        output.set_low();
    }

    // you can maybe use variant_count
    let mut state: [bool; 256] = [false; 256];

    loop {
        // should just be used for sleep?
        // for mut output in &mut outputs {
        //     output.set_high();
        // }
        // embassy_futures::select::select_array(inputs.map(|i| i.wait_for_any_edge())).await;

        for output in &mut outputs {
            output.set_low();
        }

        for input in &mut inputs {
            let mut last: Option<&mut Flex<'_>> = None;
            for output in &mut outputs {
                if let Some(last) = &mut last {
                    last.set_low();
                }
                output.set_high();
                let id = output.bit();
                last = Some(output);

                let mapped = mapping(input.bit().ilog2(), id.ilog2()) as usize;
                cortex_m::asm::delay(1_000_000);

                if state[mapped] && input.is_low() {
                    state[mapped] = false;

                    let mut buf: [u8; 9] = [0; 9];
                    let report = KeyboardReport {
                        keycodes: [0, 0, 0, 0, 0, 0],
                        leds: 0,
                        modifier: 0,
                        reserved: 0,
                    };
                    if let Ok(size) = serialize(&mut buf, &report) {
                        let mut send_buf: [u8; 11] = [2; 11];
                        send_buf[10] = 3;
                        send_buf[1..10].clone_from_slice(&buf);
                        defmt::info!("{:?}", send_buf);
                        tx.write_all(&send_buf).await.unwrap();
                    }
                } else if !state[mapped] && input.is_high() {
                    state[mapped] = true;
                    let mut buf: [u8; 9] = [0; 9];
                    let report = KeyboardReport {
                        keycodes: [mapped as u8, 0, 0, 0, 0, 0],
                        leds: 0,
                        modifier: 0,
                        reserved: 0,
                    };
                    if let Ok(size) = serialize(&mut buf, &report) {
                        let mut send_buf: [u8; 11] = [2; 11];
                        send_buf[10] = 3;
                        send_buf[1..10].clone_from_slice(&buf);
                        defmt::info!("{:?}", send_buf);
                        tx.write_all(&send_buf).await.unwrap();
                    }
                }
            }
            if let Some(last) = &mut last {
                last.set_low();
            }
        }
    }
}

fn mapping(input: u32, output: u32) -> KeyboardUsage {
    match input {
        // 1
        3 => match output {
            // A
            8 => KeyboardUsage::KeyboardBacktickTilde,
            // B
            9 => KeyboardUsage::Keyboard1Exclamation,
            // C
            10 => KeyboardUsage::Keyboard2At,
            // D
            11 => KeyboardUsage::Keyboard3Hash,
            // E
            12 => KeyboardUsage::Keyboard4Dollar,
            // F
            13 => KeyboardUsage::Keyboard5Percent,
            // G
            14 => KeyboardUsage::Keyboard6Caret,
            // H
            15 => KeyboardUsage::Keyboard7Ampersand,
            // I
            16 => KeyboardUsage::Keyboard8Asterisk,
            // J
            17 => KeyboardUsage::Keyboard9OpenParens,
            // K
            18 => KeyboardUsage::Keyboard0CloseParens,
            // L
            19 => KeyboardUsage::KeyboardBackspace,
            // M
            20 => KeyboardUsage::KeyboardCc,
            // N
            21 => KeyboardUsage::KeyboardCc,
            _ => KeyboardUsage::KeyboardBb,
        },
        // 2
        26 => match output {
            // A
            8 => KeyboardUsage::KeyboardTab,
            // B
            9 => KeyboardUsage::KeyboardQq,
            // C
            10 => KeyboardUsage::KeyboardWw,
            // D
            11 => KeyboardUsage::KeyboardEe,
            // E
            12 => KeyboardUsage::KeyboardRr,
            // F
            13 => KeyboardUsage::KeyboardTt,
            // G
            14 => KeyboardUsage::KeyboardYy,
            // H
            15 => KeyboardUsage::KeyboardUu,
            // I
            16 => KeyboardUsage::KeyboardIi,
            // J
            17 => KeyboardUsage::KeyboardOo,
            // K
            18 => KeyboardUsage::KeyboardPp,
            // L
            19 => KeyboardUsage::KeyboardOpenBracketBrace,
            // M
            20 => KeyboardUsage::KeyboardCloseBracketBrace,
            // N
            21 => KeyboardUsage::KeyboardBackslashBar,
            _ => KeyboardUsage::KeyboardBb,
        },
        // 3
        22 => match output {
            // A
            8 => KeyboardUsage::KeyboardCapsLock,
            // B
            9 => KeyboardUsage::KeyboardAa,
            // C
            10 => KeyboardUsage::KeyboardSs,
            // D
            11 => KeyboardUsage::KeyboardDd,
            // E
            12 => KeyboardUsage::KeyboardFf,
            // F
            13 => KeyboardUsage::KeyboardGg,
            // G
            14 => KeyboardUsage::KeyboardHh,
            // H
            15 => KeyboardUsage::KeyboardJj,
            // I
            16 => KeyboardUsage::KeyboardKk,
            // J
            17 => KeyboardUsage::KeyboardLl,
            // K
            18 => KeyboardUsage::KeyboardSemiColon,
            // L
            19 => KeyboardUsage::KeyboardSingleDoubleQuote,
            // M
            20 => KeyboardUsage::KeyboardCc,
            // N
            21 => KeyboardUsage::KeyboardCc,
            _ => KeyboardUsage::KeyboardBb,
        },
        // 4
        6 => match output {
            // A
            8 => KeyboardUsage::KeyboardLeftShift,
            // B
            9 => KeyboardUsage::KeyboardZz,
            // C
            10 => KeyboardUsage::KeyboardXx,
            // D
            11 => KeyboardUsage::KeyboardCc,
            // E
            12 => KeyboardUsage::KeyboardVv,
            // F
            13 => KeyboardUsage::KeyboardBb,
            // G
            14 => KeyboardUsage::KeyboardNn,
            // H
            15 => KeyboardUsage::KeyboardMm,
            // I
            16 => KeyboardUsage::KeyboardCommaLess,
            // J
            17 => KeyboardUsage::KeyboardPeriodGreater,
            // K
            18 => KeyboardUsage::KeyboardSlashQuestion,
            // L
            19 => KeyboardUsage::KeyboardCc,
            // M
            20 => KeyboardUsage::KeyboardCc,
            // N
            21 => KeyboardUsage::KeyboardCc,
            _ => KeyboardUsage::KeyboardBb,
        },
        // 5
        7 => match output {
            // A
            8 => KeyboardUsage::KeyboardLeftControl,
            // B
            9 => KeyboardUsage::KeyboardLeftAlt,
            // C
            10 => KeyboardUsage::KeyboardCc,
            // D
            11 => KeyboardUsage::KeyboardCc,
            // E
            12 => KeyboardUsage::KeyboardSpacebar,
            // F
            13 => KeyboardUsage::KeyboardEscape,
            // G
            14 => KeyboardUsage::KeyboardRightControl,
            // H
            15 => KeyboardUsage::KeyboardRightAlt,
            // I
            16 => KeyboardUsage::KeyboardCc,
            // J
            17 => KeyboardUsage::KeyboardCc,
            // K
            18 => KeyboardUsage::KeyboardCc,
            // L
            19 => KeyboardUsage::KeyboardCc,
            // M
            20 => KeyboardUsage::KeyboardCc,
            // N
            21 => KeyboardUsage::KeyboardCc,
            _ => KeyboardUsage::KeyboardBb,
        },
        _ => KeyboardUsage::KeyboardAa,
    }
}
