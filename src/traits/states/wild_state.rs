use crate::states::State;
use crate::{Encounter, Lead};

pub trait WildState: State {
    fn get_lead(&self) -> Lead;

    fn set_lead(&mut self, lead: Lead);

    fn get_encounter_slot(&self) -> u8;

    fn set_encounter_slot(&mut self, encounter_slot: u8);

    fn get_encounter(&self) -> Encounter;

    fn set_encounter(&mut self, encounter: Encounter);

    fn get_item(&self) -> u8;

    fn set_item(&mut self, item: u8);
}
