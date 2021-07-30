//! Line inputs configuration
#![allow(clippy::new_without_default)]

use crate::Command;
use core::marker::PhantomData;

///Marker indicating line in concern
pub struct LineIn<CHANNEL> {
    channel: PhantomData<CHANNEL>,
}

///Marker indicating left channel concern
pub struct Left;

///Marker indicating right channel concern
pub struct Right;

///Marker indicating left line in concern
pub type LeftLineIn = LineIn<Left>;

///Marker indicating left line in concern
pub type RightLineIn = LineIn<Right>;

impl Command<LeftLineIn> {
    pub fn new() -> Self {
        Self {
            data: 0b0_1001_0111,
            t: PhantomData::<LeftLineIn>,
        }
    }
}

impl Command<RightLineIn> {
    pub fn new() -> Self {
        Self {
            data: 0x1 << 9 | 0b0_1001_0111,
            t: PhantomData::<RightLineIn>,
        }
    }
}

impl<CHANNEL> Command<LineIn<CHANNEL>> {
    pub fn invol(self) -> Invol<CHANNEL> {
        Invol { cmd: self }
    }
}

///Writer of LINVOL or RINVOL fields. Control line input volume.
pub struct Invol<CHANNEL> {
    cmd: Command<LineIn<CHANNEL>>,
}

impl<CHANNEL> Invol<CHANNEL> {
    #[must_use]
    pub fn bits(mut self, value: u8) -> Command<LineIn<CHANNEL>> {
        let mask = 0b1_1111;
        self.cmd.data = self.cmd.data & !mask | (value as u16) & mask;
        self.cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn left_line_in_new() {
        let cmd = Command::<LeftLineIn>::new();
        let expected = 0b0000_0000_1001_0111;
        assert!(
            cmd.data == expected,
            "Got {:#b},expected {:#b}",
            cmd.data,
            expected
        )
    }
    #[test]
    fn right_line_in_new() {
        let cmd = Command::<RightLineIn>::new();
        let expected = 0b0000_0010_1001_0111;
        assert!(
            cmd.data == expected,
            "Got {:#b},expected {:#b}",
            cmd.data,
            expected
        )
    }
    #[test]
    fn set_bits_dont_overwrite() {
        let cmd = Command::<RightLineIn>::new();
        //this should trigger a warning
        //cmd.invol().bits(0b1111_1111);
        let cmd = cmd.invol().bits(0b1111_1111);
        let expected = 0b0000_0010_1001_1111;
        assert!(
            cmd.data == expected,
            "Got {:#b},expected {:#b}",
            cmd.data,
            expected
        )
    }
}
