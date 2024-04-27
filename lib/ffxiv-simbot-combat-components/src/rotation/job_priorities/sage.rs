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

lazy_static! {
    static ref DOT: AttackSkill = AttackSkill {
        id: 0,
        name: String::from("Eukrasian Dosis III"),
        player_id: 0,
        potency: 750,
        trait_multiplier: 1.3,
        buff: None,
        debuff: None,
        combo: None,
        turn_type: FfxivTurnType::Gcd,
        delay_millisecond: None,
        cooldown_millisecond: 30000,
        resource_required: vec![],
        current_cooldown_millisecond: 0,
        stacks: 1,
    };
    static ref GCD: AttackSkill = AttackSkill {
        id: 1,
        name: String::from("Dosis III"),
        player_id: 0,
        potency: 330,
        trait_multiplier: 1.3,
        buff: None,
        debuff: None,
        combo: None,
        turn_type: FfxivTurnType::Gcd,
        delay_millisecond: None,
        cooldown_millisecond: 0,
        resource_required: vec![],
        current_cooldown_millisecond: 0,
        stacks: 1,
    };
    static ref PHLEGMA: AttackSkill = AttackSkill {
        id: 2,
        name: String::from("Phlegma III"),
        player_id: 0,
        potency: 600,
        trait_multiplier: 1.3,
        buff: None,
        debuff: None,
        combo: None,
        turn_type: FfxivTurnType::Gcd,
        delay_millisecond: None,
        cooldown_millisecond: 40000,
        resource_required: vec![],
        current_cooldown_millisecond: 0,
        stacks: 2,
    };
    static ref SAGE_OPENER: Vec<Option<AttackSkill>> = vec![
        Some(GCD.clone()),
        None,
        None,
        Some(DOT.clone()),
        None,
        None,
        Some(GCD.clone()),
        None,
        None,
        Some(GCD.clone()),
    ];
    static ref SAGE_PRIORITY_LIST: Vec<SkillPriorityInfo> = vec![
        SkillPriorityInfo {
            skill: DOT.clone(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill: PHLEGMA.clone(),
            prerequisite: Some(SkillPrerequisite::Or(
                SkillPrerequisite::HasStacks(2),
                SkillPrerequisite::IsBurst
            )),
        },
        SkillPriorityInfo {
            skill: GCD.clone(),
            prerequisite: None,
        },
    ];
}

impl Default for SagePriorityTable {
    fn default() -> Self {
        SagePriorityTable {
            turn_count: 0,
            skills: vec![DOT.clone(), GCD.clone(), PHLEGMA.clone()],
            opener: SAGE_OPENER.clone(),
            priority_list: SAGE_PRIORITY_LIST.clone(),
            current_combo: None,
        }
    }
}

#[derive(Clone)]
pub struct SagePriorityTable {
    turn_count: TurnCount,

    skills: Vec<AttackSkill>,
    opener: Vec<Option<AttackSkill>>,
    priority_list: Vec<SkillPriorityInfo>,
    current_combo: Option<IdType>,
}

impl PriorityTable<FfxivPlayer, AttackSkill> for SagePriorityTable {
    fn get_highest_priority_skill<P>(
        &mut self,
        _: Rc<RefCell<Vec<BuffStatus>>>,
        _: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &P,
    ) -> Option<SkillInfo<AttackSkill>>
    where
        P: Player,
    {
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
