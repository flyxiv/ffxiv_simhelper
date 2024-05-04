use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::job_priorities::job_abilities::sage_abilities::{
    make_sage_gcd_priority_table, make_sage_opener, make_sage_skills,
};
use crate::rotation::job_priorities::SkillTable;
use crate::rotation::priority_table::PriorityTable;
use crate::rotation::{FfxivPriorityTable, SkillPriorityInfo};
use crate::skill::attack_skill::{AttackSkill, SkillInfo};
use crate::{IdType, ResourceType, StackType, TurnCount};
use ffxiv_simbot_db::ffxiv_context::FfxivContext;
use ffxiv_simbot_db::stat_calculator::CharacterPower;

#[derive(Clone)]
pub struct SagePriorityTable {
    turn_count: TurnCount,
    skills: SkillTable,
    opener: Vec<Option<AttackSkill>>,
    gcd_priority_list: Vec<SkillPriorityInfo<AttackSkill>>,
    ogcd_priority_list: Vec<SkillPriorityInfo<AttackSkill>>,
}

impl PriorityTable<FfxivPlayer, AttackSkill> for SagePriorityTable {
    fn add_resource1(&self, _: ResourceType) {}

    fn add_resource2(&self, _: ResourceType) {}

    fn update_combo(&mut self, _: Option<IdType>) {}

    fn get_opener_len(&self) -> usize {
        self.opener.len()
    }

    fn get_opener_at(&self, index: usize) -> &Option<AttackSkill> {
        &self.opener[index]
    }

    fn get_turn_count(&self) -> IdType {
        self.turn_count
    }

    fn increment_turn(&mut self) {
        self.turn_count += 1;
    }

    fn add_additional_skills(
        &self,
        skills: &Vec<SkillInfo<AttackSkill>>,
        _: &FfxivPlayer,
    ) -> Vec<SkillInfo<AttackSkill>> {
        skills.clone()
    }

    fn get_skills_mut(&mut self) -> &mut SkillTable {
        &mut self.skills
    }

    fn get_resource(&self, _: IdType) -> ResourceType {
        0
    }

    fn get_skill_stack(&self, skill_id: IdType) -> StackType {
        let skill = self.skills.get(&skill_id).unwrap();
        skill.stacks
    }

    fn get_priority_table(
        &self,
        turn_type: &FfxivTurnType,
    ) -> &Vec<SkillPriorityInfo<AttackSkill>> {
        match turn_type {
            FfxivTurnType::Gcd => &self.gcd_priority_list,
            _ => &self.ogcd_priority_list,
        }
    }

    fn is_guaranteed_crit(&self, _: &AttackSkill) -> bool {
        false
    }

    fn is_guaranteed_direct_hit(&self, _: &AttackSkill) -> bool {
        false
    }

    fn get_current_combo(&self) -> Option<IdType> {
        None
    }
}

impl SagePriorityTable {
    pub fn new(player_id: IdType) -> Self {
        Self {
            turn_count: 0,
            skills: make_sage_skills(player_id),
            opener: make_sage_opener(player_id),
            gcd_priority_list: make_sage_gcd_priority_table(player_id),
            ogcd_priority_list: Vec::new(),
        }
    }
}

impl FfxivPlayer {
    pub fn new_sage(
        player_id: IdType,
        power: CharacterPower,
        context: &FfxivContext,
    ) -> FfxivPlayer {
        let sage_job = context.jobs.get("SGE").unwrap();

        Self::new(
            player_id,
            sage_job.clone(),
            power,
            FfxivPriorityTable::Sage(SagePriorityTable::new(player_id)),
        )
    }
}

impl CooldownTimer for SagePriorityTable {
    fn update_cooldown(&mut self, elapsed_time: i32) {
        for (_, skill) in self.skills.iter_mut() {
            skill.update_cooldown(elapsed_time);
        }
    }
}
