#[derive(Default, Copy, Clone)]
pub struct LockInfo {
    nature: u8,
    gender_lower: u8,
    gender_upper: u8,
    free: bool,
}

impl LockInfo {
    pub fn new(nature: u8, gender_lower: u8, gender_upper: u8) -> Self {
        Self {
            nature,
            gender_lower,
            gender_upper,
            free: nature == 255 && gender_lower == 255 && gender_upper == 255,
        }
    }

    pub fn compare(&self, pid: u32) -> bool {
        let gender = (pid & 255) as u8;
        gender >= self.gender_lower
            && gender <= self.gender_upper
            && self.nature == (pid % 25) as u8
    }

    pub fn get_free(&self) -> bool {
        self.free
    }
}
