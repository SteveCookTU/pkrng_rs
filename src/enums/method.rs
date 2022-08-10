#[derive(Default, Copy, Clone, PartialOrd, PartialEq, Eq)]
#[repr(u8)]
pub enum Method {
    #[default]
    None,
    Method1,
    Method1Reverse,
    Method2,
    Method4,
    MethodH1,
    MethodH2,
    MethodH4,
    XD,
    Colo,
    XDColo,
    Channel,
    EBred,
    EBredSplit,
    EBredAlternate,
    EBredPID,
    RSFRLGBred,
    RSFRLGBredSplit,
    RSFRLGBredAlternate,
    RSFRLGBredMixed,
    MethodJ,
    MethodK,
    ChainedShiny,
    WondercardIVs,
    Gen4Normal,
    Gen4Masuda,
    DPPtIVs,
    HGSSIVs,
    Gen4Combined,
    Method5IVs,
    Method5CGear,
    Method5,
    BWBred,
    BW2Bred,
    DreamRadar,
    Method5Event,
}

impl From<Method> for u8 {
    fn from(method: Method) -> Self {
        method as u8
    }
}
