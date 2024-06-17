use crate::jobs_skill_data::bard::priorities::BardPriorityTable;
use crate::jobs_skill_data::black_mage::priorities::BlackmagePriorityTable;
use crate::jobs_skill_data::dancer::priorities::DancerPriorityTable;
use crate::jobs_skill_data::dragoon::priorities::DragoonPriorityTable;
use crate::jobs_skill_data::gunbreaker::priorities::GunbreakerPriorityTable;
use crate::jobs_skill_data::machinist::priorities::MachinistPriorityTable;
use crate::jobs_skill_data::monk::priorities::MonkPriorityTable;
use crate::jobs_skill_data::ninja::priorities::NinjaPriorityTable;
use crate::jobs_skill_data::paladin::priorities::PaladinPriorityTable;
use crate::jobs_skill_data::reaper::priorities::ReaperPriorityTable;
use crate::jobs_skill_data::redmage::priorities::RedmagePriorityTable;
use crate::jobs_skill_data::sage::priorities::SagePriorityTable;
use crate::jobs_skill_data::samurai::priorities::SamuraiPriorityTable;
use crate::jobs_skill_data::scholar::priorities::ScholarPriorityTable;
use crate::jobs_skill_data::summoner::priorities::SummonerPriorityTable;
use crate::jobs_skill_data::warrior::priorities::WarriorPriorityTable;
use crate::jobs_skill_data::white_mage::priorities::WhitemagePriorityTable;
use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::IdType;

#[derive(Clone)]
pub(crate) enum FfxivPriorityTable {
    Ninja(NinjaPriorityTable),
    Sage(SagePriorityTable),
    Bard(BardPriorityTable),
    Dancer(DancerPriorityTable),
    Monk(MonkPriorityTable),
    Dragoon(DragoonPriorityTable),
    Blackmage(BlackmagePriorityTable),
    Whitemage(WhitemagePriorityTable),
    Paladin(PaladinPriorityTable),
    Warrior(WarriorPriorityTable),
    Scholar(ScholarPriorityTable),
    Summoner(SummonerPriorityTable),
    Redmage(RedmagePriorityTable),
    Gunbreaker(GunbreakerPriorityTable),
    Machinist(MachinistPriorityTable),
    Samurai(SamuraiPriorityTable),
    Reaper(ReaperPriorityTable),
}

impl PriorityTable for FfxivPriorityTable {
    fn get_opener_len(&self) -> usize {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_opener_len(),
            FfxivPriorityTable::Sage(sage) => sage.get_opener_len(),
            FfxivPriorityTable::Bard(bard) => bard.get_opener_len(),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_opener_len(),
            FfxivPriorityTable::Monk(monk) => monk.get_opener_len(),
            FfxivPriorityTable::Dragoon(dragoon) => dragoon.get_opener_len(),
            FfxivPriorityTable::Blackmage(blackmage) => blackmage.get_opener_len(),
            FfxivPriorityTable::Whitemage(whitemage) => whitemage.get_opener_len(),
            FfxivPriorityTable::Paladin(paladin) => paladin.get_opener_len(),
            FfxivPriorityTable::Warrior(warrior) => warrior.get_opener_len(),
            FfxivPriorityTable::Scholar(scholar) => scholar.get_opener_len(),
            FfxivPriorityTable::Summoner(summoner) => summoner.get_opener_len(),
            FfxivPriorityTable::Redmage(redmage) => redmage.get_opener_len(),
            FfxivPriorityTable::Gunbreaker(gunbreaker) => gunbreaker.get_opener_len(),
            FfxivPriorityTable::Machinist(machinist) => machinist.get_opener_len(),
            FfxivPriorityTable::Samurai(samurai) => samurai.get_opener_len(),
            FfxivPriorityTable::Reaper(reaper) => reaper.get_opener_len(),
        }
    }

    fn get_opener_at(&self, index: usize) -> Opener {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_opener_at(index),
            FfxivPriorityTable::Sage(sage) => sage.get_opener_at(index),
            FfxivPriorityTable::Bard(bard) => bard.get_opener_at(index),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_opener_at(index),
            FfxivPriorityTable::Monk(monk) => monk.get_opener_at(index),
            FfxivPriorityTable::Dragoon(dragoon) => dragoon.get_opener_at(index),
            FfxivPriorityTable::Blackmage(blackmage) => blackmage.get_opener_at(index),
            FfxivPriorityTable::Whitemage(whitemage) => whitemage.get_opener_at(index),
            FfxivPriorityTable::Paladin(paladin) => paladin.get_opener_at(index),
            FfxivPriorityTable::Warrior(warrior) => warrior.get_opener_at(index),
            FfxivPriorityTable::Scholar(scholar) => scholar.get_opener_at(index),
            FfxivPriorityTable::Summoner(summoner) => summoner.get_opener_at(index),
            FfxivPriorityTable::Redmage(redmage) => redmage.get_opener_at(index),
            FfxivPriorityTable::Gunbreaker(gunbreaker) => gunbreaker.get_opener_at(index),
            FfxivPriorityTable::Machinist(machinist) => machinist.get_opener_at(index),
            FfxivPriorityTable::Samurai(samurai) => samurai.get_opener_at(index),
            FfxivPriorityTable::Reaper(reaper) => reaper.get_opener_at(index),
        }
    }

    fn get_gcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_gcd_priority_table(),
            FfxivPriorityTable::Sage(sage) => sage.get_gcd_priority_table(),
            FfxivPriorityTable::Bard(bard) => bard.get_gcd_priority_table(),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_gcd_priority_table(),
            FfxivPriorityTable::Monk(monk) => monk.get_gcd_priority_table(),
            FfxivPriorityTable::Dragoon(dragoon) => dragoon.get_gcd_priority_table(),
            FfxivPriorityTable::Blackmage(blackmage) => blackmage.get_gcd_priority_table(),
            FfxivPriorityTable::Whitemage(whitemage) => whitemage.get_gcd_priority_table(),
            FfxivPriorityTable::Paladin(paladin) => paladin.get_gcd_priority_table(),
            FfxivPriorityTable::Warrior(warrior) => warrior.get_gcd_priority_table(),
            FfxivPriorityTable::Scholar(scholar) => scholar.get_gcd_priority_table(),
            FfxivPriorityTable::Summoner(summoner) => summoner.get_gcd_priority_table(),
            FfxivPriorityTable::Redmage(redmage) => redmage.get_gcd_priority_table(),
            FfxivPriorityTable::Gunbreaker(gunbreaker) => gunbreaker.get_gcd_priority_table(),
            FfxivPriorityTable::Machinist(machinist) => machinist.get_gcd_priority_table(),
            FfxivPriorityTable::Samurai(samurai) => samurai.get_gcd_priority_table(),
            FfxivPriorityTable::Reaper(reaper) => reaper.get_gcd_priority_table(),
        }
    }

    fn get_ogcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_ogcd_priority_table(),
            FfxivPriorityTable::Sage(sage) => sage.get_ogcd_priority_table(),
            FfxivPriorityTable::Bard(bard) => bard.get_ogcd_priority_table(),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_ogcd_priority_table(),
            FfxivPriorityTable::Monk(monk) => monk.get_ogcd_priority_table(),
            FfxivPriorityTable::Dragoon(dragoon) => dragoon.get_ogcd_priority_table(),
            FfxivPriorityTable::Blackmage(blackmage) => blackmage.get_ogcd_priority_table(),
            FfxivPriorityTable::Whitemage(whitemage) => whitemage.get_ogcd_priority_table(),
            FfxivPriorityTable::Paladin(paladin) => paladin.get_ogcd_priority_table(),
            FfxivPriorityTable::Warrior(warrior) => warrior.get_ogcd_priority_table(),
            FfxivPriorityTable::Scholar(scholar) => scholar.get_ogcd_priority_table(),
            FfxivPriorityTable::Summoner(summoner) => summoner.get_ogcd_priority_table(),
            FfxivPriorityTable::Redmage(redmage) => redmage.get_ogcd_priority_table(),
            FfxivPriorityTable::Gunbreaker(gunbreaker) => gunbreaker.get_ogcd_priority_table(),
            FfxivPriorityTable::Machinist(machinist) => machinist.get_ogcd_priority_table(),
            FfxivPriorityTable::Samurai(samurai) => samurai.get_ogcd_priority_table(),
            FfxivPriorityTable::Reaper(reaper) => reaper.get_ogcd_priority_table(),
        }
    }

    fn increment_turn(&self) {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.increment_turn(),
            FfxivPriorityTable::Sage(sage) => sage.increment_turn(),
            FfxivPriorityTable::Bard(bard) => bard.increment_turn(),
            FfxivPriorityTable::Dancer(dancer) => dancer.increment_turn(),
            FfxivPriorityTable::Monk(monk) => monk.increment_turn(),
            FfxivPriorityTable::Dragoon(dragoon) => dragoon.increment_turn(),
            FfxivPriorityTable::Blackmage(blackmage) => blackmage.increment_turn(),
            FfxivPriorityTable::Whitemage(whitemage) => whitemage.increment_turn(),
            FfxivPriorityTable::Paladin(paladin) => paladin.increment_turn(),
            FfxivPriorityTable::Warrior(warrior) => warrior.increment_turn(),
            FfxivPriorityTable::Scholar(scholar) => scholar.increment_turn(),
            FfxivPriorityTable::Summoner(summoner) => summoner.increment_turn(),
            FfxivPriorityTable::Redmage(redmage) => redmage.increment_turn(),
            FfxivPriorityTable::Gunbreaker(gunbreaker) => gunbreaker.increment_turn(),
            FfxivPriorityTable::Machinist(machinist) => machinist.increment_turn(),
            FfxivPriorityTable::Samurai(samurai) => samurai.increment_turn(),
            FfxivPriorityTable::Reaper(reaper) => reaper.increment_turn(),
        }
    }

    fn get_turn_count(&self) -> IdType {
        match self {
            FfxivPriorityTable::Ninja(ninja) => ninja.get_turn_count(),
            FfxivPriorityTable::Sage(sage) => sage.get_turn_count(),
            FfxivPriorityTable::Bard(bard) => bard.get_turn_count(),
            FfxivPriorityTable::Dancer(dancer) => dancer.get_turn_count(),
            FfxivPriorityTable::Monk(monk) => monk.get_turn_count(),
            FfxivPriorityTable::Dragoon(dragoon) => dragoon.get_turn_count(),
            FfxivPriorityTable::Blackmage(blackmage) => blackmage.get_turn_count(),
            FfxivPriorityTable::Whitemage(whitemage) => whitemage.get_turn_count(),
            FfxivPriorityTable::Paladin(paladin) => paladin.get_turn_count(),
            FfxivPriorityTable::Warrior(warrior) => warrior.get_turn_count(),
            FfxivPriorityTable::Scholar(scholar) => scholar.get_turn_count(),
            FfxivPriorityTable::Summoner(summoner) => summoner.get_turn_count(),
            FfxivPriorityTable::Redmage(redmage) => redmage.get_turn_count(),
            FfxivPriorityTable::Gunbreaker(gunbreaker) => gunbreaker.get_turn_count(),
            FfxivPriorityTable::Machinist(machinist) => machinist.get_turn_count(),
            FfxivPriorityTable::Samurai(samurai) => samurai.get_turn_count(),
            FfxivPriorityTable::Reaper(reaper) => reaper.get_turn_count(),
        }
    }
}
