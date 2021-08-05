//! Headphone ouputs configuration
#![allow(clippy::new_without_default)]

use crate::{Command, Left, Right};
use core::marker::PhantomData;

///Marker indicating headphone ouputs concern
pub struct HeadphoneOut<CHANNEL> {
    data: u16,
    channel: PhantomData<CHANNEL>,
}

///Marker indicating left line in concern
pub type LeftHeadphoneOut = HeadphoneOut<Left>;

///Marker indicating left line in concern
pub type RightHeadphoneOut = HeadphoneOut<Right>;

/// Instanciate a builder for left line in configuration.
pub fn left_headphone_out() -> LeftHeadphoneOut {
    LeftHeadphoneOut::new()
}

/// Instanciate a builder for right line in configuration.
pub fn right_headphone_out() -> RightHeadphoneOut {
    RightHeadphoneOut::new()
}

impl LeftHeadphoneOut {
    pub fn new() -> Self {
        Self {
            data: 0x2 << 9 | 0b0_0111_1001,
            channel: PhantomData::<Left>,
        }
    }
}

impl RightHeadphoneOut {
    pub fn new() -> Self {
        Self {
            data: 0x3 << 9 | 0b0_0111_1001,
            channel: PhantomData::<Right>,
        }
    }
}

impl<CHANNEL> HeadphoneOut<CHANNEL> {
    pub fn hpvol(self) -> Hpvol<CHANNEL> {
        Hpvol { cmd: self }
    }
    pub fn zcen(self) -> Zcen<CHANNEL> {
        Zcen { cmd: self }
    }
    pub fn hpboth(self) -> Hpboth<CHANNEL> {
        Hpboth { cmd: self }
    }
    pub fn into_command(self) -> Command<()> {
        Command::<()> {
            data: self.data,
            t: PhantomData::<()>,
        }
    }

}

///Writer of LHPVOL or RHPVOL fields. Control line input volume.
pub struct Hpvol<CHANNEL> {
    cmd: HeadphoneOut<CHANNEL>,
}

impl<CHANNEL> Hpvol<CHANNEL> {
    impl_bits!(HeadphoneOut<CHANNEL>, 7, 0);
}

impl_toggle_writer!(Zcen<CHANNEL>, HeadphoneOut<CHANNEL>, 7);
impl_toggle_writer!(Hpboth<CHANNEL>, HeadphoneOut<CHANNEL>, 8);
