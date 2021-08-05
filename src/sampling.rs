//! Command builder for sampling configuration.
//!
//! This module offer two style for building a command, a nice style using a master clock and a raw
//! style allowing more advance use.
//!
//! # Style with master clock
//! With this method, the builder is instantiated with a marker to indicate the internal master clock
//! frequency. Valid markers are :
//!  - `Mclk12M288` for a 12.288 MHz master clock
//!  - `Mclk18M432` for a 18.432 MHz master clock
//!  - `Mclk11M2896` for a 11.2896 MHz master clock
//!  - `Mclk16M9344` for a 16.9344 MHz master clock
//!  - `Mclk12M` for a 12MHz master clock, correspond to USB mode.
//!
//! You also don't write directly to USB/NORMAL, BOSR, or SR fields. Instead, you use a *virtual*
//! SampleRate field that do it for you.
//!
//!  ## Example
//! ```
//! # use wm8731_alt::sampling::*;
//! //instanciate the builder
//! let cmd = sampling_command_builder_mclk(Mclk12M288);
//! //setup the sampling rate
//! let cmd = cmd.sample_rate().adc48k_dac48k();
//! //build the command
//! let cmd = cmd.into_command();
//! ```
//!
//! # Raw style
//! With this method, you write directly to USB/NORMAL, BOSR, and SR fields. This way is useful for
//! case not handled by the other method. Notably, the Sr field writer don't have very meaningful
//! method name, because same combination of USB/NORMAL, BOSR, and SR can produce different
//! sampling rate by just changing the master clock. Look the
//! [WAN0117](https://statics.cirrus.com/pubs/appNote/WAN0117.pdf) application notice for
//! advanced sampling rate selection.
//!
//! ## Example
//! ```
//! # use wm8731_alt::sampling::*;
//! //instantiate the builder
//! let cmd = sampling_command_builder();
//! //normal mode operation
//! let cmd = cmd.usb_normal().normal();
//! //write bosr bit
//! let cmd = cmd.bosr().clear_bit();
//! //write sr field
//! let cmd = cmd.sr().sr_0b0000();
//! //build the command
//! let cmd = cmd.into_command();
//! ```
//!
//! # Safety and coherence
//! For safety and coherence some reason, a compile time check is done to enforce some
//! manipulations and prohibit others. To avoid compilation error from this checks, a good
//! guideline is to always setup explicitly USB/NORMAL, BOSR first and then setup SR for the raw
//! style, and explicitly setup SampleRate fields for the style with master clock. don't forget to
//! check with the datasheet if values are good.
//!
//! ## Example
//! ```
//! # #[cfg(any())] //avoid some compilation error when testing doc
//! # {
//! # use wm8731_alt::sampling::*;
//! //error, sample rate require to be explicitly set
//! let cmd = sampling_command_builder_mclk(Mclk12M288).into_command();
//! //error, this sampling rate setup is impossible with the current master clock
//! let cmd = sampling_command_builder_mclk(Mclk12M288).sample_rate().adc44k1_dac44k1();
//! //error, sr require to be explicitly set
//! let cmd = sampling_command_builder().into_command();
//! //error, USB/NORMAL and BOSR need to be set before SR because it affect SR validity
//! let cmd = sampling_command_builder().sr().sr_0b0000();
//! //error, cannot change USB/NORMAL and BOSR after SR because it affect SR validity
//! let cmd =
//! sampling_command_builder().usb_normal().usb().bosr().clear_bit().sr().sr_0b0000().bosr().set_bit();
//! //error, USB/NORMAL, BOSR, SR combination is invalid
//! let cmd =
//! sampling_command_builder().usb_normal().usb().bosr().set_bit().sr().sr_0b0000();
//! # }
//! ```
//!
//!
#![allow(clippy::new_without_default)]

use crate::Command;
use core::marker::PhantomData;

/// Builder for sampling command.
#[derive(Debug, Eq, PartialEq)]
pub struct Sampling<T> {
    data: u16,
    t: PhantomData<T>,
}

impl<T> Copy for Sampling<T> {}

impl<T> Clone for Sampling<T> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Marker trait to say a marker correspnd to something set.
pub trait IsSet {}

/// Marker used to indicate Normal mode.
pub struct Normal;
impl IsSet for Normal {}
/// Marker used to indicate USB mode.
pub struct Usb;
impl IsSet for Usb {}
/// Marker used to indicate BOSR bit is set.
pub struct BosrIsSet;
impl IsSet for BosrIsSet {}
/// Marker used to indicate BOSR bit is clear.
pub struct BosrIsClear;
impl IsSet for BosrIsClear {}
/// Marker to indicate Sr is exeplictly set.
pub struct SrIsSet;
impl IsSet for SrIsSet {}

/// Marker used to indicate something is not yet defined but required to be.
pub struct Unset;

/// Marker trait to say a marker correspond to a master clock value.
pub trait Mclk {}

///Marker indicating use of 12.288Mhz internal master clock (normal mode).
pub struct Mclk12M288;
impl Mclk for Mclk12M288 {}
///Marker indicating use of 18.432Mhz internal master clock (normal mode).
pub struct Mclk18M432;
impl Mclk for Mclk18M432 {}
///Marker indicating use of 11.2896Mhz internal master clock (normal mode).
pub struct Mclk11M2896;
impl Mclk for Mclk11M2896 {}
///Marker indicating use of 16.9344Mhz internal master clock (normal mode).
pub struct Mclk16M9344;
impl Mclk for Mclk16M9344 {}
///Marker indicating use of 12Mhz internal master clock (USB mode).
pub struct Mclk12M;
impl Mclk for Mclk12M {}

/// Instanciate a command builder for sampling configuration.
pub fn sampling_command_builder() -> Sampling<(Unset, Unset, Unset)> {
    Sampling::<(Unset, Unset, Unset)>::new()
}

/// Instanciate a command builder to set sampling configuration for a particular master clock.
pub fn sampling_command_builder_mclk<MCLK>(_: MCLK) -> Sampling<(MCLK, Unset)>
where
    MCLK: Mclk,
{
    Sampling::<(MCLK, Unset)> {
        data: 0b1000 << 9,
        t: PhantomData::<(MCLK, Unset)>,
    }
}

impl<MCLK, SR> Sampling<(MCLK, SR)>
where
    MCLK: Mclk,
{
    pub fn sample_rate(self) -> SampleRate<(MCLK, SR)> {
        SampleRate::<(MCLK, SR)> { cmd: self }
    }
}
/// Virtual field writer for more meaningfull sampling rate setting.
///
/// This actually write USB/NORMAL, BOSR, and SR fields.
pub struct SampleRate<T> {
    cmd: Sampling<T>,
}

impl<MCLK, SR> SampleRate<(MCLK, SR)> {
    unsafe fn bits(mut self, value: u8) -> Sampling<(MCLK, SrIsSet)> {
        let mask = !((!0) << 6);
        self.cmd.data = self.cmd.data & !mask | (value as u16) << 2 & mask;
        Sampling::<(MCLK, SrIsSet)> {
            data: self.cmd.data,
            t: PhantomData::<(MCLK, SrIsSet)>,
        }
    }
}

impl<SR> SampleRate<(Mclk12M288, SR)> {
    ///Set 48khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc48k_dac48k(self) -> Sampling<(Mclk12M288, SrIsSet)> {
        unsafe { self.bits(0b000000) }
    }
    ///Set sampling rate of 48khz for ADC and 8khz for DAC.
    #[must_use]
    pub fn adc48k_dac8k(self) -> Sampling<(Mclk12M288, SrIsSet)> {
        unsafe { self.bits(0b000100) }
    }
    ///Set sampling rate of 8khz for ADC and 48khz for DAC.
    #[must_use]
    pub fn adc8k_dac48k(self) -> Sampling<(Mclk12M288, SrIsSet)> {
        unsafe { self.bits(0b001000) }
    }
    ///Set 8khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc8k_dac8k(self) -> Sampling<(Mclk12M288, SrIsSet)> {
        unsafe { self.bits(0b001100) }
    }
    ///Set 32khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc32k_dac32k(self) -> Sampling<(Mclk12M288, SrIsSet)> {
        unsafe { self.bits(0b011000) }
    }
    ///Set 96khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc96k_dac96k(self) -> Sampling<(Mclk12M288, SrIsSet)> {
        unsafe { self.bits(0b011100) }
    }
}

impl<SR> SampleRate<(Mclk18M432, SR)> {
    ///Set 48khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc48k_dac48k(self) -> Sampling<(Mclk18M432, SrIsSet)> {
        unsafe { self.bits(0b000010) }
    }
    ///Set sampling rate of 48khz for ADC and 8khz for DAC.
    #[must_use]
    pub fn adc48k_dac8k(self) -> Sampling<(Mclk18M432, SrIsSet)> {
        unsafe { self.bits(0b000110) }
    }
    ///Set sampling rate of 8khz for ADC and 48khz for DAC.
    #[must_use]
    pub fn adc8k_dac48k(self) -> Sampling<(Mclk18M432, SrIsSet)> {
        unsafe { self.bits(0b001010) }
    }
    ///Set 8khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc8k_dac8k(self) -> Sampling<(Mclk18M432, SrIsSet)> {
        unsafe { self.bits(0b001110) }
    }
    ///Set 32khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc32k_dac32k(self) -> Sampling<(Mclk18M432, SrIsSet)> {
        unsafe { self.bits(0b011010) }
    }
    ///Set 96khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc96k_dac96k(self) -> Sampling<(Mclk18M432, SrIsSet)> {
        unsafe { self.bits(0b011110) }
    }
}

impl<SR> SampleRate<(Mclk11M2896, SR)> {
    ///Set 44.1khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc44k1_dac44k1(self) -> Sampling<(Mclk11M2896, SrIsSet)> {
        unsafe { self.bits(0b100000) }
    }
    ///Set sampling rate of 44.1khz for ADC and approximatively 8khz for DAC.
    ///
    ///The actual DAC sampling rate is 8.018kHz
    #[must_use]
    pub fn adc44k1_dac8k(self) -> Sampling<(Mclk11M2896, SrIsSet)> {
        unsafe { self.bits(0b100100) }
    }
    ///Set sampling rate of approximatively 8khz for ADC and 44.1khz for DAC.
    ///
    ///The actual ADC sampling rate is 8.018kHz
    #[must_use]
    pub fn adc8k_dac44k1(self) -> Sampling<(Mclk11M2896, SrIsSet)> {
        unsafe { self.bits(0b101000) }
    }
    ///Set approximatively 8khz sampling rate for ADC and DAC.
    ///
    ///The actual sampling rate is 8.018kHz
    #[must_use]
    pub fn adc8k_dac8k(self) -> Sampling<(Mclk11M2896, SrIsSet)> {
        unsafe { self.bits(0b101100) }
    }
    ///Set 88.2khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc88k2_dac88k2(self) -> Sampling<(Mclk11M2896, SrIsSet)> {
        unsafe { self.bits(0b111100) }
    }
}

impl<SR> SampleRate<(Mclk16M9344, SR)> {
    ///Set 44.1khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc44k1_dac44k1(self) -> Sampling<(Mclk16M9344, SrIsSet)> {
        unsafe { self.bits(0b100010) }
    }
    ///Set sampling rate of 44.1khz for ADC and approximatively 8khz for DAC.
    ///
    ///The actual DAC sampling rate is 8.018kHz
    #[must_use]
    pub fn adc44k1_dac8k(self) -> Sampling<(Mclk16M9344, SrIsSet)> {
        unsafe { self.bits(0b100110) }
    }
    ///Set sampling rate of approximatively 8khz for ADC and 44.1khz for DAC.
    ///
    ///The actual ADC sampling rate is 8.018kHz
    #[must_use]
    pub fn adc8k_dac44k1(self) -> Sampling<(Mclk16M9344, SrIsSet)> {
        unsafe { self.bits(0b101010) }
    }
    ///Set approximatively 8khz sampling rate for ADC and DAC.
    ///
    ///The actual sampling rate is 8.018kHz
    #[must_use]
    pub fn adc8k_dac8k(self) -> Sampling<(Mclk16M9344, SrIsSet)> {
        unsafe { self.bits(0b101110) }
    }
    ///Set 88.2khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc88k2_dac88k2(self) -> Sampling<(Mclk16M9344, SrIsSet)> {
        unsafe { self.bits(0b111110) }
    }
}

impl<SR> SampleRate<(Mclk12M, SR)> {
    ///Set 48khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc48k_dac48k(self) -> Sampling<(Mclk12M, SrIsSet)> {
        unsafe { self.bits(0b000001) }
    }
    ///Set approximatively 44.1khz sampling rate for ADC and DAC.
    ///
    ///The actual sampling rate is 44.118kHz.
    #[must_use]
    pub fn adc44k1_dac44k1(self) -> Sampling<(Mclk12M, SrIsSet)> {
        unsafe { self.bits(0b100011) }
    }
    ///Set sampling rate of 48khz for ADC and 8khz for DAC.
    #[must_use]
    pub fn adc48k_dac8k(self) -> Sampling<(Mclk12M, SrIsSet)> {
        unsafe { self.bits(0b000101) }
    }
    ///Set sampling rate of approximatively 44.1khz for ADC and approximatively 8khz for DAC.
    ///
    ///The actual sampling rate are 44.118kHz for the ADC and 8.021kHz for the DAC.
    #[must_use]
    pub fn adc44k1_dac8k(self) -> Sampling<(Mclk12M, SrIsSet)> {
        unsafe { self.bits(0b100111) }
    }
    ///Set sampling rate of 8khz for ADC and 48khz for DAC.
    #[must_use]
    pub fn adc8k_dac48k(self) -> Sampling<(Mclk12M, SrIsSet)> {
        unsafe { self.bits(0b001001) }
    }
    ///Set sampling rate of approximatively 8khz for ADC and approximatively 44.1khz for DAC.
    ///
    ///The actual sampling rate are 8.021kHz for the ADC and 44.118kHz  for the DAC.
    #[must_use]
    pub fn adc8k_dac44k1(self) -> Sampling<(Mclk12M, SrIsSet)> {
        unsafe { self.bits(0b101011) }
    }
    ///Set 8khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc8k_dac8k(self) -> Sampling<(Mclk12M, SrIsSet)> {
        unsafe { self.bits(0b001101) }
    }
    ///Set approximatively 8khz sampling rate for ADC and DAC.
    ///
    ///The actual sampling rate is 8.021kHz.
    #[must_use]
    pub fn adc8k_dac8k_bis(self) -> Sampling<(Mclk12M, SrIsSet)> {
        unsafe { self.bits(0b101111) }
    }
    ///Set 32khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc32k_dac32k(self) -> Sampling<(Mclk12M, SrIsSet)> {
        unsafe { self.bits(0b011001) }
    }
    ///Set 96khz sampling rate for ADC and DAC.
    #[must_use]
    pub fn adc96k_dac96k(self) -> Sampling<(Mclk12M, SrIsSet)> {
        unsafe { self.bits(0b011101) }
    }
    ///Set approximatively 88.2kHz sampling rate for ADC and DAC.
    ///
    ///The actual sampling rate is 88.235kHz.
    #[must_use]
    pub fn adc88k2_dac88k2(self) -> Sampling<(Mclk12M, SrIsSet)> {
        unsafe { self.bits(0b111111) }
    }
}

impl Sampling<(Unset, Unset, Unset)> {
    #[allow(clippy::identity_op)]
    pub fn new() -> Self {
        Self {
            data: 0b1000 << 9 | 0b0000_0000,
            t: PhantomData::<(Unset, Unset, Unset)>,
        }
    }
}

//it's always safe to manipulate those fields
impl<T> Sampling<T> {
    pub fn clkidiv2(self) -> Clkidiv2<T> {
        Clkidiv2 { cmd: self }
    }
    pub fn clkodiv2(self) -> Clkodiv2<T> {
        Clkodiv2 { cmd: self }
    }
}

//Once sr have been explicitly set, a valid command can be instanciated
impl<MODE, BOSR> Sampling<(MODE, BOSR, SrIsSet)> {
    /// Instanciate a command
    pub fn into_command(self) -> Command<()> {
        Command::<()> {
            data: self.data,
            t: PhantomData::<()>,
        }
    }
}

//Once SampleRate have been explicitly set, a valid command can be instanciated
impl<MCLK> Sampling<(MCLK, SrIsSet)> {
    /// Instanciate a command
    pub fn into_command(self) -> Command<()> {
        Command::<()> {
            data: self.data,
            t: PhantomData::<()>,
        }
    }
}

//make the normal/usb mode only settable once (over constraint?)
//prevent to change normal/ub mode once sr is set
impl<BOSR> Sampling<(Unset, BOSR, Unset)> {
    pub fn usb_normal(self) -> UsbNormal<(Unset, BOSR, Unset)> {
        UsbNormal { cmd: self }
    }
}

//make the bosr bit only settable once (over constraint?)
//prevent to change bosr bit once sr is set
impl<MODE> Sampling<(MODE, Unset, Unset)> {
    pub fn bosr(self) -> Bosr<(MODE, Unset, Unset)> {
        Bosr { cmd: self }
    }
}

//When Usb mode is explicitly set, enforce bosr is set before setting sr
impl<BOSR, SR> Sampling<(Usb, BOSR, SR)>
where
    BOSR: IsSet,
{
    pub fn sr(self) -> Sr<(Usb, BOSR, SR)> {
        Sr { cmd: self }
    }
}

//When Normal mode, sr validity is no affect by bosr, so no need to explicitly set it before setting
//SR
impl<BOSR, SR> Sampling<(Normal, BOSR, SR)> {
    pub fn sr(self) -> Sr<(Normal, BOSR, SR)> {
        Sr { cmd: self }
    }
}

/// Field writer. Allow to select USB or Normal mode.
pub struct UsbNormal<T> {
    cmd: Sampling<T>,
}

impl<MODE, BOSR, SR> UsbNormal<(MODE, BOSR, SR)> {
    #[must_use]
    pub fn clear_bit(mut self) -> Sampling<(Normal, BOSR, SR)> {
        self.cmd.data &= !(0b1 << 0);
        Sampling::<(Normal, BOSR, SR)> {
            data: self.cmd.data,
            t: PhantomData::<(Normal, BOSR, SR)>,
        }
    }
    #[must_use]
    pub fn set_bit(mut self) -> Sampling<(Usb, BOSR, SR)> {
        self.cmd.data |= 0b1 << 0;
        Sampling::<(Usb, BOSR, SR)> {
            data: self.cmd.data,
            t: PhantomData::<(Usb, BOSR, SR)>,
        }
    }
    #[must_use]
    pub fn normal(mut self) -> Sampling<(Normal, BOSR, SR)> {
        self.cmd.data &= !(0b1 << 0);
        Sampling::<(Normal, BOSR, SR)> {
            data: self.cmd.data,
            t: PhantomData::<(Normal, BOSR, SR)>,
        }
    }
    #[must_use]
    pub fn usb(mut self) -> Sampling<(Usb, BOSR, SR)> {
        self.cmd.data |= 0b1 << 0;
        Sampling::<(Usb, BOSR, SR)> {
            data: self.cmd.data,
            t: PhantomData::<(Usb, BOSR, SR)>,
        }
    }
}

/// Field writer. Select the Base Over-Sampling Rate.
pub struct Bosr<T> {
    cmd: Sampling<T>,
}

impl<MODE, BOSR, SR> Bosr<(MODE, BOSR, SR)> {
    #[must_use]
    pub fn clear_bit(mut self) -> Sampling<(MODE, BosrIsClear, SR)> {
        self.cmd.data &= !(0b1 << 1);
        Sampling::<(MODE, BosrIsClear, SR)> {
            data: self.cmd.data,
            t: PhantomData::<(MODE, BosrIsClear, SR)>,
        }
    }
    #[must_use]
    pub fn set_bit(mut self) -> Sampling<(MODE, BosrIsSet, SR)> {
        self.cmd.data |= 0b1 << 1;
        Sampling::<(MODE, BosrIsSet, SR)> {
            data: self.cmd.data,
            t: PhantomData::<(MODE, BosrIsSet, SR)>,
        }
    }
}

/// Field writer. Allow to write raw bits into the sr field.
pub struct Sr<T> {
    cmd: Sampling<T>,
}

impl<MODE, BOSR, SR> Sr<(MODE, BOSR, SR)> {
    //impl_bits!(unsafe, Sampling<T>, 4, 2);
    /// Set the field with raw bits.
    ///
    /// # Safety
    ///
    /// Some bit combinations are invalid, please read the datasheet.
    pub unsafe fn bits(mut self, value: u8) -> Sampling<(MODE, BOSR, SrIsSet)> {
        let mask = !((!0) << 4) << 2;
        self.cmd.data = self.cmd.data & !mask | (value as u16) << 2 & mask;
        Sampling::<(MODE, BOSR, SrIsSet)> {
            data: self.cmd.data,
            t: PhantomData::<(MODE, BOSR, SrIsSet)>,
        }
    }
}

impl<BOSR, SR> Sr<(Normal, BOSR, SR)> {
    #[must_use]
    pub fn sr_0b0000(self) -> Sampling<(Normal, BOSR, SrIsSet)> {
        unsafe { self.bits(0b0000) }
    }
    #[must_use]
    pub fn sr_0b0001(self) -> Sampling<(Normal, BOSR, SrIsSet)> {
        unsafe { self.bits(0b0001) }
    }
    #[must_use]
    pub fn sr_0b0010(self) -> Sampling<(Normal, BOSR, SrIsSet)> {
        unsafe { self.bits(0b0010) }
    }
    #[must_use]
    pub fn sr_0b0011(self) -> Sampling<(Normal, BOSR, SrIsSet)> {
        unsafe { self.bits(0b0011) }
    }
    #[must_use]
    pub fn sr_0b0110(self) -> Sampling<(Normal, BOSR, SrIsSet)> {
        unsafe { self.bits(0b0110) }
    }
    #[must_use]
    pub fn sr_0b0111(self) -> Sampling<(Normal, BOSR, SrIsSet)> {
        unsafe { self.bits(0b0111) }
    }
    #[must_use]
    pub fn sr_0b1000(self) -> Sampling<(Normal, BOSR, SrIsSet)> {
        unsafe { self.bits(0b1000) }
    }
    #[must_use]
    pub fn sr_0b1001(self) -> Sampling<(Normal, BOSR, SrIsSet)> {
        unsafe { self.bits(0b1001) }
    }
    #[must_use]
    pub fn sr_0b1010(self) -> Sampling<(Normal, BOSR, SrIsSet)> {
        unsafe { self.bits(0b1010) }
    }
    #[must_use]
    pub fn sr_0b1011(self) -> Sampling<(Normal, BOSR, SrIsSet)> {
        unsafe { self.bits(0b1011) }
    }
    #[must_use]
    pub fn sr_0b1111(self) -> Sampling<(Normal, BOSR, SrIsSet)> {
        unsafe { self.bits(0b1111) }
    }
}

impl<SR> Sr<(Usb, BosrIsClear, SR)> {
    #[must_use]
    pub fn sr_0b0000(self) -> Sampling<(Usb, BosrIsClear, SrIsSet)> {
        unsafe { self.bits(0b0000) }
    }
    #[must_use]
    pub fn sr_0b0001(self) -> Sampling<(Usb, BosrIsClear, SrIsSet)> {
        unsafe { self.bits(0b0001) }
    }
    #[must_use]
    pub fn sr_0b0010(self) -> Sampling<(Usb, BosrIsClear, SrIsSet)> {
        unsafe { self.bits(0b0010) }
    }
    #[must_use]
    pub fn sr_0b0011(self) -> Sampling<(Usb, BosrIsClear, SrIsSet)> {
        unsafe { self.bits(0b0011) }
    }
    #[must_use]
    pub fn sr_0b0110(self) -> Sampling<(Usb, BosrIsClear, SrIsSet)> {
        unsafe { self.bits(0b0110) }
    }
    #[must_use]
    pub fn sr_0b0111(self) -> Sampling<(Usb, BosrIsClear, SrIsSet)> {
        unsafe { self.bits(0b0111) }
    }
}

impl<SR> Sr<(Usb, BosrIsSet, SR)> {
    #[must_use]
    pub fn sr_0b1000(self) -> Sampling<(Usb, BosrIsSet, SrIsSet)> {
        unsafe { self.bits(0b1000) }
    }
    #[must_use]
    pub fn sr_0b1001(self) -> Sampling<(Usb, BosrIsSet, SrIsSet)> {
        unsafe { self.bits(0b1001) }
    }
    #[must_use]
    pub fn sr_0b1010(self) -> Sampling<(Usb, BosrIsSet, SrIsSet)> {
        unsafe { self.bits(0b1010) }
    }
    #[must_use]
    pub fn sr_0b1011(self) -> Sampling<(Usb, BosrIsSet, SrIsSet)> {
        unsafe { self.bits(0b1011) }
    }
    #[must_use]
    pub fn sr_0b1111(self) -> Sampling<(Usb, BosrIsSet, SrIsSet)> {
        unsafe { self.bits(0b1111) }
    }
}

impl_toggle_writer!(Clkidiv2<T>, Sampling<T>, 6);
impl_toggle_writer!(Clkodiv2<T>, Sampling<T>, 7);

#[cfg(test)]
mod tests {
    use super::*;
    // all() to compile, any() to not compile
    #[cfg(all())]
    fn _should_compile() {
        let new_cmd = sampling_command_builder();
        // for normal mode, setting bosr in not actually required
        let _ = new_cmd
            .usb_normal()
            .normal()
            .bosr()
            .set_bit()
            .sr()
            .sr_0b1111()
            .into_command();
        //in usb mode, we need to set bosr before sr
        let _ = new_cmd.usb_normal().usb().bosr().set_bit().sr().sr_0b1111();
        //in usb mode, we need to set bosr before sr
        let _ = new_cmd
            .usb_normal()
            .usb()
            .bosr()
            .clear_bit()
            .sr()
            .sr_0b0000();
    }
    // all() to compile, any() to not compile
    #[cfg(any())]
    fn _should_compile_warn() {
        let new_cmd = sampling_command_builder();
        //should warn, you may think you change the command but this is not the case
        new_cmd.usb_normal().normal();

        let cmd = sampling_command_builder_mclk(Mclk12M288);
        //should warn, you may think you change the command but this is not the case
        cmd.sample_rate().adc48k_dac48k();
    }
    // all() to compile, any() to not compile
    #[cfg(any())]
    fn _should_compile_error() {
        let new_cmd = sampling_command_builder();
        //error, bosr not set in usb mode sr not available
        let _ = new_cmd.usb_normal().usb().sr().sr_0b1111();
        //error, cannot set this sr value with this bosr value
        let _ = new_cmd
            .usb_normal()
            .usb()
            .bosr()
            .clear_bit()
            .sr()
            .sr_0b1111();
        //error, cannot set this sr value with this bosr value
        let _ = new_cmd.usb_normal().usb().bosr().set_bit().sr().sr_0b0000();
        //error, cannot change usb_normal after sr is set
        let _ = new_cmd
            .usb_normal()
            .normal()
            .sr()
            .sr_0b0000()
            .usb_normal()
            .usb();
        //error, cannot change bosr after sr is set
        let _ = new_cmd
            .usb_normal()
            .normal()
            .sr()
            .sr_0b0000()
            .bosr()
            .set_bit();
    }
}
