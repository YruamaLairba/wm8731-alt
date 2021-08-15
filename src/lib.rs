#![no_std]
use core::marker::PhantomData;
use embedded_hal::blocking::{i2c, spi};
use embedded_hal::digital::v2::OutputPin;

#[macro_use]
mod macros;

pub mod active_control;
pub mod analogue_audio_path;
pub mod digital_audio_interface;
pub mod digital_audio_path;
pub mod headphone_out;
pub mod line_in;
pub mod power_down;
pub mod sampling;

pub mod reset {
    //! Reset the device
    #![allow(clippy::new_without_default)]
    use crate::Command;
    use core::marker::PhantomData;
    /// Reset command builder.
    #[derive(Debug, Eq, PartialEq)]
    pub struct Reset {
        data: u16,
    }

    impl Copy for Reset {}

    impl Clone for Reset {
        fn clone(&self) -> Self {
            *self
        }
    }

    /// Instantiate a reset command builder.
    pub fn reset() -> Reset {
        Reset::new()
    }

    impl Reset {
        fn new() -> Self {
            Self {
                data: 0b110 << 9 | 0b1001_1111,
            }
        }
        pub fn into_command(self) -> Command<()> {
            Command::<()> {
                data: self.data,
                t: PhantomData::<()>,
            }
        }
    }
}

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

/// Serial Interface abstraction for the wm8731 generic driver.
pub trait WriteCommand {
    fn send<T>(&mut self, cmd: Command<T>);
}

/// Generic blocking I2C communication implementation using embedded-hal.
pub struct I2CInterface<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> I2CInterface<I2C>
where
    I2C: i2c::Write,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }
    pub fn release(self) -> I2C {
        self.i2c
    }
}

impl<I2C> WriteCommand for I2CInterface<I2C>
where
    I2C: i2c::Write,
{
    fn send<T>(&mut self, cmd: Command<T>) {
        let frame: [u8; 2] = cmd.into();
        let _ = self.i2c.write(self.address, &frame);
    }
}

/// Generic blocking SPI communication implementation using embedded-hal.
pub struct SPIInterface<SPI, CS, W> {
    spi: SPI,
    cs: CS,
    w: PhantomData<W>,
}

impl<SPI, CS, W> SPIInterface<SPI, CS, W> {
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self {
            spi,
            cs,
            w: PhantomData::<W>,
        }
    }
    pub fn release(self) -> SPI {
        self.spi
    }
}

impl<SPI, CS> WriteCommand for SPIInterface<SPI, CS, u8>
where
    SPI: spi::Write<u8>,
    CS: OutputPin,
{
    fn send<T>(&mut self, cmd: Command<T>) {
        let frame: [u8; 2] = cmd.into();
        let _ = self.cs.set_low();
        let _ = self.spi.write(&frame);
        let _ = self.cs.set_high();
    }
}

impl<SPI, CS> WriteCommand for SPIInterface<SPI, CS, u16>
where
    SPI: spi::Write<u16>,
    CS: OutputPin,
{
    fn send<T>(&mut self, cmd: Command<T>) {
        let frame: [u16; 1] = cmd.into();
        let _ = self.cs.set_low();
        let _ = self.spi.write(&frame);
        let _ = self.cs.set_high();
    }
}

pub struct Wm8731<I> {
    interface: I,
}

impl<I> Wm8731<I>
where
    I: WriteCommand,
{
    ///Reset the codec and instantiate a driver.
    pub fn new(mut interface: I) -> Self {
        use crate::reset::*;
        interface.send(reset().into_command());
        Self { interface }
    }
    pub fn send<T>(&mut self, cmd: Command<T>) {
        self.interface.send(cmd);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal::blocking::{i2c, spi};
    use embedded_hal::digital::v2::OutputPin;

    struct FakeSpi;
    impl spi::Write<u8> for FakeSpi {
        type Error = ();
        fn write(&mut self, _words: &[u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }
    impl spi::Write<u16> for FakeSpi {
        type Error = ();
        fn write(&mut self, _words: &[u16]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    struct FakePin;

    impl OutputPin for FakePin {
        type Error = ();
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    fn _should_compile() {
        use crate::reset::*;
        let mut spi_if: SPIInterface<_, _, u8> = SPIInterface::new(FakeSpi, FakePin);
        spi_if.send(reset().into_command());
        let _wm8731 = Wm8731::new(spi_if);
    }
}
