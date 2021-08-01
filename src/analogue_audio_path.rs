//! Analogue Audio Path configuration
#![allow(clippy::new_without_default)]

use crate::Command;
use core::marker::PhantomData;

///Marker indicating Analogue Audio Path concern
pub struct AnalogueAudioPath;

impl_command_new!(AnalogueAudioPath, 0b100, 0b1010);

impl Command<AnalogueAudioPath> {
    pub fn micboost(self) -> Micboost {
        Micboost { cmd: self }
    }
    pub fn mutemic(self) -> Mutemic {
        Mutemic { cmd: self }
    }
    pub fn insel(self) -> Insel {
        Insel { cmd: self }
    }
    pub fn bypass(self) -> Bypass {
        Bypass { cmd: self }
    }
    pub fn dacsel(self) -> Dacsel {
        Dacsel { cmd: self }
    }
    pub fn sidetone(self) -> Sidetone {
        Sidetone { cmd: self }
    }
    pub fn sideatt(self) -> Sideatt {
        Sideatt { cmd: self }
    }
}

impl_toggle_writer!(Micboost, Command<AnalogueAudioPath>, 0);
impl_toggle_writer!(Mutemic, Command<AnalogueAudioPath>, 1);
impl_toggle_writer!(Bypass, Command<AnalogueAudioPath>, 3);
impl_toggle_writer!(Sidetone, Command<AnalogueAudioPath>, 5);

pub enum InselV{
    Line,
    Microphone,
}

pub struct Insel {
    cmd: Command<AnalogueAudioPath>
}

impl Insel {
    impl_bit!(Command<AnalogueAudioPath>,2);
    impl_set_bit!(Command<AnalogueAudioPath>,2);
    impl_clear_bit!(Command<AnalogueAudioPath>,2);
    impl_set_bit!(microphone, Command<AnalogueAudioPath>,2);
    impl_clear_bit!(line, Command<AnalogueAudioPath>,2);

    #[must_use]
    pub fn variant(self,value: InselV) -> Command<AnalogueAudioPath> {
        match value {
            InselV::Microphone => self.microphone(),
            InselV::Line => self.line(),
        }
    }
}


pub enum DacselV{
    Deselect,
    Select,
}

pub struct Dacsel {
    cmd: Command<AnalogueAudioPath>
}

impl Dacsel {
    impl_bit!(Command<AnalogueAudioPath>,2);
    impl_set_bit!(Command<AnalogueAudioPath>,2);
    impl_clear_bit!(Command<AnalogueAudioPath>,2);
    impl_set_bit!(select, Command<AnalogueAudioPath>,2);
    impl_clear_bit!(deselect, Command<AnalogueAudioPath>,2);

    #[must_use]
    pub fn variant(self,value: DacselV) -> Command<AnalogueAudioPath> {
        match value {
            DacselV::Deselect => self.select(),
            DacselV::Select => self.deselect(),
        }
    }
}

pub struct Sideatt {
    cmd: Command<AnalogueAudioPath>
}

impl Sideatt {
    impl_bits!(Command<AnalogueAudioPath>,2,6);
}
