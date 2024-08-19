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
use crate::types::{
    ComboType, IdType, PlayerIdType, ResourceIdType, ResourceType, SkillStackType, TimeType,
};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

const APEX_MAX_STACK: ResourceType = 20;
const WANDERER_MAX_STACK: ResourceType = 3;
const ARMY_MAX_STACK: ResourceType = 4;
const SONG_MAX_STACK: ResourceType = 3;
const RADIANT_MAX_STACK: ResourceType = 3;

#[derive(Clone)]
pub(crate) struct BardCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: PlayerIdType,
    // 1-20
    apex_stack: ResourceType,
    wanderer_stack: ResourceType,
    army_stack: ResourceType,
    song_stack: ResourceType,
    armys_muse: BuffStatus,
    radiant_stack: ResourceType,
}

impl CombatResource for BardCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        if resource_id == 0 {
            self.apex_stack = min(APEX_MAX_STACK, self.apex_stack + resource_amount);
        } else if resource_id == 1 {
            self.wanderer_stack = min(WANDERER_MAX_STACK, self.wanderer_stack + resource_amount);
        } else if resource_id == 2 {
            self.army_stack = min(ARMY_MAX_STACK, self.army_stack + resource_amount);
        } else if resource_id == 3 {
            self.song_stack = min(SONG_MAX_STACK, self.song_stack + resource_amount);
        } else if resource_id == 4 {
            self.radiant_stack = min(RADIANT_MAX_STACK, self.radiant_stack + resource_amount);
        }
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        if resource_id == 0 {
            self.apex_stack
        } else if resource_id == 1 {
            self.wanderer_stack
        } else if resource_id == 2 {
            self.army_stack
        } else if resource_id == 3 {
            self.song_stack
        } else if resource_id == 4 {
            self.radiant_stack
        } else {
            -1
        }
    }

    fn get_current_combo(&self) -> ComboType {
        None
    }

    fn update_combo(&mut self, _: &ComboType) {}

    fn trigger_on_event(
        &mut self,
        skill_id: IdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_combat_time_millisecond: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        let skill_internal_events = vec![];
        let mut skill_events = vec![];

        if get_song_skill_ids().contains(&skill_id) {
            if skill_id == 1312 {
                let army_paeon_stacks = self.army_stack;

                if army_paeon_stacks > 0 {
                    let mut muse = self.armys_muse.clone();
                    muse.stacks = army_paeon_stacks as SkillStackType;

                    skill_events.push(FfxivEvent::ApplyBuff(
                        self.player_id,
                        self.player_id,
                        muse,
                        10000,
                        10000,
                        current_combat_time_millisecond,
                    ));
                }
            }

            self.reset_song_stacks();
            self.song_stack = min(SONG_MAX_STACK, self.song_stack + 1);
        }

        (skill_events, skill_internal_events)
    }

    fn trigger_on_crit(&mut self) {}

    fn get_next_buff_target(&self, _: IdType) -> PlayerIdType {
        0
    }
    fn update_stack_timer(&mut self, _: TimeType) {}
}

impl BardCombatResources {
    fn reset_song_stacks(&mut self) {
        self.wanderer_stack = 0;
        self.army_stack = 0;
    }
}

impl BardCombatResources {
    pub(crate) fn new(
        player_id: PlayerIdType,
        ffxiv_event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) -> Self {
        Self {
            skills: make_bard_skill_list(player_id, ffxiv_event_queue),
            player_id,
            apex_stack: 0,
            wanderer_stack: 0,
            army_stack: 0,
            song_stack: 0,
            armys_muse: BuffStatus {
                id: 1313,
                name: String::from("Army's Muse"),
                owner_id: player_id,
                duration_left_millisecond: 0,
                status_info: vec![StatusInfo::SpeedByStack(vec![1, 2, 4, 12])],
                duration_millisecond: 10000,
                is_raidwide: false,
                stacks: 1,
                max_stacks: 4,
                trigger_proc_event_on_gcd: vec![],
            },
            radiant_stack: 0,
        }
    }
}
