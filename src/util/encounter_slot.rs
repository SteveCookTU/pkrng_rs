use crate::Encounter;

fn calc_slot<const SIZE: usize, const GREATER: bool>(compare: u8, ranges: [u8; SIZE]) -> u8 {
    for (i, &range) in ranges.iter().enumerate() {
        if GREATER {
            if compare >= range {
                return i as u8;
            }
        } else if compare < range {
            return i as u8;
        }
    }

    255
}

pub fn h_slot(result: u16, encounter: Encounter) -> u8 {
    let compare = (result % 100) as u8;
    match encounter {
        Encounter::OldRod => calc_slot::<2, false>(compare, [70, 100]),
        Encounter::GoodRod => calc_slot::<3, false>(compare, [60, 80, 100]),
        Encounter::SuperRod => calc_slot::<5, false>(compare, [40, 80, 95, 99, 100]),
        Encounter::Surfing | Encounter::RockSmash => {
            calc_slot::<5, false>(compare, [60, 90, 95, 99, 100])
        }
        _ => calc_slot::<12, false>(compare, [20, 40, 50, 60, 70, 80, 85, 90, 94, 98, 99, 100]),
    }
}
