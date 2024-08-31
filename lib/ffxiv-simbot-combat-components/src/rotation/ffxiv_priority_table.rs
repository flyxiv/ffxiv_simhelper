use crate::jobs_skill_data::astrologian::priorities::AstrologianPriorityTable;
use crate::jobs_skill_data::bard::priorities::BardPriorityTable;
use crate::jobs_skill_data::black_mage::priorities::BlackmagePriorityTable;
use crate::jobs_skill_data::dancer::priorities::DancerPriorityTable;
use crate::jobs_skill_data::darkknight::priorities::DarkknightPriorityTable;
use crate::jobs_skill_data::dragoon::priorities::DragoonPriorityTable;
use crate::jobs_skill_data::gunbreaker::priorities::GunbreakerPriorityTable;
use crate::jobs_skill_data::machinist::priorities::MachinistPriorityTable;
use crate::jobs_skill_data::monk::priorities::MonkPriorityTable;
use crate::jobs_skill_data::ninja::priorities::NinjaPriorityTable;
use crate::jobs_skill_data::paladin::priorities::PaladinPriorityTable;
use crate::jobs_skill_data::pictomancer::priorities::PictomancerPriorityTable;
use crate::jobs_skill_data::reaper::priorities::ReaperPriorityTable;
use crate::jobs_skill_data::redmage::priorities::RedmagePriorityTable;
use crate::jobs_skill_data::sage::priorities::SagePriorityTable;
use crate::jobs_skill_data::samurai::priorities::SamuraiPriorityTable;
use crate::jobs_skill_data::scholar::priorities::ScholarPriorityTable;
use crate::jobs_skill_data::summoner::priorities::SummonerPriorityTable;
use crate::jobs_skill_data::viper::priorities::ViperPriorityTable;
use crate::jobs_skill_data::warrior::priorities::WarriorPriorityTable;
use crate::jobs_skill_data::white_mage::priorities::WhitemagePriorityTable;
use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::types::IdType;

#[derive(Clone)]
pub(crate) enum FfxivPriorityTable {
    Paladin(PaladinPriorityTable),
    Warrior(WarriorPriorityTable),
    Darkknight(DarkknightPriorityTable),
    Gunbreaker(GunbreakerPriorityTable),
    Whitemage(WhitemagePriorityTable),
    Scholar(ScholarPriorityTable),
    Astrologian(AstrologianPriorityTable),
    Sage(SagePriorityTable),
    Dragoon(DragoonPriorityTable),
    Monk(MonkPriorityTable),
    Ninja(NinjaPriorityTable),
    Samurai(SamuraiPriorityTable),
    Reaper(ReaperPriorityTable),
    Viper(ViperPriorityTable),
    Bard(BardPriorityTable),
    Dancer(DancerPriorityTable),
    Machinist(MachinistPriorityTable),
    Blackmage(BlackmagePriorityTable),
    Summoner(SummonerPriorityTable),
    Redmage(RedmagePriorityTable),
    Pictomancer(PictomancerPriorityTable),
}

impl PriorityTable for FfxivPriorityTable {
    fn get_opener_len(&self) -> usize {
        match self {
            Self::Paladin(paladin) => paladin.get_opener_len(),
            Self::Warrior(warrior) => warrior.get_opener_len(),
            Self::Darkknight(darkknight) => darkknight.get_opener_len(),
            Self::Gunbreaker(gunbreaker) => gunbreaker.get_opener_len(),
            Self::Whitemage(whitemage) => whitemage.get_opener_len(),
            Self::Scholar(scholar) => scholar.get_opener_len(),
            Self::Astrologian(astrologian) => astrologian.get_opener_len(),
            Self::Sage(sage) => sage.get_opener_len(),
            Self::Dragoon(dragoon) => dragoon.get_opener_len(),
            Self::Monk(monk) => monk.get_opener_len(),
            Self::Ninja(ninja) => ninja.get_opener_len(),
            Self::Samurai(samurai) => samurai.get_opener_len(),
            Self::Reaper(reaper) => reaper.get_opener_len(),
            Self::Viper(viper) => viper.get_opener_len(),
            Self::Bard(bard) => bard.get_opener_len(),
            Self::Dancer(dancer) => dancer.get_opener_len(),
            Self::Machinist(machinist) => machinist.get_opener_len(),
            Self::Blackmage(blackmage) => blackmage.get_opener_len(),
            Self::Summoner(summoner) => summoner.get_opener_len(),
            Self::Redmage(redmage) => redmage.get_opener_len(),
            Self::Pictomancer(pictomancer) => pictomancer.get_opener_len(),
        }
    }

    fn get_opener_at(&self, index: usize) -> Opener {
        match self {
            Self::Paladin(paladin) => paladin.get_opener_at(index),
            Self::Warrior(warrior) => warrior.get_opener_at(index),
            Self::Darkknight(darkknight) => darkknight.get_opener_at(index),
            Self::Gunbreaker(gunbreaker) => gunbreaker.get_opener_at(index),
            Self::Whitemage(whitemage) => whitemage.get_opener_at(index),
            Self::Scholar(scholar) => scholar.get_opener_at(index),
            Self::Astrologian(astrologian) => astrologian.get_opener_at(index),
            Self::Sage(sage) => sage.get_opener_at(index),
            Self::Dragoon(dragoon) => dragoon.get_opener_at(index),
            Self::Monk(monk) => monk.get_opener_at(index),
            Self::Ninja(ninja) => ninja.get_opener_at(index),
            Self::Samurai(samurai) => samurai.get_opener_at(index),
            Self::Reaper(reaper) => reaper.get_opener_at(index),
            Self::Viper(viper) => viper.get_opener_at(index),
            Self::Bard(bard) => bard.get_opener_at(index),
            Self::Dancer(dancer) => dancer.get_opener_at(index),
            Self::Machinist(machinist) => machinist.get_opener_at(index),
            Self::Blackmage(blackmage) => blackmage.get_opener_at(index),
            Self::Summoner(summoner) => summoner.get_opener_at(index),
            Self::Redmage(redmage) => redmage.get_opener_at(index),
            Self::Pictomancer(pictomancer) => pictomancer.get_opener_at(index),
        }
    }

    fn get_gcd_priority_table(&self) -> &[SkillPriorityInfo] {
        match self {
            Self::Paladin(paladin) => paladin.get_gcd_priority_table(),
            Self::Warrior(warrior) => warrior.get_gcd_priority_table(),
            Self::Darkknight(darkknight) => darkknight.get_gcd_priority_table(),
            Self::Gunbreaker(gunbreaker) => gunbreaker.get_gcd_priority_table(),
            Self::Whitemage(whitemage) => whitemage.get_gcd_priority_table(),
            Self::Scholar(scholar) => scholar.get_gcd_priority_table(),
            Self::Astrologian(astrologian) => astrologian.get_gcd_priority_table(),
            Self::Sage(sage) => sage.get_gcd_priority_table(),
            Self::Dragoon(dragoon) => dragoon.get_gcd_priority_table(),
            Self::Monk(monk) => monk.get_gcd_priority_table(),
            Self::Ninja(ninja) => ninja.get_gcd_priority_table(),
            Self::Samurai(samurai) => samurai.get_gcd_priority_table(),
            Self::Reaper(reaper) => reaper.get_gcd_priority_table(),
            Self::Viper(viper) => viper.get_gcd_priority_table(),
            Self::Bard(bard) => bard.get_gcd_priority_table(),
            Self::Dancer(dancer) => dancer.get_gcd_priority_table(),
            Self::Machinist(machinist) => machinist.get_gcd_priority_table(),
            Self::Blackmage(blackmage) => blackmage.get_gcd_priority_table(),
            Self::Summoner(summoner) => summoner.get_gcd_priority_table(),
            Self::Redmage(redmage) => redmage.get_gcd_priority_table(),
            Self::Pictomancer(pictomancer) => pictomancer.get_gcd_priority_table(),
        }
    }

    fn get_ogcd_priority_table(&self) -> &[SkillPriorityInfo] {
        match self {
            Self::Paladin(paladin) => paladin.get_ogcd_priority_table(),
            Self::Warrior(warrior) => warrior.get_ogcd_priority_table(),
            Self::Darkknight(darkknight) => darkknight.get_ogcd_priority_table(),
            Self::Gunbreaker(gunbreaker) => gunbreaker.get_ogcd_priority_table(),
            Self::Whitemage(whitemage) => whitemage.get_ogcd_priority_table(),
            Self::Scholar(scholar) => scholar.get_ogcd_priority_table(),
            Self::Astrologian(astrologian) => astrologian.get_ogcd_priority_table(),
            Self::Sage(sage) => sage.get_ogcd_priority_table(),
            Self::Dragoon(dragoon) => dragoon.get_ogcd_priority_table(),
            Self::Monk(monk) => monk.get_ogcd_priority_table(),
            Self::Ninja(ninja) => ninja.get_ogcd_priority_table(),
            Self::Samurai(samurai) => samurai.get_ogcd_priority_table(),
            Self::Reaper(reaper) => reaper.get_ogcd_priority_table(),
            Self::Viper(viper) => viper.get_ogcd_priority_table(),
            Self::Bard(bard) => bard.get_ogcd_priority_table(),
            Self::Dancer(dancer) => dancer.get_ogcd_priority_table(),
            Self::Machinist(machinist) => machinist.get_ogcd_priority_table(),
            Self::Blackmage(blackmage) => blackmage.get_ogcd_priority_table(),
            Self::Summoner(summoner) => summoner.get_ogcd_priority_table(),
            Self::Redmage(redmage) => redmage.get_ogcd_priority_table(),
            Self::Pictomancer(pictomancer) => pictomancer.get_ogcd_priority_table(),
        }
    }

    fn increment_turn(&self) {
        match self {
            Self::Paladin(paladin) => paladin.increment_turn(),
            Self::Warrior(warrior) => warrior.increment_turn(),
            Self::Darkknight(darkknight) => darkknight.increment_turn(),
            Self::Gunbreaker(gunbreaker) => gunbreaker.increment_turn(),
            Self::Whitemage(whitemage) => whitemage.increment_turn(),
            Self::Scholar(scholar) => scholar.increment_turn(),
            Self::Astrologian(astrologian) => astrologian.increment_turn(),
            Self::Sage(sage) => sage.increment_turn(),
            Self::Dragoon(dragoon) => dragoon.increment_turn(),
            Self::Monk(monk) => monk.increment_turn(),
            Self::Ninja(ninja) => ninja.increment_turn(),
            Self::Samurai(samurai) => samurai.increment_turn(),
            Self::Reaper(reaper) => reaper.increment_turn(),
            Self::Viper(viper) => viper.increment_turn(),
            Self::Bard(bard) => bard.increment_turn(),
            Self::Dancer(dancer) => dancer.increment_turn(),
            Self::Machinist(machinist) => machinist.increment_turn(),
            Self::Blackmage(blackmage) => blackmage.increment_turn(),
            Self::Summoner(summoner) => summoner.increment_turn(),
            Self::Redmage(redmage) => redmage.increment_turn(),
            Self::Pictomancer(pictomancer) => pictomancer.increment_turn(),
        }
    }

    fn get_turn_count(&self) -> IdType {
        match self {
            Self::Paladin(paladin) => paladin.get_turn_count(),
            Self::Warrior(warrior) => warrior.get_turn_count(),
            Self::Darkknight(darkknight) => darkknight.get_turn_count(),
            Self::Gunbreaker(gunbreaker) => gunbreaker.get_turn_count(),
            Self::Whitemage(whitemage) => whitemage.get_turn_count(),
            Self::Scholar(scholar) => scholar.get_turn_count(),
            Self::Astrologian(astrologian) => astrologian.get_turn_count(),
            Self::Sage(sage) => sage.get_turn_count(),
            Self::Dragoon(dragoon) => dragoon.get_turn_count(),
            Self::Monk(monk) => monk.get_turn_count(),
            Self::Ninja(ninja) => ninja.get_turn_count(),
            Self::Samurai(samurai) => samurai.get_turn_count(),
            Self::Reaper(reaper) => reaper.get_turn_count(),
            Self::Viper(viper) => viper.get_turn_count(),
            Self::Bard(bard) => bard.get_turn_count(),
            Self::Dancer(dancer) => dancer.get_turn_count(),
            Self::Machinist(machinist) => machinist.get_turn_count(),
            Self::Blackmage(blackmage) => blackmage.get_turn_count(),
            Self::Summoner(summoner) => summoner.get_turn_count(),
            Self::Redmage(redmage) => redmage.get_turn_count(),
            Self::Pictomancer(pictomancer) => pictomancer.get_turn_count(),
        }
    }
}
