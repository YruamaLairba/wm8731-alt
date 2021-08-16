#![no_std]
use crate::command::Command;
use crate::interface::WriteFrame;

#[macro_use]
mod macros;

pub mod command;
pub mod interface;

///The wm8731 driver
pub struct Wm8731<I> {
    interface: I,
}

impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    ///Instantiate a driver. This also reset the codec to guarantee a known state.
    pub fn new(interface: I) -> Self {
        use crate::command::reset::*;
        let mut codec = Self { interface };
        codec.send(reset().into_command());
        codec
    }

    ///Send a command to the codec.
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
