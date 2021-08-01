//! Headphone ouputs configuration
#![allow(clippy::new_without_default)]

use crate::{Command, Left, Right};
use core::marker::PhantomData;

///Marker indicating headphone ouputs concern
pub struct HeadphoneOut<CHANNEL> {
    channel: PhantomData<CHANNEL>,
}

///Marker indicating left line in concern
pub type LeftHeadphoneOut = HeadphoneOut<Left>;

///Marker indicating left line in concern
pub type RightHeadphoneOut = HeadphoneOut<Right>;

impl Command<LeftHeadphoneOut> {
    pub fn new() -> Self {
        Self {
            data: 0x2 << 9 | 0b0_0111_1001,
            t: PhantomData::<LeftHeadphoneOut>,
        }
    }
}

impl Command<RightHeadphoneOut> {
    pub fn new() -> Self {
        Self {
            data: 0x3 << 9 | 0b0_0111_1001,
            t: PhantomData::<RightHeadphoneOut>,
        }
    }
}

impl<CHANNEL> Command<HeadphoneOut<CHANNEL>> {
    pub fn hpvol(self) -> Hpvol<CHANNEL> {
        Hpvol { cmd: self }
    }
    pub fn zcen(self) -> Zcen<CHANNEL> {
        Zcen { cmd: self }
    }
    pub fn hpboth(self) -> Hpboth<CHANNEL> {
        Hpboth { cmd: self }
    }
}

///Writer of LHPVOL or RHPVOL fields. Control line input volume.
pub struct Hpvol<CHANNEL> {
    cmd: Command<HeadphoneOut<CHANNEL>>,
}

impl<CHANNEL> Hpvol<CHANNEL> {
    impl_bits!(Command<HeadphoneOut<CHANNEL>>, 7, 0);
}

impl_toggle_writer!(Zcen<CHANNEL>, Command<HeadphoneOut<CHANNEL>>, 7);
impl_toggle_writer!(Hpboth<CHANNEL>, Command<HeadphoneOut<CHANNEL>>, 8);
