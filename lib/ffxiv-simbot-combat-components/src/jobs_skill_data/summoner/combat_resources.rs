use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::event::FfxivEventQueue;
use crate::jobs_skill_data::bard::abilities::{get_song_skill_ids, make_bard_skill_list};
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_info::StatusInfo;
use crate::{ComboType, IdType, ResourceType, TimeType};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const ENERGY_MAX_STACK: ResourceType = 2;
const IFRIT_MAX_STACK: ResourceType = 1;
const TITAN_MAX_STACK: ResourceType = 1;
const GARUDA_MAX_STACK: ResourceType = 1;
const TRANCE_MAX_STACK: ResourceType = 1;

#[derive(Clone)]
pub(crate) struct SummonerCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: IdType,
    current_combo: ComboType,
    energy_stack: ResourceType,
    ifrit_stack: ResourceType,
    titan_stack: ResourceType,
    garuda_stack: ResourceType,
    trance_stack: ResourceType,
}

impl CombatResource for SummonerCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: IdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            self.energy_stack = min(ENERGY_MAX_STACK, self.energy_stack + resource_amount);
        } else if resource_id == 1 {
            self.ifrit_stack = min(IFRIT_MAX_STACK, self.ifrit_stack + resource_amount);
        } else if resource_id == 2 {
            self.titan_stack = min(TITAN_MAX_STACK, self.titan_stack + resource_amount);
        } else if resource_id == 3 {
            self.garuda_stack = min(self.garuda_stack, self.garuda_stack + resource_amount);
        } else if resource_id == 4 {
            self.trance_stack = min(TRANCE_MAX_STACK, self.trance_stack + resource_amount);
        }
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            self.energy_stack
        } else if resource_id == 1 {
            self.ifrit_stack
        } else if resource_id == 2 {
            self.titan_stack
        } else if resource_id == 3 {
            self.garuda_stack
        } else if resource_id == 4 {
            self.trance_stack
        } else {
            -1
        }
    }

    fn get_current_combo(&self) -> ComboType {
        self.current_combo
    }

    fn update_combo(&mut self, combo: &Option<IdType>) {
        if let Some(combo) = combo {
            self.current_combo = Some(*combo);
        }
    }

    fn trigger_on_event(
        &mut self,
        _: IdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        _: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        (vec![], vec![])
    }

    fn trigger_on_crit(&mut self) {}

    fn get_next_buff_target(&self, _: IdType) -> IdType {
        0
    }
    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl SummonerCombatResources {
    pub(crate) fn new(player_id: IdType, ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>) -> Self {
        Self {
            skills: make_bard_skill_list(player_id, ffxiv_event_queue),
            player_id,
            current_combo: None,
            energy_stack: 0,
            ifrit_stack: 0,
            titan_stack: 0,
            garuda_stack: 0,
            trance_stack: 0,
        }
    }
}
