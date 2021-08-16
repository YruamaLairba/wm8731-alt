#![no_std]
use core::marker::PhantomData;

use crate::interface::WriteFrame;

#[macro_use]
mod macros;

pub mod active_control;
pub mod analogue_audio_path;
pub mod digital_audio_interface;
pub mod digital_audio_path;
pub mod headphone_out;
pub mod interface;
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

pub struct Wm8731<I> {
    interface: I,
}

impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    ///Reset the codec and instantiate a driver.
    pub fn new(interface: I) -> Self {
        use crate::reset::*;
        let mut codec = Self { interface };
        codec.send(reset().into_command());
        codec
    }
    pub fn send<T>(&mut self, cmd: Command<T>) {
        self.interface.send(cmd.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal::blocking::spi;
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
        use crate::interface::SPIInterface;
        let spi_if: SPIInterface<_, _, u8> = SPIInterface::new(FakeSpi, FakePin);
        let _wm8731 = Wm8731::new(spi_if);
    }
    #[cfg(any())]
    fn _should_not_compile() {
        use crate::interface::SPIInterface;
        use crate::reset::*;
        let mut spi_if: SPIInterface<_, _, u8> = SPIInterface::new(FakeSpi, FakePin);
        //forbidden to encourage using the driver instead serial interface
        spi_if.send(reset().into_command());
    }
}
