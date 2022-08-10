use crate::states::IDState;

pub struct IDFilter {
    tid_filter: Vec<u16>,
    sid_filter: Vec<u16>,
    tsv_filter: Vec<u16>,
}

impl IDFilter {
    pub fn new(tid_filter: Vec<u16>, sid_filter: Vec<u16>, tsv_filter: Vec<u16>) -> Self {
        Self {
            tid_filter,
            sid_filter,
            tsv_filter,
        }
    }

    pub fn compare(&self, state: &dyn IDState) -> bool {
        if !self.tid_filter.is_empty() && !self.tid_filter.iter().any(|&tid| tid == state.get_tid())
        {
            return false;
        }

        if !self.sid_filter.is_empty() && !self.sid_filter.iter().any(|&sid| sid == state.get_sid())
        {
            return false;
        }

        if !self.tsv_filter.is_empty() && !self.tsv_filter.iter().any(|&tsv| tsv == state.get_tsv())
        {
            return false;
        }

        true
    }
}
