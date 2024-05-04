use crate::id_entity::IdEntity;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::job_priorities::job_abilities::ninja_abilities::{
    bunshin_gcd_ids, get_bunshin_stack, make_ninja_gcd_table, make_ninja_ogcd_table,
    make_ninja_opener, make_ninja_skill_list,
};
use crate::rotation::job_priorities::SkillTable;
use crate::rotation::priority_table::PriorityTable;
use crate::rotation::{FfxivPriorityTable, SkillPriorityInfo};
use crate::skill::attack_skill::{AttackSkill, SkillInfo};
use crate::{IdType, ResourceType, StackType, TurnCount};
use ffxiv_simbot_db::ffxiv_context::FfxivContext;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct NinjaPriorityTable {
    turn_count: TurnCount,
    skills: SkillTable,

    opener: Vec<Option<AttackSkill>>,

    gcd_priority_table: Vec<SkillPriorityInfo<AttackSkill>>,
    ogcd_priority_table: Vec<SkillPriorityInfo<AttackSkill>>,

    ninki: RefCell<ResourceType>,
    bunshin_count: RefCell<ResourceType>,

    current_combo: Option<IdType>,
}

impl PriorityTable<FfxivPlayer, AttackSkill> for NinjaPriorityTable {
    fn add_resource1(&self, resource: ResourceType) {
        *self.ninki.borrow_mut() += resource;
    }

    fn add_resource2(&self, resource: ResourceType) {
        *self.bunshin_count.borrow_mut() += resource;
    }

    fn update_combo(&mut self, combo_id: Option<IdType>) {
        self.current_combo = combo_id;
    }

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
        player: &FfxivPlayer,
    ) -> Vec<SkillInfo<AttackSkill>> {
        let skill_info = &skills[0];
        let skill_id = skill_info.skill.get_id();
        let mut skills = skills.clone();

        if bunshin_gcd_ids().contains(&skill_id) && self.get_resource(1) > 0 {
            self.add_resource1(5);
            self.add_resource2(-1);
            skills.push(self.make_skill_info(get_bunshin_stack(player.get_id()), player));
        }

        skills
    }

    fn get_skills_mut(&mut self) -> &mut SkillTable {
        &mut self.skills
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            *self.ninki.borrow()
        } else {
            *self.bunshin_count.borrow()
        }
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
            ninki: RefCell::new(0),
            bunshin_count: RefCell::new(0),
            current_combo: None,
        }
    }
}

impl FfxivPlayer {
    pub fn new_ninja(
        player_id: IdType,
        power: CharacterPower,
        context: &FfxivContext,
    ) -> FfxivPlayer {
        let ninja_job = context.jobs.get("NIN").unwrap();

        Self::new(
            player_id,
            ninja_job.clone(),
            power,
            FfxivPriorityTable::Ninja(NinjaPriorityTable::new(player_id)),
        )
    }
}

impl CooldownTimer for NinjaPriorityTable {
    fn update_cooldown(&mut self, elapsed_time: i32) {
        for (_, skill) in self.skills.iter_mut() {
            skill.update_cooldown(elapsed_time);
        }
    }
}
