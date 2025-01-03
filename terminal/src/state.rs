use embedded_graphics::{
    geometry::Point,
    mono_font::{ascii::FONT_9X18, ascii::FONT_9X18_BOLD, MonoFont},
};

use crate::csi::select_graphic_rendition::SelectGraphicRendition;

pub mod graphics;

pub struct State<'a> {
    italic: bool,
    bold: bool,
    underline: bool,
    overline: bool,
    foreground: SelectGraphicRendition,
    background: SelectGraphicRendition,
    faint: bool,
    reverse: bool,
    crossed_out: bool,
    conceal: bool,
    normal_font: &'a MonoFont<'a>,
    bold_font: &'a MonoFont<'a>,
    position: Point,
}

impl State<'_> {
    pub fn update(&mut self, sgr: SelectGraphicRendition) {
        match sgr {
            SelectGraphicRendition::Reset => *self = Self::default(),
            SelectGraphicRendition::Bold => {
                self.bold = true;
            }
            SelectGraphicRendition::Faint => {
                self.faint = true;
            }
            SelectGraphicRendition::Italic => {
                self.italic = true;
            }
            SelectGraphicRendition::Underline => {
                self.underline = true;
            }
            SelectGraphicRendition::ReverseVideo => {
                self.reverse = true;
            }
            SelectGraphicRendition::Conceal => {
                self.conceal = true;
            }
            SelectGraphicRendition::CrossedOut => {
                self.crossed_out = true;
            }
            SelectGraphicRendition::NormalIntensity => {
                self.bold = false;
                self.faint = false;
            }
            SelectGraphicRendition::NotItalic => {
                self.italic = false;
            }
            SelectGraphicRendition::NotUnderline => {
                self.underline = false;
            }
            SelectGraphicRendition::NotReversed => {
                self.reverse = false;
            }
            SelectGraphicRendition::Reveal => {
                self.conceal = false;
            }
            SelectGraphicRendition::NotCrossedOut => {
                self.crossed_out = false;
            }
            SelectGraphicRendition::SetForegroundColor1 => {
                self.foreground = SelectGraphicRendition::SetForegroundColor1;
            }
            SelectGraphicRendition::SetForegroundColor2 => {
                self.foreground = SelectGraphicRendition::SetForegroundColor2;
            }
            SelectGraphicRendition::SetForegroundColor3 => {
                self.foreground = SelectGraphicRendition::SetForegroundColor3;
            }
            SelectGraphicRendition::SetForegroundColor4 => {
                self.foreground = SelectGraphicRendition::SetForegroundColor4;
            }
            SelectGraphicRendition::SetForegroundColor5 => {
                self.foreground = SelectGraphicRendition::SetForegroundColor5;
            }
            SelectGraphicRendition::SetForegroundColor6 => {
                self.foreground = SelectGraphicRendition::SetForegroundColor6;
            }
            SelectGraphicRendition::SetForegroundColor7 => {
                self.foreground = SelectGraphicRendition::SetForegroundColor7;
            }
            SelectGraphicRendition::SetForegroundColor8 => {
                self.foreground = SelectGraphicRendition::SetForegroundColor8;
            }
            SelectGraphicRendition::SetBackgroundColor1 => {
                self.background = SelectGraphicRendition::SetBackgroundColor1;
            }
            SelectGraphicRendition::SetBackgroundColor2 => {
                self.background = SelectGraphicRendition::SetBackgroundColor2;
            }
            SelectGraphicRendition::SetBackgroundColor3 => {
                self.background = SelectGraphicRendition::SetBackgroundColor3;
            }
            SelectGraphicRendition::SetBackgroundColor4 => {
                self.background = SelectGraphicRendition::SetBackgroundColor4;
            }
            SelectGraphicRendition::SetBackgroundColor5 => {
                self.background = SelectGraphicRendition::SetBackgroundColor5;
            }
            SelectGraphicRendition::SetBackgroundColor6 => {
                self.background = SelectGraphicRendition::SetBackgroundColor6;
            }
            SelectGraphicRendition::SetBackgroundColor7 => {
                self.background = SelectGraphicRendition::SetBackgroundColor7;
            }
            SelectGraphicRendition::SetBackgroundColor8 => {
                self.background = SelectGraphicRendition::SetBackgroundColor8;
            }
            SelectGraphicRendition::SetBrightForegroundColor1 => {
                self.foreground = SelectGraphicRendition::SetBrightForegroundColor1;
            }
            SelectGraphicRendition::SetBrightForegroundColor2 => {
                self.foreground = SelectGraphicRendition::SetBrightForegroundColor2;
            }
            SelectGraphicRendition::SetBrightForegroundColor3 => {
                self.foreground = SelectGraphicRendition::SetBrightForegroundColor3;
            }
            SelectGraphicRendition::SetBrightForegroundColor4 => {
                self.foreground = SelectGraphicRendition::SetBrightForegroundColor4;
            }
            SelectGraphicRendition::SetBrightForegroundColor5 => {
                self.foreground = SelectGraphicRendition::SetBrightForegroundColor5;
            }
            SelectGraphicRendition::SetBrightForegroundColor6 => {
                self.foreground = SelectGraphicRendition::SetBrightForegroundColor6;
            }
            SelectGraphicRendition::SetBrightForegroundColor7 => {
                self.foreground = SelectGraphicRendition::SetBrightForegroundColor7;
            }
            SelectGraphicRendition::SetBrightForegroundColor8 => {
                self.foreground = SelectGraphicRendition::SetBrightForegroundColor8;
            }
            SelectGraphicRendition::SetBrightBackgroundColor1 => {
                self.background = SelectGraphicRendition::SetBrightBackgroundColor1;
            }
            SelectGraphicRendition::SetBrightBackgroundColor2 => {
                self.background = SelectGraphicRendition::SetBrightBackgroundColor2;
            }
            SelectGraphicRendition::SetBrightBackgroundColor3 => {
                self.background = SelectGraphicRendition::SetBrightBackgroundColor3;
            }
            SelectGraphicRendition::SetBrightBackgroundColor4 => {
                self.background = SelectGraphicRendition::SetBrightBackgroundColor4;
            }
            SelectGraphicRendition::SetBrightBackgroundColor5 => {
                self.background = SelectGraphicRendition::SetBrightBackgroundColor5;
            }
            SelectGraphicRendition::SetBrightBackgroundColor6 => {
                self.background = SelectGraphicRendition::SetBrightBackgroundColor6;
            }
            SelectGraphicRendition::SetBrightBackgroundColor7 => {
                self.background = SelectGraphicRendition::SetBrightBackgroundColor7;
            }
            SelectGraphicRendition::SetBrightBackgroundColor8 => {
                self.background = SelectGraphicRendition::SetBrightBackgroundColor8;
            }

            SelectGraphicRendition::DefaultForegroundColor => {
                self.foreground = SelectGraphicRendition::SetBrightForegroundColor8;
            }
            SelectGraphicRendition::DefaultBackgroundColor => {
                self.background = SelectGraphicRendition::SetBackgroundColor1;
            }
            //     SetForegroundColor = 38,
            //     SetBackgroundColor = 48,
            //     NeitherFramedNorEncircled = 54,
            //     NotOverlined = 55,
            //     SetUnderlineColor = 58,
            //     DefaultUnderlineColor = 59,
            _ => {}
        }
    }

    pub fn update_position(&mut self, position: Point) {
        self.position = position;
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn next_line(&mut self) {
        self.position = Point::new(0, self.position.y + 15);
    }

    pub fn reset_position(&mut self) {
        self.position = Point::new(0, 14);
    }
}

impl Default for State<'_> {
    fn default() -> Self {
        Self {
            italic: false,
            reverse: false,
            conceal: false,
            foreground: SelectGraphicRendition::SetBrightForegroundColor8,
            background: SelectGraphicRendition::SetBackgroundColor1,
            overline: false,
            faint: false,
            bold: false,
            normal_font: &FONT_9X18,
            bold_font: &FONT_9X18_BOLD,
            underline: false,
            crossed_out: false,
            // default position should be based on font?
            position: Point::new(0, 14),
        }
    }
}
