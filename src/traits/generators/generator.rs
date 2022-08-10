pub trait Generator {
    fn set_offset(&mut self, offset: u32);

    fn set_initial_advances(&mut self, advances: u32);
}
