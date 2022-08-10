use crate::{Encounter, Lead};

pub trait WildGenerator {
    fn set_encounter(&mut self, encounter: Encounter);

    fn set_lead(&mut self, lead: Lead);

    fn set_synch_nature(&mut self, synch_nature: u8);
}
