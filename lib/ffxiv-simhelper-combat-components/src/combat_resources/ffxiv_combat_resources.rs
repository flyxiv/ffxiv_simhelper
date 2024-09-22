use crate::combat_resources::CombatResource;
use crate::event::FfxivEventQueue;
use crate::jobs_skill_data::astrologian::combat_resources::AstrologianCombatResources;
use crate::jobs_skill_data::bard::combat_resources::BardCombatResources;
use crate::jobs_skill_data::black_mage::combat_resources::BlackmageCombatResources;
use crate::jobs_skill_data::dancer::combat_resources::DancerCombatResources;
use crate::jobs_skill_data::darkknight::combat_resources::DarkknightCombatResources;
use crate::jobs_skill_data::dragoon::combat_resources::DragoonCombatResources;
use crate::jobs_skill_data::gunbreaker::combat_resources::GunbreakerCombatResources;
use crate::jobs_skill_data::machinist::combat_resources::MachinistCombatResources;
use crate::jobs_skill_data::monk::combat_resources::MonkCombatResources;
use crate::jobs_skill_data::ninja::combat_resources::NinjaCombatResources;
use crate::jobs_skill_data::paladin::combat_resources::PaladinCombatResources;
use crate::jobs_skill_data::pictomancer::combat_resources::PictomancerCombatResources;
use crate::jobs_skill_data::reaper::combat_resources::ReaperCombatResources;
use crate::jobs_skill_data::redmage::combat_resources::RedmageCombatResources;
use crate::jobs_skill_data::sage::combat_resources::SageCombatResources;
use crate::jobs_skill_data::samurai::combat_resources::SamuraiCombatResources;
use crate::jobs_skill_data::scholar::combat_resources::ScholarCombatResources;
use crate::jobs_skill_data::summoner::combat_resources::SummonerCombatResources;
use crate::jobs_skill_data::viper::combat_resources::ViperCombatResources;
use crate::jobs_skill_data::warrior::combat_resources::WarriorCombatResources;
use crate::jobs_skill_data::white_mage::combat_resources::WhitemageCombatResources;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, PlayerIdType, ResourceIdType, ResourceType, SkillIdType, TimeType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const ALL_FFXIV_COMBAT_JOBS: [&str; 21] = [
    "PLD", "WAR", "DRK", "GNB", "WHM", "SCH", "AST", "SGE", "MNK", "DRG", "NIN", "SAM", "RPR",
    "VPR", "BRD", "DNC", "MCH", "BLM", "SMN", "RDM", "PCT",
];

#[derive(Clone)]
pub(crate) enum FfxivCombatResources {
    Paladin(PaladinCombatResources),
    Warrior(WarriorCombatResources),
    Darkknight(DarkknightCombatResources),
    Gunbreaker(GunbreakerCombatResources),
    Astrologian(AstrologianCombatResources),
    Whitemage(WhitemageCombatResources),
    Sage(SageCombatResources),
    Dragoon(DragoonCombatResources),
    Monk(MonkCombatResources),
    Ninja(NinjaCombatResources),
    Samurai(SamuraiCombatResources),
    Reaper(ReaperCombatResources),
    Viper(ViperCombatResources),
    Bard(BardCombatResources),
    Machinist(MachinistCombatResources),
    Dancer(DancerCombatResources),
    Scholar(ScholarCombatResources),
    Blackmage(BlackmageCombatResources),
    Summoner(SummonerCombatResources),
    Redmage(RedmageCombatResources),
    Pictomancer(PictomancerCombatResources),
}

impl CombatResource for FfxivCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        match self {
            Self::Paladin(paladin_resources) => paladin_resources.get_skills_mut(),
            Self::Warrior(warrior_resources) => warrior_resources.get_skills_mut(),
            Self::Darkknight(darkknight_resources) => darkknight_resources.get_skills_mut(),
            Self::Gunbreaker(gunbreaker_resources) => gunbreaker_resources.get_skills_mut(),
            Self::Whitemage(whitemage_resources) => whitemage_resources.get_skills_mut(),
            Self::Scholar(scholar_resources) => scholar_resources.get_skills_mut(),
            Self::Astrologian(astrologian_resources) => astrologian_resources.get_skills_mut(),
            Self::Sage(sage_resources) => sage_resources.get_skills_mut(),
            Self::Dragoon(dragoon_resources) => dragoon_resources.get_skills_mut(),
            Self::Monk(monk_resources) => monk_resources.get_skills_mut(),
            Self::Ninja(ninja_resources) => ninja_resources.get_skills_mut(),
            Self::Samurai(samurai_resources) => samurai_resources.get_skills_mut(),
            Self::Reaper(reaper_resources) => reaper_resources.get_skills_mut(),
            Self::Viper(viper_resources) => viper_resources.get_skills_mut(),
            Self::Bard(bard_resources) => bard_resources.get_skills_mut(),
            Self::Machinist(machinist_resources) => machinist_resources.get_skills_mut(),
            Self::Dancer(dancer_resources) => dancer_resources.get_skills_mut(),
            Self::Blackmage(blackmage_resources) => blackmage_resources.get_skills_mut(),
            Self::Summoner(summoner_resources) => summoner_resources.get_skills_mut(),
            Self::Redmage(redmage_resources) => redmage_resources.get_skills_mut(),
            Self::Pictomancer(pictomancer_resources) => pictomancer_resources.get_skills_mut(),
        }
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        match self {
            Self::Paladin(paladin_resources) => paladin_resources.get_skills(),
            Self::Warrior(warrior_resources) => warrior_resources.get_skills(),
            Self::Darkknight(darkknight_resources) => darkknight_resources.get_skills(),
            Self::Gunbreaker(gunbreaker_resources) => gunbreaker_resources.get_skills(),
            Self::Whitemage(whitemage_resources) => whitemage_resources.get_skills(),
            Self::Scholar(scholar_resources) => scholar_resources.get_skills(),
            Self::Astrologian(astrologian_resources) => astrologian_resources.get_skills(),
            Self::Sage(sage_resources) => sage_resources.get_skills(),
            Self::Dragoon(dragoon_resources) => dragoon_resources.get_skills(),
            Self::Monk(monk_resources) => monk_resources.get_skills(),
            Self::Ninja(ninja_resources) => ninja_resources.get_skills(),
            Self::Samurai(samurai_resources) => samurai_resources.get_skills(),
            Self::Reaper(reaper_resources) => reaper_resources.get_skills(),
            Self::Viper(viper_resources) => viper_resources.get_skills(),
            Self::Bard(bard_resources) => bard_resources.get_skills(),
            Self::Machinist(machinist_resources) => machinist_resources.get_skills(),
            Self::Dancer(dancer_resources) => dancer_resources.get_skills(),
            Self::Blackmage(blackmage_resources) => blackmage_resources.get_skills(),
            Self::Summoner(summoner_resources) => summoner_resources.get_skills(),
            Self::Redmage(redmage_resources) => redmage_resources.get_skills(),
            Self::Pictomancer(pictomancer_resources) => pictomancer_resources.get_skills(),
        }
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_type: ResourceType) {
        match self {
            Self::Paladin(paladin_resources) => {
                paladin_resources.add_resource(resource_id, resource_type)
            }
            Self::Warrior(warrior_resources) => {
                warrior_resources.add_resource(resource_id, resource_type)
            }
            Self::Darkknight(darkknight_resources) => {
                darkknight_resources.add_resource(resource_id, resource_type)
            }
            Self::Gunbreaker(gunbreaker_resources) => {
                gunbreaker_resources.add_resource(resource_id, resource_type)
            }
            Self::Whitemage(whitemage_resources) => {
                whitemage_resources.add_resource(resource_id, resource_type)
            }
            Self::Scholar(scholar_resources) => {
                scholar_resources.add_resource(resource_id, resource_type)
            }
            Self::Astrologian(astrologian_resources) => {
                astrologian_resources.add_resource(resource_id, resource_type)
            }
            Self::Sage(sage_resources) => sage_resources.add_resource(resource_id, resource_type),
            Self::Dragoon(dragoon_resources) => {
                dragoon_resources.add_resource(resource_id, resource_type)
            }
            Self::Monk(monk_resources) => monk_resources.add_resource(resource_id, resource_type),
            Self::Ninja(ninja_resources) => {
                ninja_resources.add_resource(resource_id, resource_type)
            }
            Self::Samurai(samurai_resources) => {
                samurai_resources.add_resource(resource_id, resource_type)
            }
            Self::Reaper(reaper_resources) => {
                reaper_resources.add_resource(resource_id, resource_type)
            }
            Self::Viper(viper_resources) => {
                viper_resources.add_resource(resource_id, resource_type)
            }
            Self::Bard(bard_resources) => bard_resources.add_resource(resource_id, resource_type),
            Self::Machinist(machinist_resources) => {
                machinist_resources.add_resource(resource_id, resource_type)
            }
            Self::Dancer(dancer_resources) => {
                dancer_resources.add_resource(resource_id, resource_type)
            }
            Self::Blackmage(blackmage_resources) => {
                blackmage_resources.add_resource(resource_id, resource_type)
            }
            Self::Summoner(summoner_resources) => {
                summoner_resources.add_resource(resource_id, resource_type)
            }
            Self::Redmage(redmage_resources) => {
                redmage_resources.add_resource(resource_id, resource_type)
            }
            Self::Pictomancer(pictomancer_resources) => {
                pictomancer_resources.add_resource(resource_id, resource_type)
            }
        }
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        match self {
            Self::Paladin(paladin_resources) => paladin_resources.get_resource(resource_id),
            Self::Warrior(warrior_resources) => warrior_resources.get_resource(resource_id),
            Self::Darkknight(darkknight_resources) => {
                darkknight_resources.get_resource(resource_id)
            }
            Self::Gunbreaker(gunbreaker_resources) => {
                gunbreaker_resources.get_resource(resource_id)
            }
            Self::Whitemage(whitemage_resources) => whitemage_resources.get_resource(resource_id),
            Self::Scholar(scholar_resources) => scholar_resources.get_resource(resource_id),
            Self::Astrologian(astrologian_resources) => {
                astrologian_resources.get_resource(resource_id)
            }
            Self::Sage(sage_resources) => sage_resources.get_resource(resource_id),
            Self::Dragoon(dragoon_resources) => dragoon_resources.get_resource(resource_id),
            Self::Monk(monk_resources) => monk_resources.get_resource(resource_id),
            Self::Ninja(ninja_resources) => ninja_resources.get_resource(resource_id),
            Self::Samurai(samurai_resources) => samurai_resources.get_resource(resource_id),
            Self::Reaper(reaper_resources) => reaper_resources.get_resource(resource_id),
            Self::Viper(viper_resources) => viper_resources.get_resource(resource_id),
            Self::Bard(bard_resources) => bard_resources.get_resource(resource_id),
            Self::Machinist(machinist_resources) => machinist_resources.get_resource(resource_id),
            Self::Dancer(dancer_resources) => dancer_resources.get_resource(resource_id),
            Self::Blackmage(blackmage_resources) => blackmage_resources.get_resource(resource_id),
            Self::Summoner(summoner_resources) => summoner_resources.get_resource(resource_id),
            Self::Redmage(redmage_resources) => redmage_resources.get_resource(resource_id),
            Self::Pictomancer(pictomancer_resources) => {
                pictomancer_resources.get_resource(resource_id)
            }
        }
    }

    fn get_current_combo(&self) -> ComboType {
        match self {
            Self::Paladin(paladin_resources) => paladin_resources.get_current_combo(),
            Self::Warrior(warrior_resources) => warrior_resources.get_current_combo(),
            Self::Darkknight(darkknight_resources) => darkknight_resources.get_current_combo(),
            Self::Gunbreaker(gunbreaker_resources) => gunbreaker_resources.get_current_combo(),
            Self::Whitemage(whitemage_resources) => whitemage_resources.get_current_combo(),
            Self::Scholar(scholar_resources) => scholar_resources.get_current_combo(),
            Self::Astrologian(astrologian_resources) => astrologian_resources.get_current_combo(),
            Self::Sage(sage_resources) => sage_resources.get_current_combo(),
            Self::Dragoon(dragoon_resources) => dragoon_resources.get_current_combo(),
            Self::Monk(monk_resources) => monk_resources.get_current_combo(),
            Self::Ninja(ninja_resources) => ninja_resources.get_current_combo(),
            Self::Samurai(samurai_resources) => samurai_resources.get_current_combo(),
            Self::Reaper(reaper_resources) => reaper_resources.get_current_combo(),
            Self::Viper(viper_resources) => viper_resources.get_current_combo(),
            Self::Bard(bard_resources) => bard_resources.get_current_combo(),
            Self::Machinist(machinist_resources) => machinist_resources.get_current_combo(),
            Self::Dancer(dancer_resources) => dancer_resources.get_current_combo(),
            Self::Blackmage(blackmage_resources) => blackmage_resources.get_current_combo(),
            Self::Summoner(summoner_resources) => summoner_resources.get_current_combo(),
            Self::Redmage(redmage_resources) => redmage_resources.get_current_combo(),
            Self::Pictomancer(pictomancer_resources) => pictomancer_resources.get_current_combo(),
        }
    }

    fn update_combo(&mut self, combo: &ComboType) {
        match self {
            Self::Paladin(paladin_resources) => paladin_resources.update_combo(combo),
            Self::Warrior(warrior_resources) => warrior_resources.update_combo(combo),
            Self::Darkknight(darkknight_resources) => darkknight_resources.update_combo(combo),
            Self::Gunbreaker(gunbreaker_resources) => gunbreaker_resources.update_combo(combo),
            Self::Whitemage(whitemage_resources) => whitemage_resources.update_combo(combo),
            Self::Scholar(scholar_resources) => scholar_resources.update_combo(combo),
            Self::Astrologian(astrologian_resources) => astrologian_resources.update_combo(combo),
            Self::Sage(sage_resources) => sage_resources.update_combo(combo),
            Self::Dragoon(dragoon_resources) => dragoon_resources.update_combo(combo),
            Self::Monk(monk_resources) => monk_resources.update_combo(combo),
            Self::Ninja(ninja_resources) => ninja_resources.update_combo(combo),
            Self::Samurai(samurai_resources) => samurai_resources.update_combo(combo),
            Self::Reaper(reaper_resources) => reaper_resources.update_combo(combo),
            Self::Viper(viper_resources) => viper_resources.update_combo(combo),
            Self::Bard(bard_resources) => bard_resources.update_combo(combo),
            Self::Machinist(machinist_resources) => machinist_resources.update_combo(combo),
            Self::Dancer(dancer_resources) => dancer_resources.update_combo(combo),
            Self::Blackmage(blackmage_resources) => blackmage_resources.update_combo(combo),
            Self::Summoner(summoner_resources) => summoner_resources.update_combo(combo),
            Self::Redmage(redmage_resources) => redmage_resources.update_combo(combo),
            Self::Pictomancer(pictomancer_resources) => pictomancer_resources.update_combo(combo),
        }
    }

    fn trigger_on_event(
        &mut self,
        skill_id: SkillIdType,
        buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_time_millisecond: TimeType,
        player: &FfxivPlayer,
    ) -> SkillEvents {
        match self {
            Self::Paladin(paladin_resources) => paladin_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Warrior(warrior_resources) => warrior_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Darkknight(darkknight_resources) => darkknight_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Gunbreaker(gunbreaker_resources) => gunbreaker_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Whitemage(whitemage_resources) => whitemage_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Scholar(scholar_resources) => scholar_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Astrologian(astrologian_resources) => astrologian_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Sage(sage_resources) => sage_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Dragoon(dragoon_resources) => dragoon_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Monk(monk_resources) => monk_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Ninja(ninja_resources) => ninja_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Samurai(samurai_resources) => samurai_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Reaper(reaper_resources) => reaper_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Viper(viper_resources) => viper_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Bard(bard_resources) => bard_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Machinist(machinist_resources) => machinist_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Dancer(dancer_resources) => dancer_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Blackmage(blackmage_resources) => blackmage_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Summoner(summoner_resources) => summoner_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Redmage(redmage_resources) => redmage_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
            Self::Pictomancer(pictomancer_resources) => pictomancer_resources.trigger_on_event(
                skill_id,
                buff_list,
                debuff_list,
                current_time_millisecond,
                player,
            ),
        }
    }

    fn get_next_buff_target(&self, skill_id: SkillIdType) -> PlayerIdType {
        match self {
            Self::Paladin(paladin_resources) => paladin_resources.get_next_buff_target(skill_id),
            Self::Warrior(warrior_resources) => warrior_resources.get_next_buff_target(skill_id),
            Self::Darkknight(darkknight_resources) => {
                darkknight_resources.get_next_buff_target(skill_id)
            }
            Self::Gunbreaker(gunbreaker_resources) => {
                gunbreaker_resources.get_next_buff_target(skill_id)
            }
            Self::Whitemage(whitemage_resources) => {
                whitemage_resources.get_next_buff_target(skill_id)
            }
            Self::Scholar(scholar_resources) => scholar_resources.get_next_buff_target(skill_id),
            Self::Astrologian(astrologian_resources) => {
                astrologian_resources.get_next_buff_target(skill_id)
            }
            Self::Sage(sage_resources) => sage_resources.get_next_buff_target(skill_id),
            Self::Dragoon(dragoon_resources) => dragoon_resources.get_next_buff_target(skill_id),
            Self::Monk(monk_resources) => monk_resources.get_next_buff_target(skill_id),
            Self::Ninja(ninja_resources) => ninja_resources.get_next_buff_target(skill_id),
            Self::Samurai(samurai_resources) => samurai_resources.get_next_buff_target(skill_id),
            Self::Reaper(reaper_resources) => reaper_resources.get_next_buff_target(skill_id),
            Self::Viper(viper_resources) => viper_resources.get_next_buff_target(skill_id),
            Self::Bard(bard_resources) => bard_resources.get_next_buff_target(skill_id),
            Self::Machinist(machinist_resources) => {
                machinist_resources.get_next_buff_target(skill_id)
            }
            Self::Dancer(dancer_resources) => dancer_resources.get_next_buff_target(skill_id),
            Self::Blackmage(blackmage_resources) => {
                blackmage_resources.get_next_buff_target(skill_id)
            }
            Self::Summoner(summoner_resources) => summoner_resources.get_next_buff_target(skill_id),
            Self::Redmage(redmage_resources) => redmage_resources.get_next_buff_target(skill_id),
            Self::Pictomancer(pictomancer_resources) => {
                pictomancer_resources.get_next_buff_target(skill_id)
            }
        }
    }

    fn update_stack_timer(&mut self, elapsed_time: TimeType) {
        match self {
            Self::Paladin(paladin_resources) => paladin_resources.update_stack_timer(elapsed_time),
            Self::Warrior(warrior_resources) => warrior_resources.update_stack_timer(elapsed_time),
            Self::Darkknight(darkknight_resources) => {
                darkknight_resources.update_stack_timer(elapsed_time)
            }
            Self::Gunbreaker(gunbreaker_resources) => {
                gunbreaker_resources.update_stack_timer(elapsed_time)
            }
            Self::Whitemage(whitemage_resources) => {
                whitemage_resources.update_stack_timer(elapsed_time)
            }
            Self::Scholar(scholar_resources) => scholar_resources.update_stack_timer(elapsed_time),
            Self::Astrologian(astrologian_resources) => {
                astrologian_resources.update_stack_timer(elapsed_time)
            }
            Self::Sage(sage_resources) => sage_resources.update_stack_timer(elapsed_time),
            Self::Dragoon(dragoon_resources) => dragoon_resources.update_stack_timer(elapsed_time),
            Self::Monk(monk_resources) => monk_resources.update_stack_timer(elapsed_time),
            Self::Ninja(ninja_resources) => ninja_resources.update_stack_timer(elapsed_time),
            Self::Samurai(samurai_resources) => samurai_resources.update_stack_timer(elapsed_time),
            Self::Reaper(reaper_resources) => reaper_resources.update_stack_timer(elapsed_time),
            Self::Viper(viper_resources) => viper_resources.update_stack_timer(elapsed_time),
            Self::Bard(bard_resources) => bard_resources.update_stack_timer(elapsed_time),
            Self::Machinist(machinist_resources) => {
                machinist_resources.update_stack_timer(elapsed_time)
            }
            Self::Dancer(dancer_resources) => dancer_resources.update_stack_timer(elapsed_time),
            Self::Blackmage(blackmage_resources) => {
                blackmage_resources.update_stack_timer(elapsed_time)
            }
            Self::Summoner(summoner_resources) => {
                summoner_resources.update_stack_timer(elapsed_time)
            }
            Self::Redmage(redmage_resources) => redmage_resources.update_stack_timer(elapsed_time),
            Self::Pictomancer(pictomancer_resources) => {
                pictomancer_resources.update_stack_timer(elapsed_time)
            }
        }
    }

    fn trigger_on_crit(&mut self) {
        match self {
            Self::Paladin(paladin_resources) => paladin_resources.trigger_on_crit(),
            Self::Warrior(warrior_resources) => warrior_resources.trigger_on_crit(),
            Self::Darkknight(darkknight_resources) => darkknight_resources.trigger_on_crit(),
            Self::Gunbreaker(gunbreaker_resources) => gunbreaker_resources.trigger_on_crit(),
            Self::Whitemage(whitemage_resources) => whitemage_resources.trigger_on_crit(),
            Self::Scholar(scholar_resources) => scholar_resources.trigger_on_crit(),
            Self::Astrologian(astrologian_resources) => astrologian_resources.trigger_on_crit(),
            Self::Sage(sage_resources) => sage_resources.trigger_on_crit(),
            Self::Dragoon(dragoon_resources) => dragoon_resources.trigger_on_crit(),
            Self::Monk(monk_resources) => monk_resources.trigger_on_crit(),
            Self::Ninja(ninja_resources) => ninja_resources.trigger_on_crit(),
            Self::Samurai(samurai_resources) => samurai_resources.trigger_on_crit(),
            Self::Reaper(reaper_resources) => reaper_resources.trigger_on_crit(),
            Self::Viper(viper_resources) => viper_resources.trigger_on_crit(),
            Self::Bard(bard_resources) => bard_resources.trigger_on_crit(),
            Self::Machinist(machinist_resources) => machinist_resources.trigger_on_crit(),
            Self::Dancer(dancer_resources) => dancer_resources.trigger_on_crit(),
            Self::Blackmage(blackmage_resources) => blackmage_resources.trigger_on_crit(),
            Self::Summoner(summoner_resources) => summoner_resources.trigger_on_crit(),
            Self::Redmage(redmage_resources) => redmage_resources.trigger_on_crit(),
            Self::Pictomancer(pictomancer_resources) => pictomancer_resources.trigger_on_crit(),
        }
    }
}

impl FfxivCombatResources {
    /// for AST, partner_player_id1 is the player_id of melee partner, partner_player_id2 is the player_id of ranged partner
    pub(crate) fn new(
        job_abbrev: &String,
        player_id: PlayerIdType,
        partner_player_id1: Option<PlayerIdType>,
        partner_player_id2: Option<PlayerIdType>,
        event_queue: Rc<RefCell<FfxivEventQueue>>,
        player_count: usize,
    ) -> Self {
        debug_assert!(ALL_FFXIV_COMBAT_JOBS.contains(&job_abbrev.as_str()));

        match job_abbrev.as_str() {
            "PLD" => Self::Paladin(PaladinCombatResources::new(player_id)),
            "WAR" => Self::Warrior(WarriorCombatResources::new(player_id)),
            "DRK" => Self::Darkknight(DarkknightCombatResources::new(player_id)),
            "GNB" => Self::Gunbreaker(GunbreakerCombatResources::new(player_id)),
            "WHM" => Self::Whitemage(WhitemageCombatResources::new(player_id)),
            "SCH" => Self::Scholar(ScholarCombatResources::new(player_id)),
            "AST" => Self::Astrologian(AstrologianCombatResources::new(
                player_id,
                partner_player_id1.unwrap(),
                partner_player_id2.unwrap(),
            )),
            "SGE" => Self::Sage(SageCombatResources::new(player_id)),
            "MNK" => Self::Monk(MonkCombatResources::new(player_id)),
            "DRG" => Self::Dragoon(DragoonCombatResources::new(player_id)),
            "NIN" => Self::Ninja(NinjaCombatResources::new(player_id)),
            "SAM" => Self::Samurai(SamuraiCombatResources::new(player_id)),
            "RPR" => Self::Reaper(ReaperCombatResources::new(player_id, player_count)),
            "VPR" => Self::Viper(ViperCombatResources::new(player_id)),
            "BRD" => Self::Bard(BardCombatResources::new(player_id, event_queue.clone())),
            "MCH" => Self::Machinist(MachinistCombatResources::new(player_id)),
            "DNC" => Self::Dancer(DancerCombatResources::new(
                player_id,
                partner_player_id1.unwrap(),
            )),
            "BLM" => Self::Blackmage(BlackmageCombatResources::new(player_id)),
            "SMN" => Self::Summoner(SummonerCombatResources::new(player_id, event_queue.clone())),
            "RDM" => Self::Redmage(RedmageCombatResources::new(player_id)),
            _ => Self::Pictomancer(PictomancerCombatResources::new(player_id)),
        }
    }
}
