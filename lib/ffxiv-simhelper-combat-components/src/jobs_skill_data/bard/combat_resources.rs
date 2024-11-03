use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent;
use crate::event::FfxivEventQueue;
use crate::jobs_skill_data::bard::abilities::make_bard_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_info::StatusInfo;
use crate::types::{
    ComboType, PlayerIdType, ResourceIdType, ResourceType, SkillIdType, SkillStackType, TimeType,
};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

lazy_static! {
    static ref BARD_SONG_SKILL_IDS: Vec<SkillIdType> = vec![1312, 1313, 1314];
}

const BARD_STACK_COUNT: usize = 5;

/// 1 of these stacks = 5 Apex Arrow points in real game.
const APEX_MAX_STACK: ResourceType = 20;

const WANDERER_MAX_STACK: ResourceType = 3;
const ARMY_MAX_STACK: ResourceType = 4;
const SONG_MAX_STACK: ResourceType = 3;
const RADIANT_MAX_STACK: ResourceType = 3;

const WANDERER_STACK_ID: ResourceIdType = 1;
const ARMY_STACK_ID: ResourceIdType = 2;
const SONG_STACK_ID: ResourceIdType = 3;

const RESOURCE_MAX_STACKS: [ResourceType; BARD_STACK_COUNT] = [
    APEX_MAX_STACK,
    WANDERER_MAX_STACK,
    ARMY_MAX_STACK,
    SONG_MAX_STACK,
    RADIANT_MAX_STACK,
];

#[derive(Clone)]
pub(crate) struct BardCombatResources {
    skills: SkillTable<AttackSkill>,
    player_id: PlayerIdType,

    resources: [ResourceType; BARD_STACK_COUNT],
    armys_muse: BuffStatus,
}

impl CombatResource for BardCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_amount: ResourceType) {
        let resource_id = resource_id as usize;

        self.resources[resource_id] = min(
            RESOURCE_MAX_STACKS[resource_id],
            self.resources[resource_id] + resource_amount,
        );
    }

    fn get_resource(&self, resource_id: ResourceIdType) -> ResourceType {
        self.resources[resource_id as usize]
    }

    fn get_current_combo(&self) -> ComboType {
        None
    }

    fn update_combo(&mut self, _: &ComboType) {}

    fn trigger_on_event(
        &mut self,
        skill_id: SkillIdType,
        _: Rc<RefCell<HashMap<StatusKey, BuffStatus>>>,
        _: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_combat_time_millisecond: TimeType,
        _: &FfxivPlayer,
    ) -> SkillEvents {
        let skill_internal_events = vec![];
        let mut skill_events = vec![];

        if BARD_SONG_SKILL_IDS.contains(&skill_id) {
            if skill_id == 1312 {
                let army_paeon_stacks = self.get_resource(ARMY_STACK_ID);

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
            self.add_resource(SONG_STACK_ID, 1);
        }

        (skill_events, skill_internal_events)
    }

    fn trigger_on_gcd_crit(&mut self) {}

    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }
    fn update_other_time_related_states(&mut self, _: TimeType) {}
    fn get_combo_remaining_time(&self) -> TimeType {
        0
    }
}

impl BardCombatResources {
    fn reset_song_stacks(&mut self) {
        self.resources[WANDERER_STACK_ID as usize] = 0;
        self.resources[ARMY_STACK_ID as usize] = 0;
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
            resources: [0; BARD_STACK_COUNT],
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
        }
    }
}
