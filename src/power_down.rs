//! Power down configuration
#![allow(clippy::new_without_default)]

use crate::Command;
use core::marker::PhantomData;

///Marker Power down concern
pub struct PowerDown;

impl_command_new!(PowerDown, 0b110, 0b1001_1111);

impl Command<PowerDown> {
    pub fn lineinpd(self) -> Lineinpd {
        Lineinpd { cmd: self }
    }
    pub fn micpd(self) -> Micpd {
        Micpd { cmd: self }
    }
    pub fn adcpd(self) -> Adcpd {
        Adcpd { cmd: self }
    }
    pub fn dacpd(self) -> Dacpd {
        Dacpd { cmd: self }
    }
    pub fn outpd(self) -> Outpd {
        Outpd { cmd: self }
    }
    pub fn oscpd(self) -> Oscpd {
        Oscpd { cmd: self }
    }
    pub fn clkoutpd(self) -> Clkoutpd {
        Clkoutpd { cmd: self }
    }
    pub fn poweroff(self) -> Poweroff {
        Poweroff { cmd: self }
    }
}

impl_toggle_writer!(Lineinpd, Command<PowerDown>, 0);
impl_toggle_writer!(Micpd, Command<PowerDown>, 1);
impl_toggle_writer!(Adcpd, Command<PowerDown>, 2);
impl_toggle_writer!(Dacpd, Command<PowerDown>, 3);
impl_toggle_writer!(Outpd, Command<PowerDown>, 4);
impl_toggle_writer!(Oscpd, Command<PowerDown>, 5);
impl_toggle_writer!(Clkoutpd, Command<PowerDown>, 6);
impl_toggle_writer!(Poweroff, Command<PowerDown>, 7);
