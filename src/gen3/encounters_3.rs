use crate::gen3::EncounterArea3;
use crate::{Encounter, Game, Slot};
use pkhex_rs::personal_info_g3::PersonalInfoG3;
use pkhex_rs::personal_table;
use pkhex_rs::personal_table::PersonalTable;

const EMERALD: &[u8] = include_bytes!("../../resources/encounters/gen3/emerald.bin");
const FIRE_RED: &[u8] = include_bytes!("../../resources/encounters/gen3/firered.bin");
const LEAF_GREEN: &[u8] = include_bytes!("../../resources/encounters/gen3/leafgreen.bin");
const RUBY: &[u8] = include_bytes!("../../resources/encounters/gen3/ruby.bin");
const SAPPHIRE: &[u8] = include_bytes!("../../resources/encounters/gen3/sapphire.bin");

pub fn get_encounters(encounter: Encounter, game: Game) -> Vec<EncounterArea3> {
    let (data, info): (_, &PersonalTable<PersonalInfoG3>) = match game {
        Game::Emerald => (EMERALD, &personal_table::E),
        Game::FireRed => (FIRE_RED, &personal_table::FR),
        Game::LeafGreen => (LEAF_GREEN, &personal_table::LG),
        Game::Ruby => (RUBY, &personal_table::RS),
        _ => (SAPPHIRE, &personal_table::RS),
    };

    let mut encounters = Vec::new();

    for offset in (0..data.len()).step_by(121) {
        let entry = &data[offset..];
        let location = entry[0];
        let grass = entry[1];
        let water = entry[2];
        let rock = entry[3];
        let fish = entry[4];

        match encounter {
            Encounter::Grass => {
                if grass != 0 {
                    let mut slots = Vec::with_capacity(12);
                    for i in 0..12 {
                        let level = entry[5 + (i * 3)];
                        let index = 6 + (i * 3);
                        let species =
                            u16::from_le_bytes(entry[index..(index + 1)].try_into().unwrap());
                        slots.push(Slot::new(
                            level,
                            level,
                            species,
                            info.get_form_entry(species.into(), 0),
                        ));
                    }
                    encounters.push(EncounterArea3::new(encounter, location, grass, slots));
                }
            }
            Encounter::Surfing => {
                if water != 0 {
                    let mut slots = Vec::with_capacity(5);
                    for i in 0..5 {
                        let min = entry[41 + (i * 4)];
                        let max = entry[42 + (i * 4)];
                        let index = 43 + (i * 4);
                        let species =
                            u16::from_le_bytes(entry[index..(index + 1)].try_into().unwrap());
                        slots.push(Slot::new(
                            min,
                            max,
                            species,
                            info.get_form_entry(species.into(), 0),
                        ));
                    }
                    encounters.push(EncounterArea3::new(encounter, location, water, slots));
                }
            }
            Encounter::RockSmash => {
                if rock != 0 {
                    let mut slots = Vec::with_capacity(5);
                    for i in 0..5 {
                        let min = entry[61 + (i * 4)];
                        let max = entry[62 + (i * 4)];
                        let index = 63 + (i * 4);
                        let species =
                            u16::from_le_bytes(entry[index..(index + 1)].try_into().unwrap());
                        slots.push(Slot::new(
                            min,
                            max,
                            species,
                            info.get_form_entry(species.into(), 0),
                        ));
                    }
                    encounters.push(EncounterArea3::new(encounter, location, rock, slots));
                }
            }
            Encounter::OldRod => {
                if fish != 0 {
                    let mut slots = Vec::with_capacity(2);
                    for i in 0..2 {
                        let min = entry[81 + (i * 4)];
                        let max = entry[82 + (i * 4)];
                        let index = 83 + (i * 4);
                        let species =
                            u16::from_le_bytes(entry[index..(index + 1)].try_into().unwrap());
                        slots.push(Slot::new(
                            min,
                            max,
                            species,
                            info.get_form_entry(species.into(), 0),
                        ));
                    }
                    encounters.push(EncounterArea3::new(encounter, location, rock, slots));
                }
            }
            Encounter::GoodRod => {
                if fish != 0 {
                    let mut slots = Vec::with_capacity(3);
                    for i in 0..3 {
                        let min = entry[89 + (i * 4)];
                        let max = entry[90 + (i * 4)];
                        let index = 91 + (i * 4);
                        let species =
                            u16::from_le_bytes(entry[index..(index + 1)].try_into().unwrap());
                        slots.push(Slot::new(
                            min,
                            max,
                            species,
                            info.get_form_entry(species.into(), 0),
                        ));
                    }
                    encounters.push(EncounterArea3::new(encounter, location, rock, slots));
                }
            }
            Encounter::SuperRod => {
                if fish != 0 {
                    let mut slots = Vec::with_capacity(5);
                    for i in 0..5 {
                        let min = entry[101 + (i * 4)];
                        let max = entry[102 + (i * 4)];
                        let index = 103 + (i * 4);
                        let species =
                            u16::from_le_bytes(entry[index..(index + 1)].try_into().unwrap());
                        slots.push(Slot::new(
                            min,
                            max,
                            species,
                            info.get_form_entry(species.into(), 0),
                        ));
                    }
                    encounters.push(EncounterArea3::new(encounter, location, rock, slots));
                }
            }
            _ => {}
        }
    }

    encounters
}
