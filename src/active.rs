//! Activate or deactivate digital audio interface
#![allow(clippy::new_without_default)]

use crate::Command;
use core::marker::PhantomData;

///Marker idicating an "Active" command
pub struct Active;

impl_command_new!(Active, 0b1001, 0b0);

impl Command<Active> {
    ///Activate digital audio interface
    #[must_use]
    pub fn active(mut self) -> Command<Active> {
        self.data |= 0b1;
        self
    }
    ///Dectivate digital audio interface
    #[must_use]
    pub fn inactive(mut self) -> Command<Active> {
        self.data &= !(0b1);
        self
    }
}
