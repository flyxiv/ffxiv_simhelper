use crate::id_entity::IdEntity;
use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::job_priorities::job_abilities::dragoon_abilities::{
    get_dragoon_crit_skill_ids, make_dragoon_gcd_table, make_dragoon_ogcd_table,
    make_dragoon_opener, make_dragoon_skill_list,
};
use crate::rotation::job_priorities::SkillTable;
use crate::rotation::priority_table::PriorityTable;
use crate::rotation::{FfxivPriorityTable, SkillPriorityInfo};
use crate::skill::attack_skill::{AttackSkill, SkillInfo};
use crate::{IdType, ResourceType, StackType, TimeType, TurnCount};
use ffxiv_simbot_db::ffxiv_context::FfxivContext;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use std::cell::RefCell;

static DRAGOON_START_TIME_MILLISECOND: TimeType = 0;

#[derive(Clone)]
pub(crate) struct DragoonPriorityTable {
    turn_count: TurnCount,
    skills: SkillTable<AttackSkill>,

    opener: Vec<Option<AttackSkill>>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,

    mirage_gauge: RefCell<ResourceType>,
    firstmind_focus: RefCell<ResourceType>,

    current_combo: Option<IdType>,
}

impl PriorityTable<FfxivPlayer, AttackSkill> for DragoonPriorityTable {
    fn add_buff_distribute_to(&self, _: &mut Vec<SkillInfo<AttackSkill>>, _: &FfxivPlayer) {}

    fn add_resource1(&self, resource: ResourceType) {
        *self.mirage_gauge.borrow_mut() += resource;
    }

    fn add_resource2(&self, resource: ResourceType) {
        *self.firstmind_focus.borrow_mut() += resource;
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
        _: &FfxivPlayer,
    ) -> Vec<SkillInfo<AttackSkill>> {
        skills.clone()
    }

    fn get_skills_mut(&mut self) -> &mut SkillTable<AttackSkill> {
        &mut self.skills
    }

    fn get_skills(&self) -> &SkillTable<AttackSkill> {
        &self.skills
    }

    fn get_resource(&self, resource_id: IdType) -> ResourceType {
        if resource_id == 0 {
            *self.mirage_gauge.borrow()
        } else {
            *self.firstmind_focus.borrow()
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
        get_dragoon_crit_skill_ids().contains(&skill.get_id())
    }

    fn is_guaranteed_direct_hit(&self, _: &AttackSkill) -> bool {
        false
    }

    fn get_current_combo(&self) -> Option<IdType> {
        self.current_combo
    }
}

impl DragoonPriorityTable {
    pub fn new(player_id: IdType) -> Self {
        Self {
            turn_count: 0,
            skills: make_dragoon_skill_list(player_id),
            opener: make_dragoon_opener(player_id),
            gcd_priority_table: make_dragoon_gcd_table(),
            ogcd_priority_table: make_dragoon_ogcd_table(),
            mirage_gauge: RefCell::new(0),
            firstmind_focus: RefCell::new(0),
            current_combo: None,
        }
    }
}

impl FfxivPlayer {
    pub fn new_dragoon(
        player_id: IdType,
        power: CharacterPower,
        context: &FfxivContext,
    ) -> FfxivPlayer {
        let dragoon_job = context.jobs.get("DRG").unwrap();

        Self::new(
            player_id,
            dragoon_job.clone(),
            power,
            FfxivPriorityTable::Dragoon(DragoonPriorityTable::new(player_id)),
            DRAGOON_START_TIME_MILLISECOND,
            vec![],
        )
    }
}

impl CooldownTimer for DragoonPriorityTable {
    fn update_cooldown(&mut self, elapsed_time: i32) {
        for (_, skill) in self.skills.iter_mut() {
            skill.update_cooldown(elapsed_time);
        }
    }
}
