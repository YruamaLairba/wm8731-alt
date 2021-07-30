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
