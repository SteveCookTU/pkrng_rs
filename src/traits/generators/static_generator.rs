use crate::generators::Generator;
use crate::Lead;

pub trait StaticGenerator: Generator {
    fn set_lead(&mut self, lead: Lead);

    fn set_synch_nature(&mut self, synch_nature: u8);
}
