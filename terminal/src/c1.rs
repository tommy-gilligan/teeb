use core::{ascii::Char, slice::Iter};
use num_enum::TryFromPrimitive;
use crate::Character;

pub struct Parser<'a> {
    bytes: Iter<'a, u8>,
    escaping: bool,
}

impl<'a> Parser<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes: bytes.iter(),
            escaping: false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum C1Escape {
    SingleShiftTwo = 0x8E,
    SingleShiftThree = 0x8F,
    DeviceControlString = 0x90,
    ControlSequenceIntroducer = 0x9B,
    StringTerminator = 0x9C,
    OperatingSystemCommand = 0x9D,
    StartOfString = 0x98,
    PrivacyMessage = 0x9E,
    ApplicationProgramCommand = 0x9F,
}

impl Iterator for Parser<'_> {
    type Item = Character;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(byte) = self.bytes.next() {
                if self.escaping {
                    self.escaping = false;

                    if let Some(byte) = (*byte).checked_add(0x40) {
                        if let Ok(c) = C1Escape::try_from(byte) {
                            return Some(Character::C1Escape(c));
                        } else if let Some(c) = Char::from_u8(byte) {
                            return Some(Character::Char(c));
                        } else {
                            return Some(Character::Unrecognized(byte));
                        }
                    } else if (*byte) == 0x07 {
                            return Some(Character::C1Escape(C1Escape::StringTerminator));
                    } else {
                        return Some(Character::Unrecognized(*byte));
                    }
                } else if let Ok(c) = C1Escape::try_from(*byte) {
                    return Some(Character::C1Escape(c));
                } else if let Some(c) = Char::from_u8(*byte) {
                    if c == Char::Escape {
                        self.escaping = true;
                    } else {
                        return Some(Character::Char(c));
                    }
                } else {
                    return Some(Character::Unrecognized(*byte));
                }
            } else {
                return None;
            }
        }
    }
}
