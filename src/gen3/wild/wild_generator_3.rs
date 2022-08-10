use crate::gen3::EncounterArea3;
use crate::generators::{Generator, WildGenerator};
use crate::rng::PokeRNG;
use crate::states::{State, WildState as WildStateT};
use crate::util::encounter_slot;
use crate::{Encounter, EncounterArea, Game, Lead, Method, StateFilter, TidSid, WildState};

pub struct WildGenerator3<'a> {
    initial_advances: u32,
    max_advances: u32,
    offset: u32,
    tsv: u16,
    gender_ratio: u8,
    method: Method,
    filter: &'a StateFilter,
    version: Game,
    encounter: Encounter,
    lead: Lead,
    synch_nature: u8,
}

impl Generator for WildGenerator3<'_> {
    fn set_offset(&mut self, offset: u32) {
        self.offset = offset;
    }

    fn set_initial_advances(&mut self, advances: u32) {
        self.initial_advances = advances;
    }
}

impl WildGenerator for WildGenerator3<'_> {
    fn set_encounter(&mut self, encounter: Encounter) {
        self.encounter = encounter;
    }

    fn set_lead(&mut self, lead: Lead) {
        self.lead = lead;
    }

    fn set_synch_nature(&mut self, synch_nature: u8) {
        self.synch_nature = synch_nature;
    }
}

impl<'a> WildGenerator3<'a> {
    pub fn new(
        initial_advances: u32,
        max_advances: u32,
        tidsid: TidSid,
        gender_ratio: u8,
        method: Method,
        filter: &'a StateFilter,
        version: Game,
    ) -> Self {
        Self {
            initial_advances,
            max_advances,
            offset: 0,
            tsv: tidsid.tid ^ tidsid.sid,
            gender_ratio,
            method,
            filter,
            encounter: Encounter::Grass,
            version,
            lead: Lead::None,
            synch_nature: 0,
        }
    }

    pub fn generate(&self, seed: u32, encounter_area: &EncounterArea3) -> Vec<WildState> {
        let mut states = Vec::with_capacity(self.max_advances as usize);

        let mut rng = PokeRNG::new(seed);
        rng.advance(self.initial_advances + self.offset);

        let rate = encounter_area.get_rate() as u16 * 16;

        let rse_safari = encounter_area.rse_safari_zone() && (self.version & Game::RSE) != 0;
        let rock = (self.version & Game::FRLG) != 0;

        let mut cute_charm_flag = false;
        let cute_charm = match self.lead {
            Lead::CuteCharm125F => |pid: u32| -> bool { (pid & 0xFF) < 31 },
            Lead::CuteCharm875M => |pid: u32| -> bool { (pid & 0xFF) >= 31 },
            Lead::CuteCharm25F => |pid: u32| -> bool { (pid & 0xFF) < 63 },
            Lead::CuteCharm75M => |pid: u32| -> bool { (pid & 0xFF) >= 63 },
            Lead::CuteCharm50F => |pid: u32| -> bool { (pid & 0xFF) < 127 },
            Lead::CuteCharm50M => |pid: u32| -> bool { (pid & 0xFF) >= 127 },
            Lead::CuteCharm75F => |pid: u32| -> bool { (pid & 0xFF) < 191 },
            Lead::CuteCharm25M => |pid: u32| -> bool { (pid & 0xFF) >= 191 },
            _ => |_pid: u32| -> bool { true },
        };

        for cnt in 0..self.max_advances {
            let mut state = WildState::new(self.initial_advances + cnt);
            let mut go = rng;

            match self.encounter {
                Encounter::RockSmash => {
                    if rse_safari || rock {
                        go.next_u32();
                    }
                    if (go.next_u16() % 2880) >= rate {
                        continue;
                    }

                    let encounter_slot = encounter_slot::h_slot(go.next_u16(), self.encounter);

                    state.set_encounter_slot(encounter_slot);

                    if !self.filter.compare_encounter_slot(&state) {
                        continue;
                    }

                    state
                        .set_level(encounter_area.calc_level(encounter_slot.into(), go.next_u16()));
                    if rse_safari {
                        go.advance(1);
                    }
                }
                Encounter::Grass => {
                    go.next_u32();

                    let encounter_slot = encounter_slot::h_slot(go.next_u16(), self.encounter);
                    state.set_encounter_slot(encounter_slot);

                    if !self.filter.compare_encounter_slot(&state) {
                        continue;
                    }

                    state.set_level(encounter_area.min_level(encounter_slot.into()));
                    go.advance(if rse_safari { 2 } else { 1 });
                }
                Encounter::Surfing
                | Encounter::OldRod
                | Encounter::GoodRod
                | Encounter::SuperRod => {
                    if !rse_safari {
                        go.next_u32();
                    }

                    let encounter_slot = encounter_slot::h_slot(go.next_u16(), self.encounter);
                    state.set_encounter_slot(encounter_slot);

                    if !self.filter.compare_encounter_slot(&state) {
                        continue;
                    }

                    state
                        .set_level(encounter_area.calc_level(encounter_slot.into(), go.next_u16()));
                    if rse_safari {
                        go.next_u32();
                    }
                }
                _ => {}
            }

            if self.lead == Lead::None {
                state.set_nature((go.next_u16() % 25) as u8);
            } else if self.lead == Lead::Synchronize {
                if (go.next_u16() & 1) == 0 {
                    state.set_nature(self.synch_nature);
                } else {
                    state.set_nature((go.next_u16() % 25) as u8);
                }
            } else {
                cute_charm_flag = go.next_u16() % 3 > 0;
                state.set_nature((go.next_u16() % 25) as u8);
            }

            if !self.filter.compare_nature(&state) {
                continue;
            }

            let mut pid;

            while {
                let low = go.next_u16();
                let high = go.next_u16();
                pid = ((high as u32) << 16) | (low as u32);
                ((pid % 25) as u8) != state.get_nature() || (cute_charm_flag && !cute_charm(pid))
            } {}

            state.set_pid(pid);
            state.set_ability((pid & 1) as u8);
            state.set_gender_with_ratio((pid & 255) as u8, self.gender_ratio);
            state.set_shiny_from_comparison(
                self.tsv,
                ((pid & 0xFFFF) as u16) ^ ((pid >> 16) as u16),
                8,
            );

            let iv1;
            let iv2;

            match self.method {
                Method::MethodH1 => {
                    iv1 = go.next_u16();
                    iv2 = go.next_u16();
                }
                Method::MethodH2 => {
                    go.next_u32();
                    iv1 = go.next_u16();
                    iv2 = go.next_u16();
                }
                _ => {
                    iv1 = go.next_u16();
                    go.next_u32();
                    iv2 = go.next_u16();
                }
            }

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
