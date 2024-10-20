use crate::combat_resources::CombatResource;
use crate::event::ffxiv_event::FfxivEvent::Damage;
use crate::jobs_skill_data::machinist::abilities::make_machinist_skill_list;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::StatusKey;
use crate::rotation::SkillTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::damage_category::DamageCategory;
use crate::skill::SkillEvents;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::snapshot_status::snapshot_status_infos;
use crate::types::{ComboType, PlayerIdType, PotencyType, ResourceIdType, ResourceType};
use crate::types::{SkillIdType, TimeType};
use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::rc::Rc;

const MACHINIST_STACKS_COUNT: usize = 2;

const MIN_QUEEN_BATTERY: PotencyType = 5;
const QUEEN_BASE_POTENCY: PotencyType = 900;
const QUEEN_DELAY_MILLISECOND: TimeType = 5000;
const QUEEN_POTENCY_PER_STACK: PotencyType = 180;
const HEAT_MAX: ResourceType = 100;
const BATTERY_MAX: ResourceType = 10;

const WILDFIRE_ID: SkillIdType = 1410;
const AUTOMATON_QUEEN_ID: SkillIdType = 1413;

const WILDFIRE_POTENCY_PER_STACK: PotencyType = 240;
const WILDFIRE_DELAY_MILLISECOND: TimeType = 10000;

const BATTERY_ID: ResourceIdType = 1;

const MACHINIST_MAX_STACKS: [ResourceType; MACHINIST_STACKS_COUNT] = [HEAT_MAX, BATTERY_MAX];

#[derive(Clone)]
pub(crate) struct MachinistCombatResources {
    skills: SkillTable<AttackSkill>,
    current_combo: ComboType,
    resources: [ResourceType; MACHINIST_STACKS_COUNT],
    queen_damage_incoming: Option<(PotencyType, TimeType)>,
    wildfire_damage_incoming: Option<(PotencyType, TimeType)>,
}

impl CombatResource for MachinistCombatResources {
    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn add_resource(&mut self, resource_id: ResourceIdType, resource_type: ResourceType) {
        let resource_id = resource_id as usize;
        self.resources[resource_id] = min(
            MACHINIST_MAX_STACKS[resource_id],
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
        debuff_list: Rc<RefCell<HashMap<StatusKey, DebuffStatus>>>,
        current_time_millisecond: TimeType,
        player: &FfxivPlayer,
    ) -> SkillEvents {
        let mut ffxiv_events = vec![];

        if self.wildfire_damage_incoming.is_some() {
            let (potency, delay) = self.wildfire_damage_incoming.unwrap();

            if delay == 0 {
                // if wildfire debuff expired, burst the stacked up damage.
                ffxiv_events.push(Damage(
                    player.get_id(),
                    WILDFIRE_ID,
                    potency,
                    120,
                    false,
                    false,
                    snapshot_status_infos(
                        &buff_list.borrow(),
                        &debuff_list.borrow(),
                        player.get_id(),
                    ),
                    DamageCategory::Direct,
                    false,
                    current_time_millisecond,
                ));
                self.wildfire_damage_incoming = None;
            } else {
                // if wildfire debuff exists and skill is a GCD skill, increase the wildfire potency.
                let skill = self.skills.get(&skill_id).unwrap();
                if skill.gcd_cooldown_millisecond > 0 {
                    self.wildfire_damage_incoming =
                        Some((potency + WILDFIRE_POTENCY_PER_STACK, delay));
                }
            }
        }

        if self.queen_damage_incoming.is_some() {
            let (potency, delay) = self.queen_damage_incoming.unwrap();

            if delay == 0 {
                ffxiv_events.push(Damage(
                    player.get_id(),
                    AUTOMATON_QUEEN_ID,
                    potency,
                    120,
                    false,
                    false,
                    snapshot_status_infos(
                        &buff_list.borrow(),
                        &debuff_list.borrow(),
                        player.get_id(),
                    ),
                    DamageCategory::Direct,
                    false,
                    current_time_millisecond,
                ));

                self.queen_damage_incoming = None;
            }
        }

        if skill_id == AUTOMATON_QUEEN_ID {
            // if Automaton Queen is used, consume all battery stacks and increase the potency depending on how much battery stack is used.
            let current_stack = self.get_resource(BATTERY_ID);
            let potency = QUEEN_BASE_POTENCY
                + QUEEN_POTENCY_PER_STACK * (current_stack as PotencyType - MIN_QUEEN_BATTERY);
            self.queen_damage_incoming = Some((potency, QUEEN_DELAY_MILLISECOND));

            self.resources[BATTERY_ID as usize] = 0;
        }

        if skill_id == WILDFIRE_ID {
            self.wildfire_damage_incoming = Some((0, WILDFIRE_DELAY_MILLISECOND));
        }

        return (ffxiv_events, vec![]);
    }

    fn trigger_on_gcd_crit(&mut self) {}

    fn get_next_buff_target(&self, _: SkillIdType) -> PlayerIdType {
        0
    }
    fn update_stack_timer(&mut self, elapsed_time: TimeType) {
        if let Some((potency, delay)) = self.queen_damage_incoming {
            self.queen_damage_incoming = Some((potency, max(delay - elapsed_time, 0)));
        }

        if let Some((potency, delay)) = self.wildfire_damage_incoming {
            self.wildfire_damage_incoming = Some((potency, max(delay - elapsed_time, 0)));
        }
    }
}

impl MachinistCombatResources {
    pub(crate) fn new(player_id: PlayerIdType) -> Self {
        Self {
            skills: make_machinist_skill_list(player_id),
            current_combo: None,
            resources: [0; MACHINIST_STACKS_COUNT],
            queen_damage_incoming: None,
            wildfire_damage_incoming: None,
        }
    }
}
