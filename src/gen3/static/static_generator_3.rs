use crate::generators::{Generator, StaticGenerator};
use crate::rng::PokeRNG;
use crate::states::State;
use crate::{Lead, Method, StateFilter, StaticState, TidSid};

pub struct StaticGenerator3<'a> {
    initial_advances: u32,
    max_advances: u32,
    offset: u32,
    tsv: u16,
    gender_ratio: u8,
    method: Method,
    filter: &'a StateFilter,
}

impl Generator for StaticGenerator3<'_> {
    fn set_offset(&mut self, offset: u32) {
        self.offset = offset;
    }

    fn set_initial_advances(&mut self, advances: u32) {
        self.initial_advances = advances;
    }
}

impl StaticGenerator for StaticGenerator3<'_> {
    fn set_lead(&mut self, _lead: Lead) {}

    fn set_synch_nature(&mut self, _synch_nature: u8) {}
}

impl<'a> StaticGenerator3<'a> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        tidsid: TidSid,
        gender_ratio: u8,
        method: Method,
        filter: &'a StateFilter,
    ) -> Self {
        Self {
            initial_advances,
            max_advances,
            offset: 0,
            tsv: tidsid.tid ^ tidsid.sid,
            gender_ratio,
            method,
            filter,
        }
    }

    pub fn generate(&self, seed: u32) -> Vec<StaticState> {
        match self.method {
            Method::Method1 | Method::Method2 | Method::Method4 => self.generate_method_124(seed),
            Method::Method1Reverse => self.generate_method1_reverse(seed),
            _ => vec![],
        }
    }

    fn generate_method_124(&self, seed: u32) -> Vec<StaticState> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut rng = PokeRNG::new(seed);
        rng.advance(self.initial_advances + self.offset);

        for cnt in 0..self.max_advances {
            let mut state = StaticState::new(self.initial_advances + cnt);

            let mut go = rng;

            let low = go.next_u16();
            let high = go.next_u16();

            if self.method == Method::Method2 {
                go.advance(1);
            }

            let iv1 = go.next_u16();

            if self.method == Method::Method4 {
                go.advance(1);
            }

            let iv2 = go.next_u16();

            state.set_pid_halves(high, low);
            state.set_ability((low & 1) as u8);
            state.set_gender_with_ratio((low & 255) as u8, self.gender_ratio);
            let pid = state.get_pid();
            state.set_nature((pid % 25) as u8);
            state.set_shiny_from_comparison(self.tsv, high ^ low, 8);
            state.set_iv_halves(iv1, iv2);
            state.calculate_hidden_power();

            if self.filter.compare_state(&state) {
                states.push(state);
            }

            rng.next_u32();
        }

        states
    }

    fn generate_method1_reverse(&self, seed: u32) -> Vec<StaticState> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut rng = PokeRNG::new(seed);
        rng.advance(self.initial_advances + self.offset);

        for cnt in 0..self.max_advances {
            let mut state = StaticState::new(self.initial_advances + cnt);
            let mut go = rng;

            let high = go.next_u16();
            let low = go.next_u16();
            let iv1 = go.next_u16();
            let iv2 = go.next_u16();

            state.set_pid_halves(high, low);
            state.set_ability((low & 1) as u8);
            state.set_gender_with_ratio((low & 255) as u8, self.gender_ratio);
            let pid = state.get_pid();
            state.set_nature((pid % 25) as u8);
            state.set_shiny_from_comparison(self.tsv, high ^ low, 8);

            state.set_iv_halves(iv1, iv2);
            state.calculate_hidden_power();

            if self.filter.compare_state(&state) {
                states.push(state);
            }

            rng.next_u32();
        }

        states
    }
}
