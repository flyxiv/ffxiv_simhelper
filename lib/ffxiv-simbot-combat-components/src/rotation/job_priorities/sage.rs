use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::player::Player;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::priority_table::PriorityTable;
use crate::rotation::{FfxivPriorityTable, SkillPrerequisite, SkillPriorityInfo};
use crate::skill::attack_skill::{AttackSkill, SkillInfo};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{IdType, ResourceType, StackType, TurnCount};
use ffxiv_simbot_db::ffxiv_context::FfxivContext;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct SagePriorityTable {
    turn_count: TurnCount,

    skills: Vec<AttackSkill>,
    opener: Vec<Option<AttackSkill>>,
    priority_list: Vec<SkillPriorityInfo>,
    current_combo: Option<IdType>,
}

impl PriorityTable<FfxivPlayer, AttackSkill> for SagePriorityTable {
    fn get_highest_priority_skill(
        &mut self,
        _: Rc<RefCell<Vec<BuffStatus>>>,
        _: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &FfxivPlayer,
        ffxiv_turn_type: &FfxivTurnType,
    ) -> Option<SkillInfo<AttackSkill>> {
        let mut skill_info = SkillInfo {
            skill: GCD.clone(),
            guaranteed_critical_hit: false,
            guaranteed_direct_hit: false,
        };

        match player.get_turn_type() {
            FfxivTurnType::Gcd => {
                if self.dot_skill.cooldown_millisecond == 0 {
                    skill_info.skill = self.dot_skill.clone();
                    Some(skill_info)
                } else {
                    Some(skill_info)
                }
            }
            _ => None,
        }
    }

    fn use_opener(&self, player_turn: &FfxivTurnType) -> Option<SkillInfo<AttackSkill>> {
        todo!()
    }

    fn use_highest_priority_skill<P>(
        &mut self,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &P,
    ) -> Option<SkillInfo<AttackSkill>> {
        todo!()
    }

    fn get_skills_mut(&mut self) -> &mut Vec<AttackSkill> {
        todo!()
    }

    fn start_cooldown(&mut self, skill_info: &Option<SkillInfo<AttackSkill>>) {
        todo!()
    }

    fn get_current_combo(&self) -> Option<IdType> {
        self.current_combo
    }

    fn get_opener(&self) -> Option<SkillInfo<AttackSkill>> {
        todo!()
    }

    fn get_resource(&self) -> ResourceType {
        todo!()
    }

    fn get_skill_stack(&self, skill_id: IdType) -> StackType {
        todo!()
    }

    fn get_priority_table(&self, turn_type: &FfxivTurnType) -> &Vec<SkillPriorityInfo> {
        todo!()
    }

    fn is_guaranteed_crit(&self, skill: &AttackSkill) -> bool {
        todo!()
    }

    fn is_guaranteed_direct_hit(&self, skill: &AttackSkill) -> bool {
        todo!()
    }
}

impl FfxivPlayer {
    pub fn new_sage(id: IdType, power: CharacterPower, context: &FfxivContext) -> FfxivPlayer {
        let sage_job = context.jobs.get("SGE").unwrap();

        Self::new(
            id,
            sage_job.clone(),
            power,
            FfxivPriorityTable::Sage(SagePriorityTable::default()),
        )
    }
}
