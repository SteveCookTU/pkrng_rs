use crate::states::State;

pub trait EggState: State {
    fn get_inheritance(&self, index: usize) -> u8;

    fn set_inheritance(&mut self, index: usize, val: u8);
}
