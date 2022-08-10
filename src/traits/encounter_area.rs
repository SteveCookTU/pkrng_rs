use crate::{Encounter, Slot};
use pkhex_rs::PersonalInfo;
use std::collections::HashSet;

pub trait EncounterArea<T: PersonalInfo + 'static> {
    fn get_encounter(&self) -> Encounter;

    fn get_location(&self) -> u8;

    fn get_rate(&self) -> u8;

    fn get_pokemon(&self) -> &Vec<Slot<T>>;

    fn get_unique_species(&self) -> Vec<u16> {
        let pokemon = self.get_pokemon();

        let mut nums = HashSet::with_capacity(pokemon.len());

        for p in pokemon {
            nums.insert(p.get_species());
        }

        nums.into_iter().collect()
    }

    fn get_slots(&self, num: u16) -> Vec<bool> {
        let pokemon = self.get_pokemon();

        pokemon.iter().map(|s| s.get_species() == num).collect()
    }

    fn get_level_range(&self, species: u16) -> (u8, u8) {
        let mut range = (0, 100);
        for p in self.get_pokemon() {
            if p.get_species() == species {
                range.0 = range.0.max(p.get_min_level());
                range.1 = range.1.min(p.get_max_level());
            }
        }
        range
    }
}
