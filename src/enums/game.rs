use std::ops::BitAnd;

#[derive(Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u32)]
pub enum Game {
    #[default]
    None,
    Ruby = 1,
    Sapphire = 1 << 1,
    RS = Game::Ruby as u32 | Game::Sapphire as u32,
    Emerald = 1 << 2,
    RSE = Game::RS as u32 | Game::Emerald as u32,
    FireRed = 1 << 3,
    LeafGreen = 1 << 4,
    FRLG = Game::FireRed as u32 | Game::LeafGreen as u32,
    Gen3 = Game::RSE as u32 | Game::FRLG as u32,
    Gales = 1 << 5,
    Colosseum = 1 << 6,
    GC = Game::Gales as u32 | Game::Colosseum as u32,
    Diamond = 1 << 7,
    Pearl = 1 << 8,
    DP = Game::Diamond as u32 | Game::Pearl as u32,
    Platinum = 1 << 9,
    DPPt = Game::DP as u32 | Game::Platinum as u32,
    HeartGold = 1 << 10,
    SoulSilver = 1 << 11,
    HGSS = Game::HeartGold as u32 | Game::SoulSilver as u32,
    Gen4 = Game::DPPt as u32 | Game::HGSS as u32,
    Black = 1 << 12,
    White = 1 << 13,
    BW = Game::Black as u32 | Game::White as u32,
    Black2 = 1 << 14,
    White2 = 1 << 15,
    BW2 = Game::Black2 as u32 | Game::White2 as u32,
    Gen5 = Game::BW as u32 | Game::BW2 as u32,
    Sword = 1 << 24,
    Shield = 1 << 25,
    SwSh = Game::Sword as u32 | Game::Shield as u32,
    BD = 1 << 26,
    SP = 1 << 27,
    BDSP = Game::BD as u32 | Game::SP as u32,
    Gen8 = Game::SwSh as u32 | Game::BDSP as u32,
}

impl BitAnd for Game {
    type Output = u32;

    fn bitand(self, rhs: Self) -> Self::Output {
        (self as u32) & (rhs as u32)
    }
}
