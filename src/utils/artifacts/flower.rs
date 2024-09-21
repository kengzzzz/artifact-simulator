use crate::utils::calculate::*;
use crate::utils::stats::*;

#[derive(Clone, Debug)]
pub struct Flower<'a> {
    sub: &'a [SubStat],
}

impl<'a> Flower<'a> {
    pub fn new(sub: &'a [SubStat]) -> Self {
        let possible = [MainStat::Hp];
        validate(&possible, &MainStat::Hp, &sub);
        Flower { sub }
    }
}

impl<'a> Utils for Flower<'a> {
    fn get_main(&self) -> MainStat {
        MainStat::Hp
    }
    fn get_sub(&self) -> Vec<SubStat> {
        self.sub.to_vec()
    }
    fn get_self_idx(&self) -> u8 {
        0
    }
    fn get_main_idx(&self) -> u8 {
        0
    }
    fn get_main_chances(&self) -> Vec<u16> {
        vec![1]
    }
    fn get_roll_chances(&self) -> [u16; 10] {
        [0, 6, 6, 4, 4, 4, 4, 4, 3, 3]
    }
}

impl Roll for Flower<'_> {}
