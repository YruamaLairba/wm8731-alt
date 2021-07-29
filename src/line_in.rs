//! Line inputs configuration
#![allow(clippy::new_without_default)]

use core::marker::PhantomData;
use crate::Command;

///Marker indicating line in concern
pub struct LineIn<CHANNEL>{
    channel:PhantomData<CHANNEL>
}

///Marker indicating left channel concern
pub struct Left; 

///Marker indicating right channel concern
pub struct Right;

///Marker indicating left line in concern
pub type LeftLineIn = LineIn<Left>;

///Marker indicating left line in concern
pub type RightLineIn = LineIn<Right>;

impl Command<LeftLineIn> {
    pub fn new() -> Self{
        Self{data: 0b0_1001_0111,t:PhantomData::<LeftLineIn>}
    }
}

impl Command<RightLineIn> {
    pub fn new() -> Self{
        Self{data: 0x1<<9|0b0_1001_0111,t:PhantomData::<RightLineIn>}
    }
}

impl <CHANNEL> Command<LineIn<CHANNEL>> {
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn left_line_in_new(){
        let cmd = Command::<LeftLineIn>::new();
        let expected =0b0000_0000_1001_0111;
        assert!(cmd.data == expected,"Got {:#b},expected {:#b}",cmd.data,expected)
    }
    #[test]
    fn right_line_in_new(){
        let cmd = Command::<RightLineIn>::new();
        let expected =0b0000_0010_1001_0111;
        assert!(cmd.data == expected,"Got {:#b},expected {:#b}",cmd.data,expected)
    }
}

