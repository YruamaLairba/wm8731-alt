#![no_std]
use core::marker::PhantomData;

#[macro_use]
mod macros;

pub mod active;
pub mod analogue_audio_path;
pub mod digital_audio_interface;
pub mod digital_audio_path;
pub mod headphone_out;
pub mod line_in;
pub mod power_down;
pub mod sampling;

///Marker indicating left channel concern
pub struct Left;

///Marker indicating right channel concern
pub struct Right;

///Represent a command to send to the codec, that is register address and content to write in it.
#[derive(Debug, Eq, PartialEq)]
pub struct Command<T> {
    data: u16,
    t: PhantomData<T>,
}

impl<T> Copy for Command<T> {}

impl<T> Clone for Command<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> From<Command<T>> for [u8; 2] {
    ///Allow to convert command to an array directly usable with SPI and I2C abstraction from embedded-hal.
    fn from(cmd: Command<T>) -> [u8; 2] {
        cmd.data.to_be_bytes()
    }
}

impl<T> From<Command<T>> for [u16; 1] {
    ///Allow to convert command to an array directly usable with 16 bit word SPI abstraction from embedded-hal.
    fn from(cmd: Command<T>) -> [u16; 1] {
        [cmd.data]
    }
}

impl<T> From<Command<T>> for u16 {
    ///Allow to convert command in u16.
    fn from(cmd: Command<T>) -> u16 {
        cmd.data
    }
}

//#[cfg(test)]
mod tests {}
