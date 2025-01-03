use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::{Rgb888, PixelColor}, text::DecorationColor};

use super::State;
use crate::state::SelectGraphicRendition;

impl<'a, C> From<&'a State<'a>> for MonoTextStyle<'a, C> where C: PixelColor + From<Rgb888> {
    fn from(terminal_state: &'a State<'a>) -> Self {
        let mut foreground_color = match terminal_state.foreground {
            SelectGraphicRendition::SetForegroundColor1 => Rgb888::new(0, 0, 0),
            SelectGraphicRendition::SetForegroundColor2 => Rgb888::new(205, 0, 0),
            SelectGraphicRendition::SetForegroundColor3 => Rgb888::new(0, 205, 0),
            SelectGraphicRendition::SetForegroundColor4 => Rgb888::new(205, 205, 0),
            SelectGraphicRendition::SetForegroundColor5 => Rgb888::new(0, 0, 238),
            SelectGraphicRendition::SetForegroundColor6 => Rgb888::new(205, 0, 205),
            SelectGraphicRendition::SetForegroundColor7 => Rgb888::new(0, 205, 205),
            SelectGraphicRendition::SetForegroundColor8 => Rgb888::new(229, 229, 229),

            SelectGraphicRendition::SetBrightForegroundColor1 => Rgb888::new(127, 127, 127),
            SelectGraphicRendition::SetBrightForegroundColor2 => Rgb888::new(255, 0, 0),
            SelectGraphicRendition::SetBrightForegroundColor3 => Rgb888::new(0, 252, 0),
            SelectGraphicRendition::SetBrightForegroundColor4 => Rgb888::new(255, 255, 0),
            SelectGraphicRendition::SetBrightForegroundColor5 => Rgb888::new(0, 0, 252),
            SelectGraphicRendition::SetBrightForegroundColor6 => Rgb888::new(255, 0, 255),
            SelectGraphicRendition::SetBrightForegroundColor7 => Rgb888::new(0, 255, 255),
            SelectGraphicRendition::SetBrightForegroundColor8 => Rgb888::new(255, 255, 255),
            _ => Rgb888::new(0, 0, 0),
        };

        let mut background_color = match terminal_state.background {
            SelectGraphicRendition::SetBackgroundColor1 => Rgb888::new(0, 0, 0),
            SelectGraphicRendition::SetBackgroundColor2 => Rgb888::new(205, 0, 0),
            SelectGraphicRendition::SetBackgroundColor3 => Rgb888::new(0, 205, 0),
            SelectGraphicRendition::SetBackgroundColor4 => Rgb888::new(205, 205, 0),
            SelectGraphicRendition::SetBackgroundColor5 => Rgb888::new(0, 0, 238),
            SelectGraphicRendition::SetBackgroundColor6 => Rgb888::new(205, 0, 205),
            SelectGraphicRendition::SetBackgroundColor7 => Rgb888::new(0, 205, 205),
            SelectGraphicRendition::SetBackgroundColor8 => Rgb888::new(229, 229, 229),

            SelectGraphicRendition::SetBrightBackgroundColor1 => Rgb888::new(127, 127, 127),
            SelectGraphicRendition::SetBrightBackgroundColor2 => Rgb888::new(255, 0, 0),
            SelectGraphicRendition::SetBrightBackgroundColor3 => Rgb888::new(0, 252, 0),
            SelectGraphicRendition::SetBrightBackgroundColor4 => Rgb888::new(255, 255, 0),
            SelectGraphicRendition::SetBrightBackgroundColor5 => Rgb888::new(0, 0, 252),
            SelectGraphicRendition::SetBrightBackgroundColor6 => Rgb888::new(255, 0, 255),
            SelectGraphicRendition::SetBrightBackgroundColor7 => Rgb888::new(0, 255, 255),
            SelectGraphicRendition::SetBrightBackgroundColor8 => Rgb888::new(255, 255, 255),
            _ => Rgb888::new(255, 255, 255),
        };

        if terminal_state.reverse {
            core::mem::swap(&mut background_color, &mut foreground_color);
        }
        if terminal_state.conceal {
            foreground_color = background_color;
        }

        let mut text_style = MonoTextStyle::new(
            if terminal_state.bold {
                terminal_state.bold_font
            } else {
                terminal_state.normal_font
            },
            foreground_color.into(),
        );

        text_style.background_color = Some(background_color.into());
        text_style.underline_color = if terminal_state.underline {
            DecorationColor::TextColor
        } else {
            DecorationColor::None
        };
        text_style.strikethrough_color = if terminal_state.crossed_out {
            DecorationColor::TextColor
        } else {
            DecorationColor::None
        };

        text_style
    }
}
