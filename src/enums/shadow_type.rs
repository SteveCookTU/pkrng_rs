#[derive(Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum ShadowType {
    #[default]
    SingleLock,
    FirstShadow,
    Salamence,
    SecondShadow,
    EReader,
}

impl TryFrom<u8> for ShadowType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ShadowType::SingleLock),
            1 => Ok(ShadowType::FirstShadow),
            2 => Ok(ShadowType::Salamence),
            3 => Ok(ShadowType::SecondShadow),
            4 => Ok(ShadowType::EReader),
            _ => Err(()),
        }
    }
}
