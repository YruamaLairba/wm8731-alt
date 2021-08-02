//! Activate or deactivate digital audio interface
#![allow(clippy::new_without_default)]

use crate::Command;
use core::marker::PhantomData;

///Marker idicating an "Active Control" command
pub struct ActiveControl;

impl_command_new!(ActiveControl, 0b1001, 0b0);

impl Command<ActiveControl> {
    ///Activate digital audio interface
    #[must_use]
    pub fn active(mut self) -> Command<ActiveControl> {
        self.data |= 0b1;
        self
    }
    ///Dectivate digital audio interface
    #[must_use]
    pub fn inactive(mut self) -> Command<ActiveControl> {
        self.data &= !(0b1);
        self
    }
}
