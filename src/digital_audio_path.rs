//! Digital Audio Path configuration
#![allow(clippy::new_without_default)]

use crate::Command;
use core::marker::PhantomData;

///Marker indicating Digital Audio Path concern
pub struct DigitalAudioPath;

impl_command_new!(DigitalAudioPath, 0b101, 0b1000);

impl Command<DigitalAudioPath> {
    pub fn adchpd(self) -> Adchpd {
        Adchpd { cmd: self }
    }
    pub fn deemp(self) -> Deemp {
        Deemp { cmd: self }
    }
    pub fn dacmu(self) -> Dacmu {
        Dacmu { cmd: self }
    }
    pub fn hpor(self) -> Hpor {
        Hpor { cmd: self }
    }
}

impl_toggle_writer!(Adchpd, Command<DigitalAudioPath>, 0);
impl_toggle_writer!(Dacmu, Command<DigitalAudioPath>, 3);

pub enum DeempV {
    Disable = 0b00,
    F32k = 0b01,
    F44k1 = 0b10,
    F48k = 0b11,
}

pub struct Deemp {
    cmd: Command<DigitalAudioPath>,
}

impl Deemp {
    impl_bits!(Command<DigitalAudioPath>, 2, 1);

    #[must_use]
    pub fn variant(self, value: DeempV) -> Command<DigitalAudioPath> {
        match value {
            DeempV::Disable => self.bits(0b00),
            DeempV::F32k => self.bits(0b01),
            DeempV::F44k1 => self.bits(0b10),
            DeempV::F48k => self.bits(0b11),
        }
    }
    pub fn disable(self) -> Command<DigitalAudioPath> {
        self.bits(0b00)
    }
    pub fn f32k(self) -> Command<DigitalAudioPath> {
        self.bits(0b01)
    }
    pub fn f44k1(self) -> Command<DigitalAudioPath> {
        self.bits(0b10)
    }
    pub fn f48k(self) -> Command<DigitalAudioPath> {
        self.bits(0b11)
    }
}

pub enum HporV {
    ClearOffset,
    StoreOffset,
}

pub struct Hpor {
    cmd: Command<DigitalAudioPath>,
}

impl Hpor {
    impl_bit!(Command<DigitalAudioPath>, 4);
    impl_clear_bit!(Command<DigitalAudioPath>, 4);
    impl_set_bit!(Command<DigitalAudioPath>, 4);
    impl_clear_bit!(clear_offset, Command<DigitalAudioPath>, 4);
    impl_set_bit!(store_offset, Command<DigitalAudioPath>, 4);

    #[must_use]
    pub fn variant(self, value: HporV) -> Command<DigitalAudioPath> {
        match value {
            HporV::ClearOffset => self.clear_offset(),
            HporV::StoreOffset => self.store_offset(),
        }
    }
}
