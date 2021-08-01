//! Digital Audio Path configuration
#![allow(clippy::new_without_default)]

use crate::Command;
use core::marker::PhantomData;

///Marker indicating Digital Audio Path concern
pub struct DigitalAudioInterface;

impl_command_new!(DigitalAudioInterface, 0b110, 0b0000_1010);

impl Command<DigitalAudioInterface> {
    pub fn format(self) -> Format {
        Format { cmd: self }
    }
    pub fn iwl(self) -> Iwl {
        Iwl { cmd: self }
    }
    pub fn lrp(self) -> Lrp {
        Lrp { cmd: self }
    }
    pub fn lrswap(self) -> Lrswap {
        Lrswap { cmd: self }
    }
    pub fn ms(self) -> Ms {
        Ms { cmd: self }
    }
    pub fn bclkinv(self) -> Bclkinv {
        Bclkinv { cmd: self }
    }
}

pub enum FormatV {
    Dsp = 0b11,
    I2s = 0b10,
    LeftJustified = 0b01,
    RigthJustified = 0b00,
}

pub struct Format {
    cmd: Command<DigitalAudioInterface>,
}

impl Format {
    impl_bits!(Command<DigitalAudioInterface>, 2, 0);

    #[must_use]
    pub fn variant(self, value: FormatV) -> Command<DigitalAudioInterface> {
        match value {
            FormatV::Dsp => self.bits(0b11),
            FormatV::I2s => self.bits(0b10),
            FormatV::LeftJustified => self.bits(0b01),
            FormatV::RigthJustified => self.bits(0b00),
        }
    }
    
    #[must_use]
    pub fn dsp(self) -> Command<DigitalAudioInterface> {
        self.bits(0b11)
    }
    #[must_use]
    pub fn i2s(self) -> Command<DigitalAudioInterface> {
        self.bits(0b10)
    }
    #[must_use]
    pub fn left_justified(self) -> Command<DigitalAudioInterface> {
        self.bits(0b01)
    }
    #[must_use]
    pub fn right_justified(self) -> Command<DigitalAudioInterface> {
        self.bits(0b00)
    }
}

pub enum IwlV {
    Iwl32bits = 0b11,
    Iwl24bits = 0b10,
    Iwl20bits = 0b01,
    Iwl16bits = 0b00,
}

pub struct Iwl {
    cmd: Command<DigitalAudioInterface>,
}

impl Iwl {
    impl_bits!(Command<DigitalAudioInterface>, 2, 2);

    #[must_use]
    pub fn variant(self, value: IwlV) -> Command<DigitalAudioInterface> {
        match value {
            IwlV::Iwl32bits => self.bits(0b11),
            IwlV::Iwl24bits => self.bits(0b10),
            IwlV::Iwl20bits => self.bits(0b01),
            IwlV::Iwl16bits => self.bits(0b00),
        }
    }
    #[must_use]
    pub fn iwl_32_bits(self) -> Command<DigitalAudioInterface> {
        self.bits(0b11)
    }
    #[must_use]
    pub fn iwl_24_bits(self) -> Command<DigitalAudioInterface> {
        self.bits(0b10)
    }
    #[must_use]
    pub fn iwl_20_bits(self) -> Command<DigitalAudioInterface> {
        self.bits(0b01)
    }
    #[must_use]
    pub fn iwl_16_bits(self) -> Command<DigitalAudioInterface> {
        self.bits(0b00)
    }
}

pub struct Lrp {
    cmd: Command<DigitalAudioInterface>,
}

impl Lrp {
    impl_bit!(Command<DigitalAudioInterface>, 4);
    impl_clear_bit!(Command<DigitalAudioInterface>, 4);
    impl_set_bit!(Command<DigitalAudioInterface>, 4);
}

impl_toggle_writer!(Lrswap, Command<DigitalAudioInterface>, 5);

pub enum MsV {
    Master = 0b1,
    Slave = 0b0,
}

pub struct Ms {
    cmd: Command<DigitalAudioInterface>,
}

impl Ms {
    impl_bit!(Command<DigitalAudioInterface>, 6);
    impl_clear_bit!(Command<DigitalAudioInterface>, 6);
    impl_set_bit!(Command<DigitalAudioInterface>, 6);
    impl_clear_bit!(slave, Command<DigitalAudioInterface>, 6);
    impl_set_bit!(master, Command<DigitalAudioInterface>, 6);

    #[must_use]
    pub fn variant(self, value: MsV) -> Command<DigitalAudioInterface> {
        match value {
            MsV::Slave => self.slave(),
            MsV::Master => self.master(),
        }
    }
}

impl_toggle_writer!(Bclkinv, Command<DigitalAudioInterface>, 7);
