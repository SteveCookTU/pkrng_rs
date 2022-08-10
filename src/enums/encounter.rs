#[derive(Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Encounter {
    #[default]
    Grass,
    DoubleGrass,
    SpecialGrass,
    RockSmash,
    Surfing,
    SpecialSurf,
    OldRod,
    GoodRod,
    SuperRod,
    SpecialSuperRod,
    Static,
    BugCatchingContest,
    Headbutt,
    Roamer,
    Gift,
    EntraLink,
    GiftEgg,
    HiddenGrotto,
}

impl From<Encounter> for u8 {
    fn from(enc: Encounter) -> Self {
        enc as u8
    }
}
