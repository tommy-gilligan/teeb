use crate::c1::C1Escape;
use core::{ascii::Char, slice::Iter};
use crate::Character;

pub mod select_graphic_rendition;

pub struct Parser<'a> {
    c1: Iter<'a, crate::Character>,
    csi: bool,
    argument: Option<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(c1: &'a [crate::Character]) -> Self {
        Self {
            c1: c1.iter(),
            csi: false,
            argument: None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ControlSequenceIntroducer {
    CursorUp(usize),
    CursorDown(usize),
    CursorForward(usize),
    CursorBack(usize),
    CursorNextLine(usize),
    CursorPreviousLine(usize),
    CursorHorizontalAbsolute(usize),
    CursorPosition(usize, usize),
    EraseInDisplay(usize),
    EraseInLine(usize),
    ScrollUp(usize),
    ScrollDown(usize),
    HorizontalVerticalPosition(usize, usize),
    SelectGraphicRendition(select_graphic_rendition::SelectGraphicRendition),
    AUXPortOn,
    AUXPortOff,
    DeviceStatusReport,
    SaveCursorPosition,
    RestoreCursorPosition,
    ShowCursor,
    HideCursor,
    EnableFocusReporting,
    DisableFocusReporting,
    EnableAlternativeScreen,
    DisableAlternativeScreen,
    EnableBracketedPaste,
    DisableBracketedPaste,
}

impl Iterator for Parser<'_> {
    type Item = Character;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(c1) = self.c1.next() {
                if self.csi {
                    match *c1 {
                        crate::Character::Char(c) => {
                            match c {
                                d @ (Char::Digit0
                                | Char::Digit1
                                | Char::Digit2
                                | Char::Digit3
                                | Char::Digit4
                                | Char::Digit5
                                | Char::Digit6
                                | Char::Digit7
                                | Char::Digit8
                                | Char::Digit9) => {
                                    if self.argument.is_none() {
                                        self.argument = Some(0);
                                    }
                                    let argument = self.argument.unwrap();
                                    self.argument = Some(
                                        argument * 10
                                            + d.to_char().to_digit(10).unwrap() as usize,
                                    );
                                },
                                Char::CapitalA => {
                                    self.csi = false;
                                    self.argument = None;

                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::CursorUp(
                                            self.argument.unwrap_or(1),
                                        ),
                                    ));
                                }
                                Char::CapitalB => {
                                    self.csi = false;
                                    self.argument = None;

                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::CursorDown(
                                            self.argument.unwrap_or(1),
                                        ),
                                    ));
                                }
                                Char::CapitalC => {
                                    self.csi = false;
                                    self.argument = None;

                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::CursorForward(
                                            self.argument.unwrap_or(1),
                                        ),
                                    ));
                                }
                                Char::CapitalD => {
                                    self.csi = false;
                                    self.argument = None;

                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::CursorBack(
                                            self.argument.unwrap_or(1),
                                        ),
                                    ));
                                }
                                Char::CapitalE => {
                                    self.csi = false;
                                    self.argument = None;

                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::CursorNextLine(
                                            self.argument.unwrap_or(1),
                                        ),
                                    ));
                                }
                                Char::CapitalF => {
                                    self.csi = false;
                                    self.argument = None;

                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::CursorPreviousLine(
                                            self.argument.unwrap_or(1),
                                        ),
                                    ));
                                }
                                Char::CapitalG => {
                                    self.csi = false;
                                    self.argument = None;

                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::CursorHorizontalAbsolute(
                                            self.argument.unwrap_or(1),
                                        ),
                                    ));
                                }
                                Char::CapitalH => {
                                    self.csi = false;
                                    let argument = self.argument;
                                    self.argument = None;

                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::CursorPosition(
                                            argument.unwrap_or(1),
                                            argument.unwrap_or(1),
                                        ),
                                    ));
                                }
                                Char::CapitalJ => {
                                    self.csi = false;
                                    let argument = self.argument;
                                    self.argument = None;
                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::EraseInDisplay(
                                            argument.unwrap_or(0),
                                        ),
                                    ));
                                }
                                Char::CapitalK => {
                                    self.csi = false;
                                    let argument = self.argument.unwrap_or(0);
                                    self.argument = None;

                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::EraseInLine(
                                            argument,
                                        ),
                                    ));
                                }
                                Char::CapitalS => {
                                    self.csi = false;
                                    self.argument = None;

                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::ScrollUp(
                                            self.argument.unwrap_or(1),
                                        ),
                                    ));
                                }
                                Char::CapitalT => {
                                    self.csi = false;
                                    self.argument = None;

                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::ScrollDown(
                                            self.argument.unwrap_or(1),
                                        ),
                                    ));
                                }
                                Char::SmallM => {
                                    let argument = self.argument;
                                    self.csi = false;
                                    self.argument = None;

                                    if let Some(argument) = argument {
                                        if let Ok(sgr) = select_graphic_rendition::SelectGraphicRendition::try_from(argument as u8) {
                                            return Some(
                                                Character::ControlSequenceIntroducer(
                                                    ControlSequenceIntroducer::SelectGraphicRendition(
                                                        sgr
                                                    )
                                                )
                                            );
                                        }
                                    }
                                }
                                Char::SmallI => {
                                    let argument = self.argument;
                                    self.csi = false;
                                    self.argument = None;
                                    match argument {
                                        Some(4) => {
                                            return Some(Character::ControlSequenceIntroducer(
                                                ControlSequenceIntroducer::AUXPortOff,
                                            ));
                                        }
                                        Some(5) => {
                                            return Some(Character::ControlSequenceIntroducer(
                                                ControlSequenceIntroducer::AUXPortOn,
                                            ));
                                        }
                                        _ => {}
                                    }
                                }
                                Char::SmallN => {
                                    let argument = self.argument;
                                    self.csi = false;
                                    self.argument = None;
                                    if Some(6) == argument {
                                        return Some(Character::ControlSequenceIntroducer(
                                            ControlSequenceIntroducer::DeviceStatusReport,
                                        ));
                                    }
                                }
                                Char::SmallS => {
                                    self.csi = false;
                                    self.argument = None;
                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::SaveCursorPosition,
                                    ));
                                }
                                Char::SmallU => {
                                    self.csi = false;
                                    self.argument = None;
                                    return Some(Character::ControlSequenceIntroducer(
                                        ControlSequenceIntroducer::RestoreCursorPosition,
                                    ));
                                }
                                c => {
                                    self.csi = false;
                                    self.argument = None;

                                    return Some(Character::Char(c));
                                }
                            }
                        }
                        crate::Character::C1Escape(c1) => {
                            self.csi = false;
                            return Some(Character::C1Escape(c1));
                        }
                        crate::Character::Unrecognized(byte) => {
                            self.csi = false;
                            return Some(Character::Unrecognized(byte));
                        },
                        crate::Character::OperatingSystemCommand(_) => todo!(),
                        crate::Character::ControlSequenceIntroducer(_) => todo!()
                    }
                } else {
                    match *c1 {
                        crate::Character::Char(c) => {
                            return Some(Character::Char(c));
                        }
                        crate::Character::C1Escape(c1) => {
                            if c1 == C1Escape::ControlSequenceIntroducer {
                                self.csi = true;
                            } else {
                                return Some(Character::C1Escape(c1));
                            }
                        }
                        crate::Character::Unrecognized(byte) => {
                            return Some(Character::Unrecognized(byte));
                        },
                        crate::Character::OperatingSystemCommand(_) => todo!(),
                        crate::Character::ControlSequenceIntroducer(_) => todo!()
                    }
                }
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::csi::Parser;
    use crate::Character;
    use crate::c1::C1Escape;

    #[test]
    fn test_erase_in_line() {
        let c1 = [
            Character::C1Escape(
                C1Escape::ControlSequenceIntroducer
            ),
            Character::Char(
                core::ascii::Char::Digit1
            ),
            Character::Char(
                core::ascii::Char::CapitalK
            ),
            Character::Char(
                core::ascii::Char::CapitalK
            ),
            Character::C1Escape(
                C1Escape::ControlSequenceIntroducer
            ),
            Character::Char(
                core::ascii::Char::CapitalK
            ),
            Character::C1Escape(
                C1Escape::ControlSequenceIntroducer
            ),
            Character::Char(
                core::ascii::Char::Digit2
            ),
            Character::Char(
                core::ascii::Char::CapitalK
            ),
            Character::Char(
                core::ascii::Char::CapitalA
            ),
        ];

        let mut parser = Parser::new(&c1);
        assert_eq!(parser.next().unwrap(), crate::csi::Character::ControlSequenceIntroducer(
            super::ControlSequenceIntroducer::EraseInLine(1)
        ));
        assert_eq!(parser.next().unwrap(), crate::csi::Character::Char(
            core::ascii::Char::CapitalK
        ));
        assert_eq!(parser.next().unwrap(), crate::csi::Character::ControlSequenceIntroducer(
            super::ControlSequenceIntroducer::EraseInLine(0)
        ));
        assert_eq!(parser.next().unwrap(), crate::csi::Character::ControlSequenceIntroducer(
            super::ControlSequenceIntroducer::EraseInLine(2)
        ));
        assert_eq!(parser.next().unwrap(), crate::csi::Character::Char(
            core::ascii::Char::CapitalA
        ));
        assert_eq!(parser.next(), None);
    }
}
