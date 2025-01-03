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

pub mod c1;
pub mod csi;
pub mod state;
pub mod wrap;

pub struct Terminal<'a> {
    characters: HistoryBuffer<csi::Character, 5000>,
    state: state::State<'a>,
}

impl <'a>Terminal<'a> {
    pub fn new() -> Self {
        Self {
            characters: HistoryBuffer::new(),
            state: state::State::default(),
        }
    }

    pub fn clear<D, C>(&mut self, display: &mut D) where C: PixelColor + From<Rgb888>, D: DrawTarget<Color = C>, <D as DrawTarget>::Error: Debug {
        let style: MonoTextStyle::<C> = (&self.state).into();
        display.clear(style.background_color.unwrap()).unwrap();
    }

    pub fn draw<D, C>(&mut self, display: &mut D) where C: PixelColor + From<Rgb888>, D: DrawTarget<Color = C>, <D as DrawTarget>::Error: Debug {
        self.state.reset_position();
        self.clear(display);

        for c in self.characters.oldest_ordered() {
            match c {
                csi::Character::Char(core::ascii::Char::CarriageReturn) => {
                    self.state.next_line();
                },
                csi::Character::Char(c) => {
                    let s = std::format!("{}", c);

                    let style: MonoTextStyle::<C> = (&self.state).into();
                    let next = Text::new(&s, self.state.position(), style)
                        .draw(display)
                        .unwrap();

                    if next.x > 700 {
                        self.state.next_line();
                    } else {
                        self.state.update_position(next);
                    }
                }
                csi::Character::ControlSequenceIntroducer(
                    csi::ControlSequenceIntroducer::SelectGraphicRendition(sgr),
                ) => {
                    self.state.update(*sgr);
                }
                _ => {}
            }
        }
    }

    pub fn push(&mut self, c: csi::Character) {
        self.characters.write(c);
    }
}
