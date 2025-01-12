use crate::c1::C1Escape;
use core::{ascii::Char, slice::Iter};
use crate::Character;

pub struct Parser<'a> {
    c1: Iter<'a, crate::Character>,
    osc: bool,
    argument: Option<usize>,
    needs_terminating: bool
}

impl<'a> Parser<'a> {
    pub fn new(c1: &'a [crate::Character]) -> Self {
        Self {
            c1: c1.iter(),
            osc: false,
            needs_terminating: false,
            argument: None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OperatingSystemCommand {
    SetWorkingDirectory
}

impl Iterator for Parser<'_> {
    type Item = Character;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(c1) = self.c1.next() {
                if self.osc {
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
                                Char::Semicolon => {
                                    self.osc = false;

                                    if self.argument == Some(7) {
                                        self.needs_terminating = true; 
                                        self.argument = None;
                                        return Some(Character::OperatingSystemCommand(
                                            OperatingSystemCommand::SetWorkingDirectory
                                        ));
                                    } else {
                                        self.argument = None;
                                    }
                                },
                                _ => {
                                    self.osc = false;
                                    self.argument = None;
                                }
                            }
                        }
                        g => {
                            self.osc = false;
                            return Some(g)
                        }
                    }
                } else if self.needs_terminating {
                    match *c1 {
                        crate::Character::Char(Char::Bell) => {
                            self.needs_terminating = false;
                            return Some(crate::Character::C1Escape(crate::c1::C1Escape::StringTerminator));
                        },
                        crate::Character::C1Escape(crate::c1::C1Escape::StringTerminator) => {
                            self.needs_terminating = false;
                            return Some(crate::Character::C1Escape(crate::c1::C1Escape::StringTerminator));
                        },
                        // throw away for now
                        _ => {
                        }
                    }
                } else {
                    match *c1 {
                        crate::Character::C1Escape(c1) => {
                            if c1 == C1Escape::OperatingSystemCommand {
                                self.osc = true;
                            } else {
                                return Some(Character::C1Escape(c1));
                            }
                        },
                        g => {
                            return Some(g);
                        }
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
    use crate::osc::Parser;
    use crate::Character;
    use crate::c1::C1Escape;

    #[test]
    fn test_set_working_directory() {
        let c1 = [
            Character::C1Escape(
                C1Escape::OperatingSystemCommand
            ),
            Character::Char(
                core::ascii::Char::Digit7
            ),
            Character::Char(
                core::ascii::Char::Semicolon
            ),
            Character::Char(
                core::ascii::Char::CapitalA
            ),
            Character::Char(
                core::ascii::Char::CapitalB
            ),
            Character::Char(
                core::ascii::Char::CapitalC
            ),
            Character::C1Escape(
                C1Escape::StringTerminator
            ),
        ];

        let mut parser = Parser::new(&c1);
        assert_eq!(parser.next().unwrap(), crate::Character::OperatingSystemCommand(
            super::OperatingSystemCommand::SetWorkingDirectory
        ));
        assert_eq!(parser.next().unwrap(), crate::Character::C1Escape(
            C1Escape::StringTerminator
        ));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_set_working_directory_bell() {
        let c1 = [
            Character::C1Escape(
                C1Escape::OperatingSystemCommand
            ),
            Character::Char(
                core::ascii::Char::Digit7
            ),
            Character::Char(
                core::ascii::Char::Semicolon
            ),
            Character::Char(
                core::ascii::Char::CapitalA
            ),
            Character::Char(
                core::ascii::Char::CapitalB
            ),
            Character::Char(
                core::ascii::Char::Bell
            ),
        ];

        let mut parser = Parser::new(&c1);
        assert_eq!(parser.next().unwrap(), crate::Character::OperatingSystemCommand(
            super::OperatingSystemCommand::SetWorkingDirectory
        ));
        assert_eq!(parser.next().unwrap(), crate::Character::C1Escape(
            C1Escape::StringTerminator
        ));
        assert_eq!(parser.next(), None);
    }
}
