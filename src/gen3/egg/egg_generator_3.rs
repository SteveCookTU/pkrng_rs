use crate::gen3::egg::EggState3;
use crate::generators::{EggGenerator, Generator};
use crate::rng::PokeRNG;
use crate::states::{EggState, State};
use crate::{Daycare, Method, StateFilter, TidSid};

fn set_inheritance(
    daycare: &Daycare,
    state: &mut EggState3,
    inh: &[u16],
    par: &[u16],
    broken: bool,
) {
    const ORDER: [usize; 6] = [0, 1, 2, 5, 3, 4];

    if broken {
        const AVAILABLE_1: [usize; 6] = [0, 1, 2, 3, 4, 5];
        const AVAILABLE_2: [usize; 5] = [1, 2, 3, 4, 5];
        const AVAILABLE_3: [usize; 4] = [1, 3, 4, 5];

        let stat = AVAILABLE_1[usize::from(inh[0] % 6)];
        let parent: u8 = (par[0] & 1) as u8;
        state.set_iv(
            ORDER[stat],
            daycare.get_parent_iv(parent.into(), ORDER[stat]),
        );
        state.set_inheritance(ORDER[stat], parent + 1);

        let stat = AVAILABLE_2[usize::from(inh[1] % 5)];
        let parent: u8 = (par[1] & 1) as u8;
        state.set_iv(
            ORDER[stat],
            daycare.get_parent_iv(parent.into(), ORDER[stat]),
        );
        state.set_inheritance(ORDER[stat], parent + 1);

        let stat = AVAILABLE_3[usize::from(inh[2] % 4)];
        let parent: u8 = (par[2] & 1) as u8;
        state.set_iv(
            ORDER[stat],
            daycare.get_parent_iv(parent.into(), ORDER[stat]),
        );
        state.set_inheritance(ORDER[stat], parent + 1);
    } else {
        let mut available = [0, 1, 2, 3, 4, 5];
        let avoid = |avail: &mut [usize], stat: usize, i: usize| {
            for j in stat..(5 - i) {
                avail[j] = avail[j + 1];
            }
        };

        let stat = available[usize::from(inh[0] % 6)];
        let parent: u8 = (par[0] & 1) as u8;
        state.set_iv(
            ORDER[stat],
            daycare.get_parent_iv(parent.into(), ORDER[stat]),
        );
        state.set_inheritance(ORDER[stat], parent + 1);

        avoid(&mut available, stat, 0);

        let stat = available[usize::from(inh[1] % 5)];
        let parent: u8 = (par[1] & 1) as u8;
        state.set_iv(
            ORDER[stat],
            daycare.get_parent_iv(parent.into(), ORDER[stat]),
        );
        state.set_inheritance(ORDER[stat], parent + 1);

        avoid(&mut available, stat, 1);

        let stat = available[usize::from(inh[2] % 4)];
        let parent: u8 = (par[2] & 1) as u8;
        state.set_iv(
            ORDER[stat],
            daycare.get_parent_iv(parent.into(), ORDER[stat]),
        );
        state.set_inheritance(ORDER[stat], parent + 1);
    }
}

pub struct EggGenerator3<'a, 'b> {
    initial_advances: u32,
    max_advances: u32,
    offset: u32,
    tsv: u16,
    gender_ratio: u8,
    method: Method,
    initial_advances_pickup: u32,
    max_advances_pickup: u32,
    calibration: u8,
    min_redraw: u8,
    max_redraw: u8,
    compatability: u8,
    iv1: u8,
    iv2: u8,
    inh: u8,
    daycare: &'a Daycare,
    filter: &'b StateFilter,
}

impl Generator for EggGenerator3<'_, '_> {
    fn set_offset(&mut self, offset: u32) {
        self.offset = offset;
    }

    fn set_initial_advances(&mut self, advances: u32) {
        self.initial_advances = advances;
    }
}

impl EggGenerator for EggGenerator3<'_, '_> {}

impl<'a, 'b> EggGenerator3<'a, 'b> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        tidsid: TidSid,
        gender_ratio: u8,
        method: Method,
        daycare: &'a Daycare,
        filter: &'b StateFilter,
    ) -> EggGenerator3<'a, 'b> {
        let (iv1, iv2, inh) = match method {
            Method::EBred => (0, 0, 1),
            Method::EBredSplit => (0, 1, 1),
            Method::EBredAlternate => (0, 0, 2),
            Method::RSFRLGBred => (1, 0, 1),
            Method::RSFRLGBredSplit => (0, 1, 1),
            Method::RSFRLGBredAlternate => (1, 0, 2),
            Method::RSFRLGBredMixed => (0, 0, 2),
            _ => (0, 0, 0),
        };

        Self {
            initial_advances,
            max_advances,
            offset: 0,
            tsv: tidsid.tid ^ tidsid.sid,
            gender_ratio,
            method,
            initial_advances_pickup: 0,
            max_advances_pickup: 0,
            calibration: 0,
            min_redraw: 0,
            max_redraw: 0,
            compatability: 0,
            iv1,
            iv2,
            inh,
            daycare,
            filter,
        }
    }

    pub fn set_initial_advances_pickup(&mut self, value: u32) {
        self.initial_advances_pickup = value;
    }

    pub fn set_max_advances_pickup(&mut self, value: u32) {
        self.max_advances_pickup = value;
    }

    pub fn set_calibration(&mut self, value: u8) {
        self.calibration = value;
    }

    pub fn set_min_redraw(&mut self, value: u8) {
        self.min_redraw = value;
    }

    pub fn set_max_redraw(&mut self, value: u8) {
        self.max_redraw = value;
    }

    pub fn set_compatability(&mut self, value: u8) {
        self.compatability = value;
    }

    pub fn generate(&self, seed: u32, seed2: u32) -> Vec<EggState3> {
        match self.method {
            Method::EBredPID => self.generate_emerald_pid(),
            Method::EBred | Method::EBredSplit | Method::EBredAlternate => {
                self.generate_emerald_ivs()
            }
            Method::RSFRLGBredSplit
            | Method::RSFRLGBred
            | Method::RSFRLGBredAlternate
            | Method::RSFRLGBredMixed => {
                let lower = self.generate_lower(seed);
                if lower.is_empty() {
                    vec![]
                } else {
                    self.generate_upper(seed2, &lower)
                }
            }
            _ => vec![],
        }
    }

    fn generate_emerald_pid(&self) -> Vec<EggState3> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut everstone = false;
        let mut parent = 0;

        for i in 0..2 {
            if self.daycare.get_parent_gender(i) == 1 && self.daycare.get_parent_item(i) == 1 {
                parent = i;
                everstone = true;
            }
        }

        for i in 0..2 {
            if self.daycare.get_parent_gender(i) == 3 && self.daycare.get_parent_item(i) == 1 {
                parent = i;
                everstone = true;
            }
        }

        let mut rng = PokeRNG::new(0);
        rng.advance(self.initial_advances);

        let mut val = self.initial_advances + 1;
        let mut cnt = 0;
        while cnt <= self.max_advances {
            let mut comp = rng;

            if (comp.next_u16().wrapping_mul(100) / 0xFFFF) < self.compatability.into() {
                for redraw in self.min_redraw..=self.max_redraw {
                    let mut go = comp;

                    let offset = (self.calibration as u32) + 3 * (redraw as u32);
                    let mut state = EggState3::new(cnt + self.initial_advances - offset);

                    let flag = if everstone {
                        go.next_u16() >> 15 == 0
                    } else {
                        false
                    };

                    let mut trng = PokeRNG::new((val - offset) & 0xFFFF);

                    let mut pid;
                    if !flag {
                        pid =
                            (((go.next_u16() % 0xFFFE) + 1) as u32) | (trng.next_u32() & 0xFFF0000);
                        state.set_nature((pid & 25) as u8);
                    } else {
                        let mut i = 2;
                        while {
                            pid = (go.next_u16() as u32) | (trng.next_u32() & 0xFFF0000);
                            i += 1;
                            pid % 25 != self.daycare.get_parent_nature(parent) as u32
                        } {
                            if i == 19 {
                                break;
                            }
                        }
                        if i == 19 {
                            continue;
                        }

                        state.set_nature(self.daycare.get_parent_nature(parent));
                    }

                    state.set_pid(pid);
                    state.set_ability((pid & 1) as u8);
                    state.set_gender_with_ratio((pid & 255) as u8, self.gender_ratio);
                    state.set_shiny_from_comparison(
                        self.tsv,
                        ((pid >> 16) ^ (pid & 0xFFFF)) as u16,
                        8,
                    );

                    if self.filter.compare_pid(&state) {
                        state.set_redraw(redraw);
                        states.push(state);
                    }
                }
            }

            cnt += 1;
            val += 1;
            rng.next_u32();
        }

        states.sort_by_key(|s| s.get_advances());

        states
    }

    fn generate_emerald_ivs(&self) -> Vec<EggState3> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut rng = PokeRNG::new(0);
        rng.advance(self.initial_advances);

        for cnt in 0..=self.max_advances {
            let mut state = EggState3::new(cnt + self.initial_advances);

            let mut go = rng;
            go.advance(self.iv1 as u32);
            let iv1 = go.next_u16();
            go.advance(self.iv2 as u32);
            let iv2 = go.next_u16();
            state.set_iv_halves(iv1, iv2);

            go.advance(self.inh as u32);
            let inh1 = go.next_u16();
            let inh2 = go.next_u16();
            let inh3 = go.next_u16();
            let inh = [inh1, inh2, inh3];

            let par1 = go.next_u16();
            let par2 = go.next_u16();
            let par3 = go.next_u16();
            let par = [par1, par2, par3];

            set_inheritance(self.daycare, &mut state, &inh, &par, true);
            state.calculate_hidden_power();

            if self.filter.compare_ivs(&state) {
                states.push(state);
            }

            rng.next_u32();
        }

        states
    }

    fn generate_lower(&self, seed: u32) -> Vec<(u32, u16)> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut rng = PokeRNG::new(seed);
        rng.advance(self.initial_advances);

        for cnt in 0..=self.max_advances {
            let mut go = rng;
            if (go.next_u16().wrapping_mul(100) / 0xFFFE) < (self.compatability as u16) {
                let pid = (go.next_u16() % 0xFFFE) + 1;
                states.push((cnt + self.initial_advances, pid));
            }
        }

        states
    }

    fn generate_upper(&self, seed: u32, lower: &[(u32, u16)]) -> Vec<EggState3> {
        let mut upper = Vec::with_capacity(self.max_advances_pickup as usize);

        let mut rng = PokeRNG::new(seed);

        for cnt in 0..=self.max_advances_pickup {
            let mut state = EggState3::new(cnt + self.initial_advances_pickup);
            let mut go = rng;

            state.set_pid(go.next_u16() as u32);

            let mut go = rng;
            go.advance(self.iv1 as u32);
            let iv1 = go.next_u16();
            go.advance(self.iv2 as u32);
            let iv2 = go.next_u16();
            state.set_iv_halves(iv1, iv2);

            go.advance(self.inh as u32);
            let inh1 = go.next_u16();
            let inh2 = go.next_u16();
            let inh3 = go.next_u16();
            let inh = [inh1, inh2, inh3];

            let par1 = go.next_u16();
            let par2 = go.next_u16();
            let par3 = go.next_u16();
            let par = [par1, par2, par3];

            set_inheritance(self.daycare, &mut state, &inh, &par, false);
            state.calculate_hidden_power();

            if self.filter.compare_ivs(&state) {
                upper.push(state);
            }

            rng.next_u32();
        }

        let mut states = Vec::with_capacity(lower.len() * upper.len());

        for &low in lower {
            for up in upper.iter_mut() {
                up.set_pid_halves(up.get_pid() as u16, low.1);
                up.set_ability((low.1 & 1) as u8);
                up.set_gender_with_ratio((low.1 & 255) as u8, self.gender_ratio);
                up.set_nature((up.get_pid() % 25) as u8);
                up.set_shiny_from_comparison(
                    self.tsv,
                    ((up.get_pid() >> 16) ^ (up.get_pid() & 0xFFFF)) as u16,
                    8,
                );

                if self.filter.compare_pid(up) {
                    up.set_generate_advance(low.0);
                    states.push(*up);
                }
            }
        }

        states
    }
}
