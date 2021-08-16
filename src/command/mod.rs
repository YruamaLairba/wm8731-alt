//! Command for wm8731

use core::marker::PhantomData;

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

///Represent a command to send to the codec, that is register address and content to write in it.
#[derive(Debug, Eq, PartialEq)]
pub struct Command<T> {
    pub(crate) data: u16,
    t: PhantomData<T>,
}

impl<T> Copy for Command<T> {}

impl<T> Clone for Command<T> {
    fn clone(&self) -> Self {
        *self
    }
}
