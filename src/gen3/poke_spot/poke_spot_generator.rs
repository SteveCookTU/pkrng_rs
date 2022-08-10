use crate::gen3::gamecube::GameCubeState;
use crate::generators::Generator;
use crate::rng::XDRNG;
use crate::states::State;
use crate::{StateFilter, TidSid};

pub struct PokeSpotGenerator<'a> {
    initial_advances: u32,
    max_advances: u32,
    offset: u32,
    tsv: u16,
    gender_ratio: u8,
    filter: &'a StateFilter,
}

impl Generator for PokeSpotGenerator<'_> {
    fn set_offset(&mut self, offset: u32) {
        self.offset = offset;
    }

    fn set_initial_advances(&mut self, advances: u32) {
        self.initial_advances = advances;
    }
}

impl<'a> PokeSpotGenerator<'a> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        tidsid: TidSid,
        gender_ratio: u8,
        filter: &'a StateFilter,
    ) -> Self {
        Self {
            initial_advances,
            max_advances,
            offset: 0,
            tsv: tidsid.tid ^ tidsid.sid,
            gender_ratio,
            filter,
        }
    }

    pub fn generate(&self, seed: u32, spots: &[bool]) -> Vec<GameCubeState> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut rng = XDRNG::new(seed);
        rng.advance(self.initial_advances);

        for cnt in 0..self.max_advances {
            let mut go = rng;

            if (go.next_u16() & 3) == 0 && go.next_u16() % 100 >= 10 {
                let mut state = GameCubeState::new(self.initial_advances + cnt);

                let call: u8 = (go.next_u16() % 100).try_into().unwrap();
                if call < 50 {
                    if !spots[0] {
                        continue;
                    }
                    state.set_info(0);
                } else if call < 85 {
                    if !spots[1] {
                        continue;
                    }
                    state.set_info(1);
                } else {
                    if !spots[2] {
                        continue;
                    }
                    state.set_info(2);
                }

                let high = go.next_u16();
                let low = go.next_u16();

                state.set_pid_halves(high, low);
                state.set_ability((low & 1) as u8);
                state.set_gender_with_ratio((low & 255) as u8, self.gender_ratio);
                let pid = state.get_pid();
                state.set_nature((pid % 25) as u8);
                state.set_shiny_from_comparison(self.tsv, high ^ low, 8);

                if self.filter.compare_pid(&state) {
                    states.push(state);
                }
            }

            rng.next_u32();
        }

        states
    }
}
