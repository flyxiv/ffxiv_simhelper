use crate::jobs::FfxivPriorityTable;
use crate::player::{FfxivPlayer, Player};
use crate::priority_table::PriorityTable;
use crate::skill::{AttackSkill, SkillInfo};
use crate::status::{BuffStatus, DebuffStatus};
use crate::turn_type::FfxivTurnType;
use crate::{IdType, TurnCount};
use ffxiv_simbot_db::ffxiv_context::FfxivContext;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;

lazy_static! {
    static ref DOT: AttackSkill = AttackSkill {
        id: 0,
        name: String::from("DOT"),
        player_id: 0,
        potency: 750,
        trait_multiplier: 1.3,

        buff: None,
        debuff: None,
        is_gcd: false,
        turn_type: FfxivTurnType::Gcd,
        delay_millisecond: None,
        is_modified: false,
        cooldown_millisecond: 30000,
        resource_required: vec![],
    };
    static ref GCD: AttackSkill = AttackSkill {
        id: 1,
        name: String::from("GCD"),
        player_id: 0,
        potency: 330,
        trait_multiplier: 1.3,
        buff: None,
        debuff: None,
        is_gcd: true,
        turn_type: FfxivTurnType::Gcd,
        delay_millisecond: None,
        is_modified: false,
        cooldown_millisecond: 2500,
        resource_required: vec![],
    };
}

impl Default for SagePriorityTable {
    fn default() -> Self {
        SagePriorityTable {
            turn_count: 0,
            dot_skill: DOT.clone(),
            gcd_skill: GCD.clone(),
        }
    }
}

#[derive(Clone)]
pub struct SagePriorityTable {
    turn_count: TurnCount,
    dot_skill: AttackSkill,
    gcd_skill: AttackSkill,
}

impl PriorityTable<AttackSkill> for SagePriorityTable {
    fn get_next_skill<P>(
        &self,
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
