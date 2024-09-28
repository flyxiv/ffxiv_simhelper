use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::jobs_skill_data::ninja::abilities::make_ninja_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::types::{ComboType, PlayerIdType, ResourceIdType, ResourceType, StatusIdType};
use crate::types::{SkillIdType, TimeType};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const NINJA_STACK_COUNT: usize = 2;

pub const BUNSHIN_STACK_ID: StatusIdType = 1011;
pub const BUNSHIN_CLONE_ID: SkillIdType = 1022;

const NINKI_MAX: ResourceType = 100;
const SHURIKEN_MAX_STACK: ResourceType = 5;

const NINJA_MAX_STACKS: [ResourceType; NINJA_STACK_COUNT] = [NINKI_MAX, SHURIKEN_MAX_STACK];

lazy_static! {
    static ref NINJA_BUNSHIN_COMBO_IDS: Vec<SkillIdType> = vec![1005, 1006, 1007, 1008, 1002];
}

#[derive(Clone)]
pub(crate) struct NinjaCombatResources {
    skills: SkillTable<AttackSkill>,
    resources: [ResourceType; NINJA_STACK_COUNT],
    current_combo: ComboType,
}

impl CombatResource for NinjaCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_type: ResourceType) {
        let resource_id = resource_id as usize;
        self.resources[resource_id] = min(
            NINJA_MAX_STACKS[resource_id],
            self.resources[resource_id] + resource_type,
        );
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        self.resources[resource_id as usize]
    }

    fn get_current_combo(&self) -> ComboType {
        self.current_combo
    }

    fn update_combo(&mut self, combo: &ComboType) {
        if let Some(combo_id) = combo {
            self.current_combo = Some(*combo_id);
        }
    }

    fn trigger_on_event(
        &mut self,
        skill_id: SkillIdType,
        buff_list: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_time_millisecond: TimeType,
        player: &FfxivPlayer,
    ) -> SkillEvents {
        if NINJA_BUNSHIN_COMBO_IDS.contains(&skill_id) {
            let key = StatusKey::new(BUNSHIN_STACK_ID, player.get_id());

            if buff_list.borrow().contains_key(&key) {
                return (
                    vec![FfxivEvent::UseSkill(
                        player.get_id(),
                        None,
                        BUNSHIN_CLONE_ID,
                        current_time_millisecond,
                    )],
                    vec![],
                );
            }
        }

        return (vec![], vec![]);
    }

    fn trigger_on_crit(&mut self) {}

    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }
    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl NinjaCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_ninja_skill_list(player_id),
            resources: [0; NINJA_STACK_COUNT],
            current_combo: None,
        }
    }
}
