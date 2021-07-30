///This macro is the template for raw bits write to a field
macro_rules! impl_bits {
    ($ret:ty, $lenght:literal, $shift:literal) => {
        #[must_use]
        pub fn bits(mut self, value: u8) -> $ret {
            let mask = !((!0) << $lenght) << $shift;
            self.cmd.data = self.cmd.data & !mask | (value as u16) & mask;
            self.cmd
        }
    };
}

///Template for raw bit write to one bit field
macro_rules! impl_bit {
    ($ret:ty, $pos:literal) => {
        #[must_use]
        pub fn bit(mut self, value: u8) -> $ret {
            let mask = 0b1 << $pos;
            self.cmd.data = self.cmd.data & !mask | (value as u16) & mask;
            self.cmd
        }
    };
}

///Template for setting a bit into a one bit field
macro_rules! impl_set_bit {
    ($ret:ty, $pos:literal) => {
        #[must_use]
        pub fn set_bit(mut self) -> $ret {
            self.cmd.data |= 0b1 << $pos;
            self.cmd
        }
    };
}

///Template for enabling a bit into a one bit field
macro_rules! impl_enable {
    ($ret:ty, $pos:literal) => {
        #[must_use]
        pub fn enable(mut self) -> $ret {
            self.cmd.data |= 0b1 << $pos;
            self.cmd
        }
    };
}

///Template for clearing a bit into a one bit field
macro_rules! impl_clear_bit {
    ($ret:ty, $pos:literal) => {
        #[must_use]
        pub fn clear_bit(mut self) -> $ret {
            self.cmd.data &= !(0b1 << $pos);
            self.cmd
        }
    };
}

///Template for disabling a bit into a one bit field
macro_rules! impl_disable {
    ($ret:ty, $pos:literal) => {
        #[must_use]
        pub fn disable(mut self) -> $ret {
            self.cmd.data &= !(0b1 << $pos);
            self.cmd
        }
    };
}
macro_rules! impl_bitsetters {
    ($ret:ty, $pos:literal) => {
        impl_bit!($ret, $pos);
        impl_set_bit!($ret, $pos);
        impl_clear_bit!($ret, $pos);
        impl_enable!($ret, $pos);
        impl_disable!($ret, $pos);
    };
}

