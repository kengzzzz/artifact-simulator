#[derive(Clone, PartialEq, Debug)]
pub enum MainStat {
    Hp,
    Atk,
    HpP,
    AtkP,
    DefP,
    ER,
    Elem,
    PyroP,
    ElectroP,
    CyroP,
    HydroP,
    DendroP,
    AnemoP,
    GeoP,
    PhysicalP,
    CritR,
    CritD,
    HealB,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SubStat {
    Hp,
    Atk,
    Def,
    HpP,
    AtkP,
    DefP,
    ER,
    Elem,
    CritR,
    CritD,
}

impl MainStat {
    pub fn eq_substat(&self) -> Option<SubStat> {
        match self {
            MainStat::Hp => Some(SubStat::Hp),
            MainStat::Atk => Some(SubStat::Atk),
            MainStat::HpP => Some(SubStat::HpP),
            MainStat::AtkP => Some(SubStat::AtkP),
            MainStat::DefP => Some(SubStat::DefP),
            MainStat::ER => Some(SubStat::ER),
            MainStat::Elem => Some(SubStat::Elem),
            MainStat::CritR => Some(SubStat::CritR),
            MainStat::CritD => Some(SubStat::CritD),
            _ => None,
        }
    }
}
