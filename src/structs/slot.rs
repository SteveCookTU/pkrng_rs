use pkhex_rs::PersonalInfo;

pub struct Slot<T: PersonalInfo + 'static> {
    min_level: u8,
    max_level: u8,
    species: u16,
    info: &'static T,
}

impl<T: PersonalInfo + 'static> Slot<T> {
    pub fn new(min_level: u8, max_level: u8, species: u16, info: &'static T) -> Self {
        Self {
            min_level,
            max_level,
            species,
            info,
        }
    }

    pub fn get_min_level(&self) -> u8 {
        self.min_level
    }

    pub fn get_max_level(&self) -> u8 {
        self.max_level
    }

    pub fn get_species(&self) -> u16 {
        self.species
    }

    pub fn get_info(&self) -> &'static T {
        self.info
    }

    pub fn set_species(&mut self, species: u16, personal_info: &'static T) {
        self.species = species;
        self.info = personal_info;
    }
}
