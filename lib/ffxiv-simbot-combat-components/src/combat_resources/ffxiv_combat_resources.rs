use crate::combat_resources::CombatResource;
use crate::event::FfxivEventQueue;
use crate::jobs_skill_data::bard::combat_resources::BardCombatResources;
use crate::jobs_skill_data::dancer::combat_resources::DancerCombatResources;
use crate::jobs_skill_data::dragoon::combat_resources::DragoonCombatResources;
use crate::jobs_skill_data::monk::combat_resources::MonkCombatResources;
use crate::jobs_skill_data::ninja::combat_resources::NinjaCombatResources;
use crate::jobs_skill_data::sage::combat_resources::SageCombatResources;
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
        }
    }

    fn trigger_on_event(
        &self,
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
        }
    }
}

impl FfxivCombatResources {
    pub(crate) fn new(
        job: &Job,
        player_id: IdType,
        partner_player_id: Option<IdType>,
        event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> Self {
        match job.abbrev.as_str() {
            "Sage" => Self::Sage(SageCombatResources::new(player_id)),
            "NIN" => Self::Ninja(NinjaCombatResources::new(player_id)),
            "BRD" => Self::Bard(BardCombatResources::new(player_id, event_queue)),
            "DNC" => Self::Dancer(DancerCombatResources::new(
                player_id,
                partner_player_id.unwrap(),
            )),
            "MNK" => Self::Monk(MonkCombatResources::new(player_id)),
            "DRG" => Self::Dragoon(DragoonCombatResources::new(
                player_id,
                partner_player_id.unwrap(),
            )),
            _ => Self::Sage(SageCombatResources::new(player_id)),
        }
    }
}
