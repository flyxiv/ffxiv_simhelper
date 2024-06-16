use crate::combat_resources::CombatResource;
use crate::event::FfxivEventQueue;
use crate::jobs_skill_data::bard::combat_resources::BardCombatResources;
use crate::jobs_skill_data::black_mage::combat_resources::BlackmageCombatResources;
use crate::jobs_skill_data::dancer::combat_resources::DancerCombatResources;
use crate::jobs_skill_data::dragoon::combat_resources::DragoonCombatResources;
use crate::jobs_skill_data::monk::combat_resources::MonkCombatResources;
use crate::jobs_skill_data::ninja::combat_resources::NinjaCombatResources;
use crate::jobs_skill_data::paladin::combat_resources::PaladinCombatResources;
use crate::jobs_skill_data::sage::combat_resources::SageCombatResources;
use crate::jobs_skill_data::scholar::combat_resources::ScholarCombatResources;
use crate::jobs_skill_data::summoner::combat_resources::SummonerCombatResources;
use crate::jobs_skill_data::warrior::combat_resources::WarriorCombatResources;
use crate::jobs_skill_data::white_mage::combat_resources::WhitemageCombatResources;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{ComboType, IdType, ResourceType, TimeType};
use ffxiv_simbot_db::job::Job;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) enum FfxivCombatResources {
    Sage(SageCombatResources),
    Ninja(NinjaCombatResources),
    Bard(BardCombatResources),
    Dancer(DancerCombatResources),
    Monk(MonkCombatResources),
    Dragoon(DragoonCombatResources),
    Blackmage(BlackmageCombatResources),
    Whitemage(WhitemageCombatResources),
    Paladin(PaladinCombatResources),
    Warrior(WarriorCombatResources),
    Scholar(ScholarCombatResources),
    Summoner(SummonerCombatResources),
}

impl CombatResource for FfxivCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        match self {
            Self::Sage(sage_resources) => sage_resources.get_skills_mut(),
            Self::Ninja(ninja_resources) => ninja_resources.get_skills_mut(),
            Self::Bard(bard_resources) => bard_resources.get_skills_mut(),
            Self::Dancer(dancer_resources) => dancer_resources.get_skills_mut(),
            Self::Monk(monk_resources) => monk_resources.get_skills_mut(),
            Self::Dragoon(dragoon_resources) => dragoon_resources.get_skills_mut(),
            Self::Blackmage(blackmage_resources) => blackmage_resources.get_skills_mut(),
            Self::Whitemage(whitemage_resources) => whitemage_resources.get_skills_mut(),
            Self::Paladin(paladin_resources) => paladin_resources.get_skills_mut(),
            Self::Warrior(warrior_resources) => warrior_resources.get_skills_mut(),
            Self::Scholar(scholar_resources) => scholar_resources.get_skills_mut(),
            Self::Summoner(summoner_resources) => summoner_resources.get_skills_mut(),
        }
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        match self {
            Self::Sage(sage_resources) => sage_resources.get_skills(),
            Self::Ninja(ninja_resources) => ninja_resources.get_skills(),
            Self::Bard(bard_resources) => bard_resources.get_skills(),
            Self::Dancer(dancer_resources) => dancer_resources.get_skills(),
            Self::Monk(monk_resources) => monk_resources.get_skills(),
            Self::Dragoon(dragoon_resources) => dragoon_resources.get_skills(),
            Self::Blackmage(blackmage_resources) => blackmage_resources.get_skills(),
            Self::Whitemage(whitemage_resources) => whitemage_resources.get_skills(),
            Self::Paladin(paladin_resources) => paladin_resources.get_skills(),
            Self::Warrior(warrior_resources) => warrior_resources.get_skills(),
            Self::Scholar(scholar_resources) => scholar_resources.get_skills(),
            Self::Summoner(summoner_resources) => summoner_resources.get_skills(),
        }
    }

    fn add_resource(&mut self, resource_id: IdType, resource_type: ResourceType) {
        match self {
            Self::Sage(sage_resources) => sage_resources.add_resource(resource_id, resource_type),
            Self::Ninja(ninja_resources) => {
                ninja_resources.add_resource(resource_id, resource_type)
            }
            Self::Bard(bard_resources) => bard_resources.add_resource(resource_id, resource_type),
            Self::Dancer(dancer_resources) => {
                dancer_resources.add_resource(resource_id, resource_type)
            }
            Self::Monk(monk_resources) => monk_resources.add_resource(resource_id, resource_type),
            Self::Dragoon(dragoon_resources) => {
                dragoon_resources.add_resource(resource_id, resource_type)
            }
            Self::Blackmage(blackmage_resources) => {
                blackmage_resources.add_resource(resource_id, resource_type)
            }
            Self::Whitemage(whitemage_resources) => {
                whitemage_resources.add_resource(resource_id, resource_type)
            }
            Self::Paladin(paladin_resources) => {
                paladin_resources.add_resource(resource_id, resource_type)
            }
            Self::Warrior(warrior_resources) => {
                warrior_resources.add_resource(resource_id, resource_type)
            }
            Self::Scholar(scholar_resources) => {
                scholar_resources.add_resource(resource_id, resource_type)
            }
            Self::Summoner(summoner_resources) => {
                summoner_resources.add_resource(resource_id, resource_type)
            }
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        match self {
            Self::Sage(sage_resources) => sage_resources.get_resource(resource_id),
            Self::Ninja(ninja_resources) => ninja_resources.get_resource(resource_id),
            Self::Bard(bard_resources) => bard_resources.get_resource(resource_id),
            Self::Dancer(dancer_resources) => dancer_resources.get_resource(resource_id),
            Self::Monk(monk_resources) => monk_resources.get_resource(resource_id),
            Self::Dragoon(dragoon_resources) => dragoon_resources.get_resource(resource_id),
            Self::Blackmage(blackmage_resources) => blackmage_resources.get_resource(resource_id),
            Self::Whitemage(whitemage_resources) => whitemage_resources.get_resource(resource_id),
            Self::Paladin(paladin_resources) => paladin_resources.get_resource(resource_id),
            Self::Warrior(warrior_resources) => warrior_resources.get_resource(resource_id),
            Self::Scholar(scholar_resources) => scholar_resources.get_resource(resource_id),
            Self::Summoner(summoner_resources) => summoner_resources.get_resource(resource_id),
        }
    }

    fn get_current_combo(&self) -> ComboType {
        match self {
            Self::Sage(sage_resources) => sage_resources.get_current_combo(),
            Self::Ninja(ninja_resources) => ninja_resources.get_current_combo(),
            Self::Bard(bard_resources) => bard_resources.get_current_combo(),
            Self::Dancer(dancer_resources) => dancer_resources.get_current_combo(),
            Self::Monk(monk_resources) => monk_resources.get_current_combo(),
            Self::Dragoon(dragoon_resources) => dragoon_resources.get_current_combo(),
            Self::Blackmage(blackmage_resources) => blackmage_resources.get_current_combo(),
            Self::Whitemage(whitemage_resources) => whitemage_resources.get_current_combo(),
            Self::Paladin(paladin_resources) => paladin_resources.get_current_combo(),
            Self::Warrior(warrior_resources) => warrior_resources.get_current_combo(),
            Self::Scholar(scholar_resources) => scholar_resources.get_current_combo(),
            Self::Summoner(summoner_resources) => summoner_resources.get_current_combo(),
        }
    }

    fn update_combo(&mut self, combo: &ComboType) {
        match self {
            Self::Sage(sage_resources) => sage_resources.update_combo(combo),
            Self::Ninja(ninja_resources) => ninja_resources.update_combo(combo),
            Self::Bard(bard_resources) => bard_resources.update_combo(combo),
            Self::Dancer(dancer_resources) => dancer_resources.update_combo(combo),
            Self::Monk(monk_resources) => monk_resources.update_combo(combo),
            Self::Dragoon(dragoon_resources) => dragoon_resources.update_combo(combo),
            Self::Blackmage(blackmage_resources) => blackmage_resources.update_combo(combo),
            Self::Whitemage(whitemage_resources) => whitemage_resources.update_combo(combo),
            Self::Paladin(paladin_resources) => paladin_resources.update_combo(combo),
            Self::Warrior(warrior_resources) => warrior_resources.update_combo(combo),
            Self::Scholar(scholar_resources) => scholar_resources.update_combo(combo),
            Self::Summoner(summoner_resources) => summoner_resources.update_combo(combo),
        }
    }

    fn trigger_on_event(
        &mut self,
        skill_id: IdType,
        buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_time_millisecond: TimeType,
        player: &FfxivPlayer,
    ) -> SkillEvents {
        match self {
            Self::Sage(sage_resources) => sage_resources.trigger_on_event(
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
            Self::Bard(bard_resources) => bard_resources.trigger_on_event(
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
            Self::Monk(monk_resources) => monk_resources.trigger_on_event(
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
            Self::Blackmage(blackmage_resources) => blackmage_resources.trigger_on_event(
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
            Self::Scholar(scholar_resources) => scholar_resources.trigger_on_event(
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
        }
    }

    fn get_next_buff_target(&self, skill_id: IdType) -> IdType {
        match self {
            Self::Sage(sage_resources) => sage_resources.get_next_buff_target(skill_id),
            Self::Ninja(ninja_resources) => ninja_resources.get_next_buff_target(skill_id),
            Self::Bard(bard_resources) => bard_resources.get_next_buff_target(skill_id),
            Self::Dancer(dancer_resources) => dancer_resources.get_next_buff_target(skill_id),
            Self::Monk(monk_resources) => monk_resources.get_next_buff_target(skill_id),
            Self::Dragoon(dragoon_resources) => dragoon_resources.get_next_buff_target(skill_id),
            Self::Blackmage(blackmage_resources) => {
                blackmage_resources.get_next_buff_target(skill_id)
            }
            Self::Whitemage(whitemage_resources) => {
                whitemage_resources.get_next_buff_target(skill_id)
            }
            Self::Paladin(paladin_resources) => paladin_resources.get_next_buff_target(skill_id),
            Self::Warrior(warrior_resources) => warrior_resources.get_next_buff_target(skill_id),
            Self::Scholar(scholar_resources) => scholar_resources.get_next_buff_target(skill_id),
            Self::Summoner(summoner_resources) => summoner_resources.get_next_buff_target(skill_id),
        }
    }

    fn update_stack_timer(&mut self, elapsed_time: TimeType) {
        match self {
            Self::Sage(sage_resources) => sage_resources.update_stack_timer(elapsed_time),
            Self::Ninja(ninja_resources) => ninja_resources.update_stack_timer(elapsed_time),
            Self::Bard(bard_resources) => bard_resources.update_stack_timer(elapsed_time),
            Self::Dancer(dancer_resources) => dancer_resources.update_stack_timer(elapsed_time),
            Self::Monk(monk_resources) => monk_resources.update_stack_timer(elapsed_time),
            Self::Dragoon(dragoon_resources) => dragoon_resources.update_stack_timer(elapsed_time),
            Self::Blackmage(blackmage_resources) => {
                blackmage_resources.update_stack_timer(elapsed_time)
            }
            Self::Whitemage(whitemage_resources) => {
                whitemage_resources.update_stack_timer(elapsed_time)
            }
            Self::Paladin(paladin_resources) => paladin_resources.update_stack_timer(elapsed_time),
            Self::Warrior(warrior_resources) => warrior_resources.update_stack_timer(elapsed_time),
            Self::Scholar(scholar_resources) => scholar_resources.update_stack_timer(elapsed_time),
            Self::Summoner(summoner_resources) => {
                summoner_resources.update_stack_timer(elapsed_time)
            }
        }
    }

    fn trigger_on_crit(&mut self) {
        match self {
            Self::Sage(sage_resources) => sage_resources.trigger_on_crit(),
            Self::Ninja(ninja_resources) => ninja_resources.trigger_on_crit(),
            Self::Bard(bard_resources) => bard_resources.trigger_on_crit(),
            Self::Dancer(dancer_resources) => dancer_resources.trigger_on_crit(),
            Self::Monk(monk_resources) => monk_resources.trigger_on_crit(),
            Self::Dragoon(dragoon_resources) => dragoon_resources.trigger_on_crit(),
            Self::Blackmage(blackmage_resources) => blackmage_resources.trigger_on_crit(),
            Self::Whitemage(whitemage_resources) => whitemage_resources.trigger_on_crit(),
            Self::Paladin(paladin_resources) => paladin_resources.trigger_on_crit(),
            Self::Warrior(warrior_resources) => warrior_resources.trigger_on_crit(),
            Self::Scholar(scholar_resources) => scholar_resources.trigger_on_crit(),
            Self::Summoner(summoner_resources) => summoner_resources.trigger_on_crit(),
        }
    }
}

impl FfxivCombatResources {
    pub(crate) fn new(
        job_abbrev: &String,
        player_id: IdType,
        partner_player_id: Option<IdType>,
        event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> Self {
        match job_abbrev.as_str() {
            "SGE" => Self::Sage(SageCombatResources::new(player_id)),
            "NIN" => Self::Ninja(NinjaCombatResources::new(player_id)),
            "BRD" => Self::Bard(BardCombatResources::new(player_id, event_queue.clone())),
            "DNC" => Self::Dancer(DancerCombatResources::new(
                player_id,
                partner_player_id.unwrap(),
            )),
            "MNK" => Self::Monk(MonkCombatResources::new(player_id)),
            "DRG" => Self::Dragoon(DragoonCombatResources::new(
                player_id,
                partner_player_id.unwrap(),
            )),
            "BLM" => Self::Blackmage(BlackmageCombatResources::new(player_id)),
            "WHM" => Self::Whitemage(WhitemageCombatResources::new(player_id)),
            "PLD" => Self::Paladin(PaladinCombatResources::new(player_id)),
            "WAR" => Self::Warrior(WarriorCombatResources::new(player_id)),
            "SCH" => Self::Scholar(ScholarCombatResources::new(player_id)),
            "SMN" => Self::Summoner(SummonerCombatResources::new(player_id, event_queue.clone())),
            _ => Self::Sage(SageCombatResources::new(player_id)),
        }
    }
}
