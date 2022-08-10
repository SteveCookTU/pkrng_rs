use crate::gen3::LockInfo;
use crate::{Method, ShadowType};

const COLO: &[u8] = include_bytes!("../../resources/encounters/gen3/colo.bin");
const GALES: &[u8] = include_bytes!("../../resources/encounters/gen3/gales.bin");

#[derive(Default, Clone)]
pub struct ShadowTeam {
    locks: Vec<LockInfo>,
    shadow_type: ShadowType,
}

impl ShadowTeam {
    pub fn new(locks: Vec<LockInfo>, shadow_type: ShadowType) -> Self {
        Self { locks, shadow_type }
    }

    pub fn get_locks(&self) -> &[LockInfo] {
        &self.locks
    }

    pub fn get_type(&self) -> ShadowType {
        self.shadow_type
    }
}

pub fn load_shadow_teams(version: Method) -> Vec<ShadowTeam> {
    let data = match version {
        Method::XD => GALES,
        _ => COLO,
    };

    let mut teams = Vec::new();

    let mut i = 0;

    while i < data.len() {
        let count = data[i];

        let shadow_type = ShadowType::try_from(data[i + 1]).unwrap();

        let mut locks = Vec::with_capacity(count as usize);
        for j in 0..count as usize {
            let nature = data[i + 2 + j * 3];
            let gender_lower = data[i + 3 + j * 3];
            let gender_upper = data[i + 4 + j * 3];
            locks.push(LockInfo::new(nature, gender_lower, gender_upper));
        }

        teams.push(ShadowTeam::new(locks, shadow_type));

        i += usize::from(count) * 3 + 2;
    }

    teams
}
