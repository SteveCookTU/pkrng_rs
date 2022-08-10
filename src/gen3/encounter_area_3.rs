use crate::{Encounter, EncounterArea, Slot};
use pkhex_rs::personal_info_g3::PersonalInfoG3;

pub struct EncounterArea3 {
    encounter: Encounter,
    location: u8,
    rate: u8,
    pokemon: Vec<Slot<PersonalInfoG3>>,
}

impl EncounterArea3 {
    pub fn new(
        encounter: Encounter,
        location: u8,
        rate: u8,
        pokemon: Vec<Slot<PersonalInfoG3>>,
    ) -> Self {
        Self {
            encounter,
            location,
            rate,
            pokemon,
        }
    }

    pub fn calc_level(&self, index: usize, prng: u16) -> u8 {
        let min_level: u16 = self.pokemon[index].get_min_level().try_into().unwrap();
        let max_level: u16 = self.pokemon[index].get_max_level().try_into().unwrap();
        ((prng % (max_level - min_level + 1)) + min_level) as u8
    }

    pub fn min_level(&self, index: usize) -> u8 {
        self.pokemon[index].get_min_level()
    }

    pub fn rse_safari_zone(&self) -> bool {
        [90, 197, 89, 186, 92, 189, 91, 188, 73, 98, 74, 20, 97, 71].contains(&self.location)
    }
}

impl EncounterArea<PersonalInfoG3> for EncounterArea3 {
    fn get_encounter(&self) -> Encounter {
        self.encounter
    }

    fn get_location(&self) -> u8 {
        self.location
    }

    fn get_rate(&self) -> u8 {
        self.rate
    }

    fn get_pokemon(&self) -> &Vec<Slot<PersonalInfoG3>> {
        &self.pokemon
    }
}
