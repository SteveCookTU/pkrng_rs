use crate::states::IDState;

#[derive(Default, Copy, Clone)]
pub struct IDState3 {
    advances: u32,
    tid: u16,
    sid: u16,
    tsv: u16,
}

impl IDState for IDState3 {
    fn get_advances(&self) -> u32 {
        self.advances
    }

    fn get_tid(&self) -> u16 {
        self.tid
    }

    fn get_sid(&self) -> u16 {
        self.sid
    }

    fn get_tsv(&self) -> u16 {
        self.tsv
    }
}

impl IDState3 {
    pub fn new(advances: u32, tid: u16, sid: u16) -> Self {
        Self {
            advances,
            tid,
            sid,
            tsv: (tid ^ sid) >> 3,
        }
    }
}
