use crate::states::{State, WildState};

pub struct StateFilterBuilder {
    filter: StateFilter,
}

impl Default for StateFilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StateFilterBuilder {
    pub fn new() -> Self {
        Self {
            filter: StateFilter {
                gender: 0,
                ability: 0,
                shiny: 0,
                skip: false,
                min: [0, 0, 0, 0, 0, 0],
                max: [31, 31, 31, 31, 31, 31],
                natures: vec![],
                powers: vec![],
                encounters: vec![],
            },
        }
    }

    pub fn gender(mut self, gender: u8) -> Self {
        self.filter.gender = gender;
        self
    }

    pub fn ability(mut self, ability: u8) -> Self {
        self.filter.ability = ability;
        self
    }

    pub fn shiny(mut self, shiny: u8) -> Self {
        self.filter.shiny = shiny;
        self
    }

    pub fn skip(mut self, skip: bool) -> Self {
        self.filter.skip = skip;
        self
    }

    pub fn min(mut self, min_ivs: [u8; 6]) -> Self {
        self.filter.min = min_ivs;
        self
    }

    pub fn max(mut self, max_ivs: [u8; 6]) -> Self {
        self.filter.max = max_ivs;
        self
    }

    pub fn natures(mut self, natures: Vec<bool>) -> Self {
        self.filter.natures = natures;
        self
    }

    pub fn powers(mut self, powers: Vec<bool>) -> Self {
        self.filter.powers = powers;
        self
    }

    pub fn encounters(mut self, encounters: Vec<bool>) -> Self {
        self.filter.encounters = encounters;
        self
    }

    pub fn build(self) -> StateFilter {
        self.filter
    }
}

#[derive(Clone)]
pub struct StateFilter {
    gender: u8,
    ability: u8,
    shiny: u8,
    skip: bool,
    min: [u8; 6],
    max: [u8; 6],
    natures: Vec<bool>,
    powers: Vec<bool>,
    encounters: Vec<bool>,
}

impl StateFilter {
    pub fn compare_state(&self, state: &dyn State) -> bool {
        self.compare_pid(state) && self.compare_ivs(state)
    }

    pub fn compare_pid(&self, state: &dyn State) -> bool {
        self.compare_shiny(state)
            && self.compare_ability(state)
            && self.compare_gender(state)
            && self.compare_nature(state)
    }

    pub fn compare_ivs(&self, state: &dyn State) -> bool {
        self.compare_hidden_power(state) && self.compare_iv(state)
    }

    pub fn compare_ability(&self, state: &dyn State) -> bool {
        self.skip || self.ability == 255 || self.ability == state.get_ability()
    }

    pub fn compare_gender(&self, state: &dyn State) -> bool {
        self.skip || self.gender == 255 || self.gender == state.get_gender()
    }

    pub fn compare_nature(&self, state: &dyn State) -> bool {
        self.skip || self.natures[state.get_nature() as usize]
    }

    pub fn compare_shiny(&self, state: &dyn State) -> bool {
        self.skip || self.shiny == 255 || ((self.shiny & state.get_shiny()) > 0)
    }

    pub fn compare_iv(&self, state: &dyn State) -> bool {
        if self.skip {
            return true;
        } else {
            for i in 0..6 {
                let iv = state.get_iv(i);

                if iv < self.min[i] || iv > self.max[i] {
                    return false;
                }
            }
        }

        true
    }

    pub fn compare_hidden_power(&self, state: &dyn State) -> bool {
        self.skip || self.powers[state.get_hidden() as usize]
    }

    pub fn compare_encounter_slot(&self, state: &dyn WildState) -> bool {
        self.skip || self.encounters[state.get_encounter_slot() as usize]
    }
}
