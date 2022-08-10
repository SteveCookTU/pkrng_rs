use crate::states::{State, WildState as WildStateT};
use crate::{Encounter, Lead};

#[derive(Default, Copy, Clone)]
pub struct WildState {
    seed: u32,
    advances: u32,
    pid: u32,
    ivs: [u8; 6],
    ability: u8,
    gender: u8,
    nature: u8,
    hidden: u8,
    power: u8,
    shiny: u8,
    lead: Lead,
    encounter_slot: u8,
    encounter: Encounter,
    item: u8,
}

impl State for WildState {
    fn get_seed(&self) -> u32 {
        self.seed
    }

    fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
    }

    fn get_advances(&self) -> u32 {
        self.advances
    }

    fn set_advances(&mut self, advances: u32) {
        self.advances = advances;
    }

    fn get_pid(&self) -> u32 {
        self.pid
    }

    fn set_pid(&mut self, pid: u32) {
        self.pid = pid;
    }

    fn get_iv(&self, index: usize) -> u8 {
        self.ivs[index]
    }

    fn set_iv(&mut self, index: usize, iv: u8) {
        self.ivs[index] = iv;
    }

    fn set_ivs_individual(&mut self, hp: u8, atk: u8, def: u8, spa: u8, spd: u8, spe: u8) {
        self.ivs = [hp, atk, def, spa, spd, spe];
    }

    fn get_ability(&self) -> u8 {
        self.ability
    }

    fn set_ability(&mut self, ability: u8) {
        self.ability = ability;
    }

    fn get_gender(&self) -> u8 {
        self.gender
    }

    fn set_gender(&mut self, gender: u8) {
        self.gender = gender;
    }

    fn get_hidden(&self) -> u8 {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: u8) {
        self.hidden = hidden;
    }

    fn get_power(&self) -> u8 {
        self.power
    }

    fn set_power(&mut self, power: u8) {
        self.power = power;
    }

    fn get_nature(&self) -> u8 {
        self.nature
    }

    fn set_nature(&mut self, nature: u8) {
        self.nature = nature;
    }

    fn get_shiny(&self) -> u8 {
        self.shiny
    }

    fn set_shiny(&mut self, shiny: u8) {
        self.shiny = shiny;
    }
}

impl WildStateT for WildState {
    fn get_lead(&self) -> Lead {
        self.lead
    }

    fn set_lead(&mut self, lead: Lead) {
        self.lead = lead;
    }

    fn get_encounter_slot(&self) -> u8 {
        self.encounter_slot
    }

    fn set_encounter_slot(&mut self, encounter_slot: u8) {
        self.encounter_slot = encounter_slot;
    }

    fn get_encounter(&self) -> Encounter {
        self.encounter
    }

    fn set_encounter(&mut self, encounter: Encounter) {
        self.encounter = encounter;
    }

    fn get_item(&self) -> u8 {
        self.item
    }

    fn set_item(&mut self, item: u8) {
        self.item = item;
    }
}

impl WildState {
    pub fn new(advances: u32) -> Self {
        Self {
            advances,
            ..Default::default()
        }
    }
}
