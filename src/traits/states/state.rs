pub trait State {
    fn get_seed(&self) -> u32;

    fn set_seed(&mut self, seed: u32);

    fn get_advances(&self) -> u32;

    fn set_advances(&mut self, advances: u32);

    fn get_pid(&self) -> u32;

    fn set_pid(&mut self, pid: u32);

    fn set_pid_halves(&mut self, high: u16, low: u16) {
        self.set_pid(((high as u32) << 16) | low as u32);
    }

    fn get_iv(&self, index: usize) -> u8;

    fn set_iv(&mut self, index: usize, iv: u8);

    fn set_iv_halves(&mut self, iv1: u16, iv2: u16) {
        let hp = (iv1 & 0x1f) as u8;
        let atk = ((iv1 >> 5) & 0x1f) as u8;
        let def = ((iv1 >> 10) & 0x1f) as u8;
        let spa = ((iv2 >> 5) & 0x1f) as u8;
        let spd = ((iv2 >> 10) & 0x1f) as u8;
        let spe = (iv2 & 0x1f) as u8;

        self.set_ivs_individual(hp, atk, def, spa, spd, spe);
    }

    fn set_iv_full(&mut self, iv: u32) {
        let iv1 = iv >> 16;
        let iv2 = iv & 0xffff;

        self.set_iv_halves(iv1 as u16, iv2 as u16);
    }

    fn set_ivs_individual(&mut self, hp: u8, atk: u8, def: u8, spa: u8, spd: u8, spe: u8);

    fn get_ability(&self) -> u8;

    fn set_ability(&mut self, ability: u8);

    fn get_gender(&self) -> u8;

    fn set_gender(&mut self, gender: u8);

    fn set_gender_with_ratio(&mut self, gender: u8, gender_ratio: u8) {
        let gender = match gender_ratio {
            255 => 2,
            254 => 1,
            0 => 0,
            _ => {
                if gender < gender_ratio {
                    1
                } else {
                    0
                }
            }
        };
        self.set_gender(gender);
    }

    fn get_hidden(&self) -> u8;

    fn set_hidden(&mut self, hidden: u8);

    fn get_power(&self) -> u8;

    fn set_power(&mut self, power: u8);

    fn get_nature(&self) -> u8;

    fn set_nature(&mut self, nature: u8);

    fn get_level(&self) -> u8 {
        1
    }

    fn set_level(&mut self, _level: u8) {}

    fn get_shiny(&self) -> u8;

    fn set_shiny(&mut self, shiny: u8);

    fn set_shiny_from_comparison(&mut self, tsv: u16, psv: u16, compare: u8) {
        let shiny = if tsv == psv {
            2
        } else if ((tsv ^ psv) as u8) < compare {
            1
        } else {
            0
        };

        self.set_shiny(shiny);
    }

    fn calculate_hidden_power(&mut self) {
        const ORDER: [usize; 6] = [0, 1, 2, 5, 3, 4];
        let mut h = 0;
        let mut p = 0;

        for &i in ORDER.iter() {
            h += (self.get_iv(i) & 1) << i;
            p += ((self.get_iv(i) >> 1) & 1) << i;
        }

        self.set_hidden(h.wrapping_mul(15) / 63);
        self.set_power(30 + (p.wrapping_mul(40) / 63));
    }
}
