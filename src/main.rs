use artifact_generator::utils::artifacts::circlet::*;
use artifact_generator::utils::artifacts::flower::*;
use artifact_generator::utils::artifacts::goblet::*;
use artifact_generator::utils::artifacts::plume::*;
use artifact_generator::utils::artifacts::sands::*;
use artifact_generator::utils::calculate::*;
use artifact_generator::utils::stats::*;

fn main() {
    let try_n = 10_000_000;

    let substat1 = [SubStat::CritR, SubStat::CritD];
    let substat2 = [SubStat::CritD, SubStat::AtkP];

    let flower = Flower::new(&substat1);
    let plume = Plume::new(&substat1);
    let sands = Sands::new(&MainStat::AtkP, &substat1);
    let goblet = Goblet::new(&MainStat::CyroP, &substat1);
    let circlet = Circlet::new(&MainStat::CritR, &substat2);

    flower.roll(try_n);
    plume.roll(try_n);
    sands.roll(try_n);
    goblet.roll(try_n);
    circlet.roll(try_n);
}
