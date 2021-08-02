//! Sampling configuration
//!
//! TODO: not safe at the moment 
#![allow(clippy::new_without_default)]

use crate::Command;
use core::marker::PhantomData;

///Marker indicating Sampling concern
pub struct Sampling;

impl_command_new!(Sampling, 0b1000, 0b0000_0000);

impl Command<Sampling> {
    pub fn usb_normal(self) -> UsbNormal {
        UsbNormal { cmd: self }
    }
    pub fn bosr(self) -> Bosr {
        Bosr { cmd: self }
    }
    pub fn sr(self) -> Sr {
        Sr { cmd: self }
    }
    pub fn clkidiv2(self) -> Clkidiv2 {
        Clkidiv2 { cmd: self }
    }
    pub fn clkodiv2(self) -> Clkodiv2 {
        Clkodiv2 { cmd: self }
    }
}

pub struct UsbNormal {
    cmd: Command<Sampling>,
}

impl UsbNormal {
    impl_clear_bit!(Command<Sampling>, 0);
    impl_set_bit!(Command<Sampling>, 0);
    impl_clear_bit!(normal, Command<Sampling>, 0);
    impl_set_bit!(usb, Command<Sampling>, 0);
}
pub struct Bosr {
    cmd: Command<Sampling>,
}

impl Bosr {
    impl_clear_bit!(Command<Sampling>, 0);
    impl_set_bit!(Command<Sampling>, 0);
}

pub struct Sr {
    cmd: Command<Sampling>,
}

impl Sr {
    impl_bits!(unsafe, Command<Sampling>, 4, 2);

    #[must_use]
    pub fn sr_0b0000(self) -> Command<Sampling> {
        unsafe { self.bits(0b0000) }
    }
    #[must_use]
    pub fn sr_0b0001(self) -> Command<Sampling> {
        unsafe { self.bits(0b0001) }
    }
    #[must_use]
    pub fn sr_0b0010(self) -> Command<Sampling> {
        unsafe { self.bits(0b0010) }
    }
    #[must_use]
    pub fn sr_0b0011(self) -> Command<Sampling> {
        unsafe { self.bits(0b0011) }
    }
    #[must_use]
    pub fn sr_0b0110(self) -> Command<Sampling> {
        unsafe { self.bits(0b0110) }
    }
    #[must_use]
    pub fn sr_0b0111(self) -> Command<Sampling> {
        unsafe { self.bits(0b0111) }
    }
    #[must_use]
    pub fn sr_0b1000(self) -> Command<Sampling> {
        unsafe { self.bits(0b1000) }
    }
    #[must_use]
    pub fn sr_0b1001(self) -> Command<Sampling> {
        unsafe { self.bits(0b1001) }
    }
    #[must_use]
    pub fn sr_0b1010(self) -> Command<Sampling> {
        unsafe { self.bits(0b1010) }
    }
    #[must_use]
    pub fn sr_0b1011(self) -> Command<Sampling> {
        unsafe { self.bits(0b1011) }
    }
    #[must_use]
    pub fn sr_0b1111(self) -> Command<Sampling> {
        unsafe { self.bits(0b1111) }
    }
}

impl_toggle_writer!(Clkidiv2,Command<Sampling>,6);
impl_toggle_writer!(Clkodiv2,Command<Sampling>,7);

