#[derive(Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Lead {
    #[default]
    None,
    Search,
    Synchronize,
    CuteCharm,
    CuteCharm50M,
    CuteCharm75M,
    CuteCharm25M,
    CuteCharm875M,
    CuteCharm50F,
    CuteCharm75F,
    CuteCharm25F,
    CuteCharm125F,
    CuteCharmFemale,
    SuctionCups,
    CompoundEyes,
}

impl From<Lead> for u8 {
    fn from(lead: Lead) -> Self {
        lead as u8
    }
}
