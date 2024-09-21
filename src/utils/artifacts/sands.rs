use crate::utils::calculate::*;
use crate::utils::stats::*;

#[derive(Clone, Debug)]
pub struct Sands<'a> {
    main: &'a MainStat,
    sub: &'a [SubStat],
}

impl<'a> Sands<'a> {
    pub fn new(main: &'a MainStat, sub: &'a [SubStat]) -> Self {
        let possible = [
            MainStat::HpP,
            MainStat::AtkP,
            MainStat::DefP,
            MainStat::Elem,
            MainStat::ER,
        ];
        validate(&possible, &main, &sub);
        Sands { main, sub }
    }
}

impl<'a> Utils for Sands<'a> {
    fn get_main(&self) -> MainStat {
        self.main.to_owned()
    }
    fn get_sub(&self) -> Vec<SubStat> {
        self.sub.to_vec()
    }
    fn get_self_idx(&self) -> u8 {
        2
    }
    fn get_main_idx(&self) -> u8 {
        match self.main {
            MainStat::HpP => 0,
            MainStat::AtkP => 1,
            MainStat::DefP => 2,
            MainStat::ER => 3,
            MainStat::Elem => 4,
            _ => panic!("unexpected value!"),
        }
    }
    fn get_main_chances(&self) -> Vec<u16> {
        vec![2668, 2666, 2666, 1000, 1000]
    }
    fn get_roll_chances(&self) -> [u16; 10] {
        let mut roll_chances = [6, 6, 6, 4, 4, 4, 4, 4, 3, 3];
        match self.main {
            MainStat::HpP => {
                roll_chances[3] = 0;
            }
            MainStat::AtkP => {
                roll_chances[4] = 0;
            }
            MainStat::DefP => {
                roll_chances[5] = 0;
            }
            MainStat::ER => {
                roll_chances[6] = 0;
            }
            MainStat::Elem => {
                roll_chances[7] = 0;
            }
            _ => {}
        };
        roll_chances
    }
}

impl Roll for Sands<'_> {}
