use crate::gen3::gamecube::GameCubeState;
use crate::gen3::{load_shadow_teams, ShadowTeam};
use crate::generators::Generator;
use crate::rng::XDRNG;
use crate::states::State;
use crate::{Method, ShadowType, StateFilter, TidSid};

pub struct GameCubeGenerator<'a> {
    initial_advances: u32,
    max_advances: u32,
    offset: u32,
    tsv: u16,
    gender_ratio: u8,
    method: Method,
    filter: &'a StateFilter,
    team: ShadowTeam,
    shiny_type: u8,
}

impl Generator for GameCubeGenerator<'_> {
    fn set_offset(&mut self, offset: u32) {
        self.offset = offset;
    }

    fn set_initial_advances(&mut self, advances: u32) {
        self.initial_advances = advances;
    }
}

impl<'a> GameCubeGenerator<'a> {
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
            team: Default::default(),
            shiny_type: 0,
        }
    }

    pub fn generate(&self, seed: u32) -> Vec<GameCubeState> {
        match self.method {
            Method::XDColo => self.generate_xd_colo(seed),
            Method::XD => self.generate_xd_shadow(seed),
            Method::Colo => self.generate_colo_shadow(seed),
            Method::Channel => self.generate_channel(seed),
            _ => vec![],
        }
    }

    pub fn set_shadow_team(&mut self, index: usize, shadow_type: u8) {
        self.team = load_shadow_teams(self.method)[index].clone();
        self.shiny_type = shadow_type;
    }

    fn generate_xd_colo(&self, seed: u32) -> Vec<GameCubeState> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut rng = XDRNG::new(seed);
        rng.advance(self.initial_advances + self.offset);

        for cnt in 0..=self.max_advances {
            let mut state = GameCubeState::new(self.initial_advances + cnt);
            let mut go = rng;

            let iv1 = go.next_u16();
            let iv2 = go.next_u16();
            let ability = (go.next_u16() & 1) as u8;
            let high = go.next_u16();
            let low = go.next_u16();

            state.set_pid_halves(high, low);
            state.set_ability(ability);
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

    fn generate_xd_shadow(&self, seed: u32) -> Vec<GameCubeState> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut rng = XDRNG::new(seed);
        rng.advance(self.initial_advances + self.offset);

        let locks = self.team.get_locks();

        for cnt in 0..=self.max_advances {
            let mut state = GameCubeState::new(self.initial_advances + cnt);

            let mut go = rng;

            go.advance(2);

            'outer: for lock in locks {
                go.advance(5);

                if !lock.get_free() {
                    let mut pid;
                    while {
                        let high = go.next_u16();
                        let low = go.next_u16();
                        pid = ((high as u32) << 16) | (low as u32);
                        if (high ^ low ^ self.tsv) < 8 {
                            continue 'outer;
                        } else {
                            !lock.compare(pid)
                        }
                    } {}
                }
            }

            if self.team.get_type() == ShadowType::SecondShadow
                || self.team.get_type() == ShadowType::Salamence
            {
                go.advance(5);

                if self.shiny_type == 1 {
                    let mut psv = go.next_u16() ^ go.next_u16();
                    while (psv ^ self.tsv) < 8 {
                        psv = go.next_u16() ^ go.next_u16();
                    }
                }
            }

            go.advance(2);

            let iv1 = go.next_u16();
            let iv2 = go.next_u16();
            state.set_iv_halves(iv1, iv2);
            state.calculate_hidden_power();

            state.set_ability((go.next_u16() & 1) as u8);

            let mut high = go.next_u16();
            let mut low = go.next_u16();

            while (high ^ low ^ self.tsv) < 8 {
                high = go.next_u16();
                low = go.next_u16();
            }

            state.set_pid_halves(high, low);
            state.set_gender_with_ratio((low & 255) as u8, self.gender_ratio);
            let pid = state.get_pid();
            state.set_nature((pid % 25) as u8);
            state.set_shiny(0);

            if self.filter.compare_state(&state) {
                states.push(state);
            }
        }

        states
    }

    fn generate_colo_shadow(&self, seed: u32) -> Vec<GameCubeState> {
        let mut states = Vec::with_capacity(self.initial_advances as usize);

        let mut rng = XDRNG::new(seed);
        rng.advance(self.initial_advances + self.offset);

        let locks = self.team.get_locks();

        for cnt in 0..=self.max_advances {
            let mut state = GameCubeState::new(self.initial_advances + cnt);

            let mut go = rng;

            let trainer_tsv = go.next_u16() ^ go.next_u16();

            let mut ability = 0;
            let mut pid = 0;

            'outer: for lock in locks {
                go.advance(4);

                ability = (go.next_u16() & 1) as u8;
                while {
                    let high = go.next_u16();
                    let low = go.next_u16();
                    pid = ((high as u32) << 16) | (low as u32);
                    if (high ^ low ^ trainer_tsv) < 8 {
                        continue 'outer;
                    } else {
                        !lock.compare(pid)
                    }
                } {}
            }

            if self.team.get_type() == ShadowType::EReader {
                state.set_iv_full(0);
                state.calculate_hidden_power();
                state.set_pid(pid);
                state.set_ability(ability);
                state.set_gender_with_ratio((pid & 255) as u8, self.gender_ratio);
                state.set_nature((pid % 25) as u8);
                state.set_shiny_from_comparison(self.tsv, ((pid >> 16) ^ (pid & 0xFFFF)) as u16, 8);
            } else {
                go.advance(2);

                let iv1 = go.next_u16();
                let iv2 = go.next_u16();
                state.set_iv_halves(iv1, iv2);
                state.calculate_hidden_power();

                state.set_ability((go.next_u16() & 1) as u8);

                let mut high = go.next_u16();
                let mut low = go.next_u16();
                while (high ^ low ^ trainer_tsv) < 8 {
                    high = go.next_u16();
                    low = go.next_u16();
                }

                state.set_pid_halves(high, low);
                state.set_gender_with_ratio((low & 255) as u8, self.gender_ratio);
                let pid = state.get_pid();
                state.set_nature((pid % 25) as u8);
                state.set_shiny_from_comparison(self.tsv, high ^ low, 8);
            }

            if self.filter.compare_state(&state) {
                states.push(state);
            }

            rng.next_u32();
        }

        states
    }

    fn generate_channel(&self, seed: u32) -> Vec<GameCubeState> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut rng = XDRNG::new(seed);
        rng.advance(self.initial_advances + self.offset);

        for cnt in 0..self.max_advances {
            let mut state = GameCubeState::new(self.initial_advances + cnt);
            let mut go = rng;

            let mut mask = 0;
            while mask & 14 != 14 {
                mask |= 1 << (go.next_u32() >> 30);
            }

            const THRESHOLDS: [u16; 2] = [0x4000, 0x547A];
            go.advance(4);
            let mut flag = false;
            for thresh in THRESHOLDS {
                if go.next_u16() <= { thresh } {
                    flag = true;
                    break;
                }
            }

            go.advance(if flag { 1 } else { 2 });

            const TID: u16 = 40122;

            let sid = go.next_u16();
            let mut high = go.next_u16();
            let low = go.next_u16();
            go.advance(3);

            if (TID ^ sid ^ high ^ low) < 8 {
                high ^= 0x8000;
            }

            state.set_pid_halves(high, low);
            state.set_ability((low & 1) as u8);
            state.set_gender_with_ratio((low & 255) as u8, self.gender_ratio);
            let pid = state.get_pid();
            state.set_nature((pid % 25) as u8);
            state.set_shiny_from_comparison(TID ^ sid, high ^ low, 8);

            let hp = (go.next_u16() >> 11).try_into().unwrap();
            let atk = (go.next_u16() >> 11).try_into().unwrap();
            let def = (go.next_u16() >> 11).try_into().unwrap();
            let spe = (go.next_u16() >> 11).try_into().unwrap();
            let spa = (go.next_u16() >> 11).try_into().unwrap();
            let spd = (go.next_u16() >> 11).try_into().unwrap();

            state.set_ivs_individual(hp, atk, def, spa, spd, spe);
            state.calculate_hidden_power();

            if self.filter.compare_state(&state) {
                states.push(state);
            }
        }

        states
    }
}
