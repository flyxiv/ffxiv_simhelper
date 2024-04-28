use crate::id_entity::IdEntity;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::Player;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::job_priorities::job_abilities::ninja_abilities::{
    bunshin_gcd_ids, get_bunshin_stack, make_ninja_gcd_table, make_ninja_ogcd_table,
    make_ninja_opener, make_ninja_skill_list,
};
use crate::rotation::job_priorities::SkillTable;
use crate::rotation::priority_table::{PriorityTable, SkillResult};
use crate::rotation::SkillPriorityInfo;
use crate::skill::attack_skill::{AttackSkill, SkillInfo};
use crate::skill::ResourceRequirements;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{IdType, ResourceType, StackType, TurnCount};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct NinjaPriorityTable {
    turn_count: TurnCount,
    skills: SkillTable,

    opener: Vec<Option<AttackSkill>>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,

    ninki: ResourceType,
    bunshin_count: ResourceType,

    current_combo: Option<IdType>,
}

impl PriorityTable<FfxivPlayer, AttackSkill> for NinjaPriorityTable {
    fn update_stack_status(
        &mut self,
        skill: &AttackSkill,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
    ) {
        self.ninki += skill.resource1_created;
        self.bunshin_count += skill.resource2_created;

        self.current_combo = skill.combo;

        for resource in skill.resource_required {
            match resource {
                ResourceRequirements::StackResource1(required_resource) => {
                    self.ninki -= required_resource;
                }
                ResourceRequirements::CheckStatus(status_id) => {
                    for debuff in debuff_list.borrow_mut().iter_mut() {
                        if debuff.get_id() == status_id {
                            debuff.duration_left_millisecond = 0;
                        }
                    }

                    for buff in buff_list.borrow_mut().iter_mut() {
                        if buff.get_id() == status_id {
                            buff.duration_left_millisecond = 0;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn is_opener(&self) -> bool {
        self.turn_count < self.opener.len()
    }

    fn get_opener(&mut self, player: &FfxivPlayer) -> Option<SkillResult<AttackSkill>> {
        let skill = &self.opener[self.turn_count];
        self.turn_count += 1;

        if let Some(skill) = skill {
            Some(SkillResult::UseSkill(vec![SkillInfo {
                skill: skill.clone(),
                damage_inflict_time_millisecond: player.get_damage_inflict_time_millisecond(skill),
                guaranteed_critical_hit: false,
                guaranteed_direct_hit: false,
            }]))
        } else {
            None
        }
    }

    fn add_additional_skills(
        &mut self,
        skill: AttackSkill,
        player: &FfxivPlayer,
    ) -> Vec<AttackSkill> {
        let skill_id = skill.get_id();

        if bunshin_gcd_ids().contains(&skill_id) && self.bunshin_count > 0 {
            self.bunshin_count -= 1;
            self.ninki += 5;
            vec![skill, get_bunshin_stack(player.get_id())]
        } else {
            vec![skill]
        }
    }

    fn get_skills_mut(&mut self) -> &mut SkillTable {
        &mut self.skills
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            self.ninki
        } else {
            self.bunshin_count
        }
    }

    fn get_skill_stack(&self, skill_id: IdType) -> StackType {
        let skill = self.skills.get(&skill_id).unwrap();

        skill.stacks
    }

    fn get_priority_table(&self, turn_type: &FfxivTurnType) -> &Vec<SkillPriorityInfo> {
        match turn_type {
            FfxivTurnType::Gcd => &self.gcd_priority_table,
            _ => &self.ogcd_priority_table,
        }
    }

    fn is_guaranteed_crit(&self, skill: &AttackSkill) -> bool {
        false
    }

    fn is_guaranteed_direct_hit(&self, skill: &AttackSkill) -> bool {
        false
    }

    fn get_current_combo(&self) -> Option<IdType> {
        self.current_combo
    }
}

impl NinjaPriorityTable {
    pub fn new(player_id: IdType) -> Self {
        Self {
            turn_count: 0,
            skills: make_ninja_skill_list(player_id),
            opener: make_ninja_opener(player_id),
            gcd_priority_table: make_ninja_gcd_table(player_id),
            ogcd_priority_table: make_ninja_ogcd_table(player_id),
            ninki: 0,
            bunshin_count: 0,
            current_combo: None,
        }
    }
}
