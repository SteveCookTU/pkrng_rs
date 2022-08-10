#[derive(Default, Clone, Copy)]
pub struct Daycare {
    parent_ivs: [[u8; 6]; 2],
    parent_ability: [u8; 2],
    parent_gender: [u8; 2],
    parent_item: [u8; 2],
    parent_nature: [u8; 2],
    masuda: bool,
    nidoran_volbeat: bool,
}

impl Daycare {
    pub fn new(
        parent_ivs: [[u8; 6]; 2],
        parent_ability: [u8; 2],
        parent_gender: [u8; 2],
        parent_item: [u8; 2],
        parent_nature: [u8; 2],
        masuda: bool,
        nidoran_volbeat: bool,
    ) -> Self {
        Self {
            parent_ivs,
            parent_ability,
            parent_gender,
            parent_item,
            parent_nature,
            masuda,
            nidoran_volbeat,
        }
    }

    pub fn get_parent_iv(&self, parent: usize, index: usize) -> u8 {
        self.parent_ivs[parent][index]
    }

    pub fn get_parent_ability(&self, parent: usize) -> u8 {
        self.parent_ability[parent]
    }

    pub fn get_parent_gender(&self, parent: usize) -> u8 {
        self.parent_gender[parent]
    }

    pub fn get_parent_item(&self, parent: usize) -> u8 {
        self.parent_item[parent]
    }

    pub fn get_parent_nature(&self, parent: usize) -> u8 {
        self.parent_nature[parent]
    }

    pub fn get_everstone_count(&self) -> u8 {
        self.parent_item.iter().filter(|&&i| i == 1).count() as u8
    }

    pub fn get_power_item_count(&self) -> u8 {
        self.parent_item
            .iter()
            .filter(|&i| (2..=7).contains(i))
            .count() as u8
    }

    pub fn get_ditto(&self) -> bool {
        self.parent_gender.iter().any(|&g| g == 3)
    }

    pub fn get_masuda(&self) -> bool {
        self.masuda
    }

    pub fn get_nidoran_volbeat(&self) -> bool {
        self.nidoran_volbeat
    }
}
