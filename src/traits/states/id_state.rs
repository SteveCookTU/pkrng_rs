pub trait IDState {
    fn get_advances(&self) -> u32;

    fn get_tid(&self) -> u16;

    fn get_sid(&self) -> u16;

    fn get_tsv(&self) -> u16;
}
