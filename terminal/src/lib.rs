#![no_std]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]

extern crate std;
use core::fmt::Debug;

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::{
    draw_target::DrawTarget, mono_font::MonoTextStyle, prelude::*, text::Text,
};
use heapless::HistoryBuffer;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::geometry::Point;
use crate::csi::ControlSequenceIntroducer::EraseInDisplay;

pub mod c1;
pub mod csi;
pub mod state;
pub mod wrap;
pub mod osc;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Character {
    Char(core::ascii::Char),
    C1Escape(c1::C1Escape),
    Unrecognized(u8),
    OperatingSystemCommand(osc::OperatingSystemCommand),
    ControlSequenceIntroducer(csi::ControlSequenceIntroducer),
}

pub struct Terminal<'a> {
    characters: HistoryBuffer<Character, 5000>,
    state: state::State<'a>,
    osc: Option<osc::OperatingSystemCommand>
}

impl <'a>Terminal<'a> {
    pub fn new() -> Self {
        Self {
            characters: HistoryBuffer::new(),
            state: state::State::default(),
            osc: None
        }
    }

    pub fn clear<D, C>(&self, display: &mut D) where C: PixelColor + From<Rgb888>, D: DrawTarget<Color = C>, <D as DrawTarget>::Error: Debug {
        let style: MonoTextStyle::<C> = (&self.state).into();
        display.clear(style.background_color.unwrap()).unwrap();
    }

    pub fn draw<D, C>(&mut self, display: &mut D) where C: PixelColor + From<Rgb888>, D: DrawTarget<Color = C>, <D as DrawTarget>::Error: Debug {
        self.state.reset_position();
        self.clear(display);

        for c in self.characters.oldest_ordered() {
            match c {
                Character::C1Escape(c1::C1Escape::StringTerminator) => {
                    self.osc = None;
                },
                Character::Char(core::ascii::Char::Bell) => {
                    self.osc = None;
                },
                Character::ControlSequenceIntroducer(
                    csi::ControlSequenceIntroducer::EraseInLine(_),
                ) => {
                    self.state.backspace();

                    // let style: MonoTextStyle::<C> = (&self.state).into();
                    // let next = Text::new(&s, self.state.position(), style)
                    //     .draw(display)
                    //     .unwrap();

                },
                Character::Char(core::ascii::Char::LineFeed | core::ascii::Char::CarriageReturn) => {
                    self.state.next_line();
                },
                Character::Char(c) => {
                    if self.osc.is_none() {
                        let s = std::format!("{}", c);

                        let style: MonoTextStyle::<C> = (&self.state).into();
                        let next = Text::new(&s, self.state.position(), style)
                            .draw(display)
                            .unwrap();

                        if next.x > 800 {
                            self.state.next_line();
                        } else {
                            self.state.update_position(next);
                        }
                    }
                }
                Character::ControlSequenceIntroducer(
                    csi::ControlSequenceIntroducer::EraseInDisplay(_)
                ) => {
                    self.clear(display);
                }
                Character::ControlSequenceIntroducer(
                    csi::ControlSequenceIntroducer::CursorPosition(_, _)
                ) => {
                    self.state.reset_position();
                }
                Character::ControlSequenceIntroducer(
                    csi::ControlSequenceIntroducer::SelectGraphicRendition(sgr),
                ) => {
                    self.state.update(*sgr);
                },
                Character::OperatingSystemCommand(
                    osc::OperatingSystemCommand::SetWorkingDirectory
                ) => {
                    self.osc = Some(osc::OperatingSystemCommand::SetWorkingDirectory)
                },
                _ => {}
            }

        }
    }

    pub fn push(&mut self, c: Character) {
        self.characters.write(c);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let bytes: [u8; 7] = [
            0x1b,
            0x5d,
            0x37,
            0x3b,
            0x41,
            0x07,
            0x42,
        ];
        let c1_parser = crate::c1::Parser::new(&bytes);
        let c1_buffer: heapless::Vec::<crate::Character, 10> = c1_parser.collect();
        
        assert_eq!(
            c1_buffer,
            [
                crate::Character::C1Escape(crate::c1::C1Escape::OperatingSystemCommand),
                crate::Character::Char(core::ascii::Char::Digit7),
                crate::Character::Char(core::ascii::Char::Semicolon),
                crate::Character::Char(core::ascii::Char::CapitalA),
                crate::Character::Char(core::ascii::Char::Bell),
                crate::Character::Char(core::ascii::Char::CapitalB),
            ]
        );

        let csi_parser = crate::csi::Parser::new(&c1_buffer);
        let csi_buffer: heapless::Vec::<crate::Character, 10> = csi_parser.collect();

        assert_eq!(
            csi_buffer,
            [
                crate::Character::C1Escape(crate::c1::C1Escape::OperatingSystemCommand),
                crate::Character::Char(core::ascii::Char::Digit7),
                crate::Character::Char(core::ascii::Char::Semicolon),
                crate::Character::Char(core::ascii::Char::CapitalA),
                crate::Character::Char(core::ascii::Char::Bell),
                crate::Character::Char(core::ascii::Char::CapitalB),
            ]
        );

        let osc_parser = crate::osc::Parser::new(&csi_buffer);
        let osc_buffer: heapless::Vec::<crate::Character, 10> = osc_parser.collect();
        assert_eq!(
            osc_buffer,
            [
                crate::Character::OperatingSystemCommand(crate::osc::OperatingSystemCommand::SetWorkingDirectory),
                crate::Character::C1Escape(crate::c1::C1Escape::StringTerminator),
                crate::Character::Char(core::ascii::Char::CapitalB),
            ]
        );

    }

    #[test]
    fn test_clear() {
        let bytes: [u8; 11] = [
            0x1b,
            0x5b,
            0x33,
            0x4a,
            0x1b,
            0x5b,
            0x48,
            0x1b,
            0x5b,
            0x32,
            0x4a,
        ];
        let c1_parser = crate::c1::Parser::new(&bytes);
        let c1_buffer: heapless::Vec::<crate::Character, 20> = c1_parser.collect();
        
        assert_eq!(
            c1_buffer,
            [
                crate::Character::C1Escape(crate::c1::C1Escape::ControlSequenceIntroducer),
                crate::Character::Char(core::ascii::Char::Digit3),
                crate::Character::Char(core::ascii::Char::CapitalJ),
                crate::Character::C1Escape(crate::c1::C1Escape::ControlSequenceIntroducer),
                crate::Character::Char(core::ascii::Char::CapitalH),
                crate::Character::C1Escape(crate::c1::C1Escape::ControlSequenceIntroducer),
                crate::Character::Char(core::ascii::Char::Digit2),
                crate::Character::Char(core::ascii::Char::CapitalJ),
            ]
        );

        let csi_parser = crate::csi::Parser::new(&c1_buffer);
        let csi_buffer: heapless::Vec::<crate::Character, 10> = csi_parser.collect();

        assert_eq!(
            csi_buffer,
            [
                crate::Character::ControlSequenceIntroducer(crate::csi::ControlSequenceIntroducer::EraseInDisplay(3)),
                crate::Character::ControlSequenceIntroducer(crate::csi::ControlSequenceIntroducer::CursorPosition(1, 1)),
                crate::Character::ControlSequenceIntroducer(crate::csi::ControlSequenceIntroducer::EraseInDisplay(2)),
            ]
        );

        let osc_parser = crate::osc::Parser::new(&csi_buffer);
        let osc_buffer: heapless::Vec::<crate::Character, 10> = osc_parser.collect();
        assert_eq!(
            osc_buffer,
            [
                crate::Character::ControlSequenceIntroducer(crate::csi::ControlSequenceIntroducer::EraseInDisplay(3)),
                crate::Character::ControlSequenceIntroducer(crate::csi::ControlSequenceIntroducer::CursorPosition(1, 1)),
                crate::Character::ControlSequenceIntroducer(crate::csi::ControlSequenceIntroducer::EraseInDisplay(2)),
            ]
        );

    }
}
