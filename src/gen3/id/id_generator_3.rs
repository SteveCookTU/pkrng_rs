use crate::gen3::id::IDState3;
use crate::generators::IDGenerator;
use crate::rng::{PokeRNG, XDRNG};
use crate::IDFilter;

pub struct IDGenerator3<'a> {
    initial_advances: u32,
    max_advances: u32,
    filter: &'a IDFilter,
}

impl IDGenerator for IDGenerator3<'_> {}

impl<'a> IDGenerator3<'a> {
    pub fn new(initial_advances: u32, max_advances: u32, filter: &'a IDFilter) -> Self {
        Self {
            initial_advances,
            max_advances,
            filter,
        }
    }

    pub fn generate_xd_colo(&self, seed: u32) -> Vec<IDState3> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut rng = XDRNG::new(seed);
        rng.advance(self.initial_advances);

        for cnt in 0..self.max_advances {
            let mut go = rng;

            let tid = go.next_u16();
            let sid = go.next_u16();

            let state = IDState3::new(self.initial_advances + cnt, tid, sid);

            if self.filter.compare(&state) {
                states.push(state);
            }

            rng.next_u32();
        }

        states
    }

    pub fn generate_frlge(&self, tid: u16) -> Vec<IDState3> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut rng = PokeRNG::new(tid as u32);
        rng.advance(self.initial_advances);

        for cnt in 0..self.max_advances {
            let sid = rng.next_u16();

            let state = IDState3::new(self.initial_advances + cnt, tid, sid);

            if self.filter.compare(&state) {
                states.push(state);
            }
        }

        states
    }

    pub fn generate_rs(&self, seed: u32) -> Vec<IDState3> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut rng = PokeRNG::new(seed);
        rng.advance(self.initial_advances);

        for cnt in 0..self.max_advances {
            let mut go = rng;

            let sid = go.next_u16();
            let tid = go.next_u16();

            let state = IDState3::new(self.initial_advances + cnt, tid, sid);

            if self.filter.compare(&state) {
                states.push(state);
            }

            rng.next_u32();
        }

        states
    }
}
