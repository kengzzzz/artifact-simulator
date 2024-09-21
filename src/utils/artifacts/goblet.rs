use crate::utils::calculate::*;
use crate::utils::stats::*;

#[derive(Clone, Debug)]
pub struct Goblet<'a> {
    main: &'a MainStat,
    sub: &'a [SubStat],
}

impl<'a> Goblet<'a> {
    pub fn new(main: &'a MainStat, sub: &'a [SubStat]) -> Self {
        let possible = [
            MainStat::HpP,
            MainStat::AtkP,
            MainStat::DefP,
            MainStat::Elem,
            MainStat::PyroP,
            MainStat::ElectroP,
            MainStat::CyroP,
            MainStat::HydroP,
            MainStat::DendroP,
            MainStat::AnemoP,
            MainStat::GeoP,
            MainStat::PhysicalP,
        ];
        validate(&possible, &main, &sub);
        Goblet { main, sub }
    }
}

impl<'a> Utils for Goblet<'a> {
    fn get_main(&self) -> MainStat {
        self.main.to_owned()
    }
    fn get_sub(&self) -> Vec<SubStat> {
        self.sub.to_vec()
    }
    fn get_self_idx(&self) -> u8 {
        3
    }
    fn get_main_idx(&self) -> u8 {
        match self.main {
            MainStat::HpP => 0,
            MainStat::AtkP => 1,
            MainStat::DefP => 2,
            MainStat::PyroP => 3,
            MainStat::ElectroP => 4,
            MainStat::CyroP => 5,
            MainStat::HydroP => 6,
            MainStat::DendroP => 7,
            MainStat::AnemoP => 8,
            MainStat::GeoP => 9,
            MainStat::PhysicalP => 10,
            MainStat::Elem => 11,
            _ => panic!("unexpected value!"),
        }
    }
    fn get_main_chances(&self) -> Vec<u16> {
        vec![
            1925, 1925, 1900, 500, 500, 500, 500, 500, 500, 500, 500, 250,
        ]
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
            MainStat::Elem => {
                roll_chances[7] = 0;
            }
            _ => {}
        };
        roll_chances
    }
}

impl Roll for Goblet<'_> {}
