pub type ARNG = LcRng<0x1, 0x6C078965>;
pub type PokeRNG = LcRng<0x6073, 0x41C64E6D>;
pub type ARNGR = LcRng<0x69C77F93, 0x9638806D>;
pub type PokeRNGR = LcRng<0xA3561A1, 0xEEB9EB65>;
pub type XDRNG = LcRng<0x269EC3, 0x343FD>;
pub type XDRNGR = LcRng<0xA170F641, 0xB9B33155>;

#[derive(Copy, Clone)]
pub struct LcRng<const MULT: u32, const ADD: u32> {
    seed: u32,
}

impl<const MULT: u32, const ADD: u32> LcRng<MULT, ADD> {
    pub fn new(seed: u32) -> LcRng<MULT, ADD> {
        Self { seed }
    }

    pub fn advance(&mut self, advances: u32) -> u32 {
        for _ in 0..advances {
            self.next_u32();
        }
        self.seed
    }

    pub fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(MULT).wrapping_add(ADD);
        self.seed
    }

    pub fn next_u16(&mut self) -> u16 {
        (self.next_u32() >> 16) as u16
    }

    pub fn get_seed(&self) -> u32 {
        self.seed
    }

    pub fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
    }
}
