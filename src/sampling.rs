//! Sampling configuration
//!
//! Not safe at the moment
//! TODO: 
//!  - at the moment, usb mode and bosr = 1 and not writing sr produce an invalid configuration,
//! Enforcing sr write seems a good idea.
//!  - have more meaningfull additional api would be welcome.
//! 
#![allow(clippy::new_without_default)]

use crate::Command;
use core::marker::PhantomData;

///Marker indicating Sampling concern
pub struct Sampling<T> {
    t: PhantomData<T>,
}

/// Marker trait to say a marker correspnd to something set
pub trait IsSet {}

/// Marker used to indicate Normal mode;
pub struct Normal;
impl IsSet for Normal {}
/// Marker used to indicate USB mode;
pub struct Usb;
impl IsSet for Usb {}
/// Marker used to indicate BOSR bit is set;
pub struct BosrIsSet;
impl IsSet for BosrIsSet {}
/// Marker used to indicate BOSR bit is clear;
pub struct BosrIsClear;
impl IsSet for BosrIsClear {}

/// Marker used to indicate something is not yet defined but required to be.
pub struct Unset;

impl_command_new!(Sampling<(Unset, Unset)>, 0b1000, 0b0000_0000);

//it's always safe to manipulate those fields
impl<T> Command<Sampling<T>> {
    pub fn clkidiv2(self) -> Clkidiv2<T> {
        Clkidiv2 { cmd: self }
    }
    pub fn clkodiv2(self) -> Clkodiv2<T> {
        Clkodiv2 { cmd: self }
    }
}

//To make the usb mode only settable once
impl<T> Command<Sampling<(Unset, T)>> {
    pub fn usb_normal(self) -> UsbNormal<(Unset, T)> {
        UsbNormal { cmd: self }
    }
}

impl<T> Command<Sampling<(T, Unset)>> {
    pub fn bosr(self) -> Bosr<(T, Unset)> {
        Bosr { cmd: self }
    }
}

impl<BOSR> Command<Sampling<(Usb, BOSR)>>
where
    BOSR: IsSet,
{
    pub fn sr(self) -> Sr<(Usb, BOSR)> {
        Sr { cmd: self }
    }
}

impl<BOSR> Command<Sampling<(Normal, BOSR)>> {
    pub fn sr(self) -> Sr<(Normal, BOSR)> {
        Sr { cmd: self }
    }
}

pub struct UsbNormal<T> {
    cmd: Command<Sampling<T>>,
}

impl<MODE, BOSR> UsbNormal<(MODE, BOSR)> {
    #[must_use]
    pub fn clear_bit(mut self) -> Command<Sampling<(Normal, BOSR)>> {
        self.cmd.data &= !(0b1 << 0);
        Command::<Sampling<(Normal, BOSR)>> {
            data: self.cmd.data,
            t: PhantomData::<Sampling<(Normal, BOSR)>>,
        }
    }
    #[must_use]
    pub fn set_bit(mut self) -> Command<Sampling<(Usb, BOSR)>> {
        self.cmd.data |= 0b1 << 0;
        Command::<Sampling<(Usb, BOSR)>> {
            data: self.cmd.data,
            t: PhantomData::<Sampling<(Usb, BOSR)>>,
        }
    }
    #[must_use]
    pub fn normal(mut self) -> Command<Sampling<(Normal, BOSR)>> {
        self.cmd.data &= !(0b1 << 0);
        Command::<Sampling<(Normal, BOSR)>> {
            data: self.cmd.data,
            t: PhantomData::<Sampling<(Normal, BOSR)>>,
        }
    }
    #[must_use]
    pub fn usb(mut self) -> Command<Sampling<(Usb, BOSR)>> {
        self.cmd.data |= 0b1 << 0;
        Command::<Sampling<(Usb, BOSR)>> {
            data: self.cmd.data,
            t: PhantomData::<Sampling<(Usb, BOSR)>>,
        }
    }
}

pub struct Bosr<T> {
    cmd: Command<Sampling<T>>,
}

impl<MODE, BOSR> Bosr<(MODE, BOSR)> {
    #[must_use]
    pub fn clear_bit(mut self) -> Command<Sampling<(MODE, BosrIsClear)>> {
        self.cmd.data &= !(0b1 << 1);
        Command::<Sampling<(MODE, BosrIsClear)>> {
            data: self.cmd.data,
            t: PhantomData::<Sampling<(MODE, BosrIsClear)>>,
        }
    }
    #[must_use]
    pub fn set_bit(mut self) -> Command<Sampling<(MODE, BosrIsSet)>> {
        self.cmd.data |= 0b1 << 1;
        Command::<Sampling<(MODE, BosrIsSet)>> {
            data: self.cmd.data,
            t: PhantomData::<Sampling<(MODE, BosrIsSet)>>,
        }
    }
}

pub struct Sr<T> {
    cmd: Command<Sampling<T>>,
}

impl<T> Sr<T> {
    impl_bits!(unsafe, Command<Sampling<T>>, 4, 2);
}

impl<BOSR> Sr<(Normal, BOSR)> {
    #[must_use]
    pub fn sr_0b0000(self) -> Command<Sampling<(Normal, BOSR)>> {
        unsafe { self.bits(0b0000) }
    }
    #[must_use]
    pub fn sr_0b0001(self) -> Command<Sampling<(Normal, BOSR)>> {
        unsafe { self.bits(0b0001) }
    }
    #[must_use]
    pub fn sr_0b0010(self) -> Command<Sampling<(Normal, BOSR)>> {
        unsafe { self.bits(0b0010) }
    }
    #[must_use]
    pub fn sr_0b0011(self) -> Command<Sampling<(Normal, BOSR)>> {
        unsafe { self.bits(0b0011) }
    }
    #[must_use]
    pub fn sr_0b0110(self) -> Command<Sampling<(Normal, BOSR)>> {
        unsafe { self.bits(0b0110) }
    }
    #[must_use]
    pub fn sr_0b0111(self) -> Command<Sampling<(Normal, BOSR)>> {
        unsafe { self.bits(0b0111) }
    }
    #[must_use]
    pub fn sr_0b1000(self) -> Command<Sampling<(Normal, BOSR)>> {
        unsafe { self.bits(0b1000) }
    }
    #[must_use]
    pub fn sr_0b1001(self) -> Command<Sampling<(Normal, BOSR)>> {
        unsafe { self.bits(0b1001) }
    }
    #[must_use]
    pub fn sr_0b1010(self) -> Command<Sampling<(Normal, BOSR)>> {
        unsafe { self.bits(0b1010) }
    }
    #[must_use]
    pub fn sr_0b1011(self) -> Command<Sampling<(Normal, BOSR)>> {
        unsafe { self.bits(0b1011) }
    }
    #[must_use]
    pub fn sr_0b1111(self) -> Command<Sampling<(Normal, BOSR)>> {
        unsafe { self.bits(0b1111) }
    }
}

impl Sr<(Usb, BosrIsClear)> {
    #[must_use]
    pub fn sr_0b0000(self) -> Command<Sampling<(Usb, BosrIsClear)>> {
        unsafe { self.bits(0b0000) }
    }
    #[must_use]
    pub fn sr_0b0001(self) -> Command<Sampling<(Usb, BosrIsClear)>> {
        unsafe { self.bits(0b0001) }
    }
    #[must_use]
    pub fn sr_0b0010(self) -> Command<Sampling<(Usb, BosrIsClear)>> {
        unsafe { self.bits(0b0010) }
    }
    #[must_use]
    pub fn sr_0b0011(self) -> Command<Sampling<(Usb, BosrIsClear)>> {
        unsafe { self.bits(0b0011) }
    }
    #[must_use]
    pub fn sr_0b0110(self) -> Command<Sampling<(Usb, BosrIsClear)>> {
        unsafe { self.bits(0b0110) }
    }
    #[must_use]
    pub fn sr_0b0111(self) -> Command<Sampling<(Usb, BosrIsClear)>> {
        unsafe { self.bits(0b0111) }
    }
}

impl Sr<(Usb, BosrIsSet)> {
    #[must_use]
    pub fn sr_0b1000(self) -> Command<Sampling<(Usb, BosrIsSet)>> {
        unsafe { self.bits(0b1000) }
    }
    #[must_use]
    pub fn sr_0b1001(self) -> Command<Sampling<(Usb, BosrIsSet)>> {
        unsafe { self.bits(0b1001) }
    }
    #[must_use]
    pub fn sr_0b1010(self) -> Command<Sampling<(Usb, BosrIsSet)>> {
        unsafe { self.bits(0b1010) }
    }
    #[must_use]
    pub fn sr_0b1011(self) -> Command<Sampling<(Usb, BosrIsSet)>> {
        unsafe { self.bits(0b1011) }
    }
    #[must_use]
    pub fn sr_0b1111(self) -> Command<Sampling<(Usb, BosrIsSet)>> {
        unsafe { self.bits(0b1111) }
    }
}

impl_toggle_writer!(Clkidiv2<T>, Command<Sampling<T>>, 6);
impl_toggle_writer!(Clkodiv2<T>, Command<Sampling<T>>, 7);

#[cfg(test)]
mod tests {
    use super::*;
    // all() to compile, any() to not compile
    #[cfg(all())]
    fn _should_compile() {
        let new_cmd = Command::<Sampling<(Unset, Unset)>>::new();
        // for normal mode, setting bosr in not actually required
        let _ = new_cmd.usb_normal().normal().bosr().set_bit().sr().sr_0b1111();
        //in usb mode, we need to set bosr befor sr
        let _ = new_cmd.usb_normal().usb().bosr().set_bit().sr().sr_0b1111();
        //in usb mode, we need to set bosr befor sr
        let _ = new_cmd.usb_normal().usb().bosr().clear_bit().sr().sr_0b0000();
    }
    // all() to compile, any() to not compile
    #[cfg(any())]
    fn _should_compile_warn() {
        let new_cmd = Command::<Sampling<(Unset, Unset)>>::new();
        //should warn, you may think you change the command but this is not the case
        new_cmd.usb_normal().normal();
    }
    // all() to compile, any() to not compile
    #[cfg(any())]
    fn _should_compile_error() {
        let new_cmd = Command::<Sampling<(Unset, Unset)>>::new();
        //error, usb and bosr not set, sr not available
        let _ = new_cmd.sr().sr_0b1111();
        //error, bosr not set, sr not available
        let _ = new_cmd.usb_normal().usb().sr().sr_0b1111();
        //error, cannot set this sr value with this bosr value
        let _ = new_cmd.usb_normal().usb().bosr().clear_bit().sr().sr_0b1111();
        //error, cannot set this sr value with this bosr value
        let _ = new_cmd.usb_normal().usb().bosr().set_bit().sr().sr_0b0000();
        //error, cannot change usb_normal after sr is set
        let _ = new_cmd.usb_normal().normal().sr().sr_0b0000().usb_normal().usb();
    }
}
