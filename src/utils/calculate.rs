use crate::utils::stats::*;
use rand::Rng;
use std::fmt::Debug;

pub fn rand_with_weight(arr: &[u16]) -> usize {
    let sum: u16 = arr.iter().sum();
    let mut rng = rand::thread_rng();
    let mut result = rng.gen_range(0..=sum);
    for (idx, elem) in arr.iter().enumerate() {
        if *elem >= result {
            return idx;
        }
        result -= *elem;
    }
    panic!("unexpected rand!");
}

pub fn validate(accept_main: &[MainStat], main: &MainStat, sub: &[SubStat]) {
    if !accept_main.contains(main) {
        panic!("Invalid Main Stat!");
    }
    if (1..sub.len()).any(|i| sub[i..].contains(&sub[i - 1])) {
        panic!("Duplicate Sub Stat!");
    }
    if let Some(stat) = main.eq_substat() {
        if sub.contains(&stat) {
            panic!("Invalid Sub Stat!");
        }
    }
}

pub trait Utils {
    fn get_main(&self) -> MainStat;
    fn get_sub(&self) -> Vec<SubStat>;
    fn get_self_idx(&self) -> u8;
    fn get_main_idx(&self) -> u8;
    fn get_main_chances(&self) -> Vec<u16>;
    fn get_roll_chances(&self) -> [u16; 10];

    fn parse_substat(&self) -> Vec<i32> {
        self.get_sub()
            .iter()
            .map(|e| match e {
                SubStat::Hp => 0,
                SubStat::Atk => 1,
                SubStat::Def => 2,
                SubStat::HpP => 3,
                SubStat::AtkP => 4,
                SubStat::DefP => 5,
                SubStat::ER => 6,
                SubStat::Elem => 7,
                SubStat::CritR => 8,
                SubStat::CritD => 9,
            })
            .collect()
    }
}

pub trait Roll: Utils + Debug {
    fn roll(&self, try_n: u32) {
        let subs = self.parse_substat();
        let mut count = 0;
        for _ in 0..try_n {
            let mut rng = rand::thread_rng();
            if rng.gen_range(0..5) != self.get_self_idx() {
                continue;
            }
            if rng.gen_range(0..2) != 1 {
                continue;
            }
            let main_stat = rand_with_weight(&self.get_main_chances()) as u8;
            if main_stat != self.get_main_idx() {
                continue;
            }
            let mut r = self.get_roll_chances();
            let a1 = rand_with_weight(&r);
            r[a1] = 0;
            let a2 = rand_with_weight(&r);
            r[a2] = 0;
            let a3 = rand_with_weight(&r);
            r[a3] = 0;
            let a4 = rand_with_weight(&r);
            r[a4] = 0;
            if subs.iter().all(|&e| r[e as usize] == 0) {
                count += 1;
            }
        }
        let percentage = count as f64 / try_n as f64 * 100.0;
        println!(
            "The chance of obtaining {:?} in the specified set is {:.4}%",
            self, percentage
        );
    }
}
