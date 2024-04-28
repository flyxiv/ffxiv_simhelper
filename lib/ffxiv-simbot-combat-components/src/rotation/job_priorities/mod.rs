use crate::skill::attack_skill::AttackSkill;
use crate::IdType;
use std::collections::HashMap;

pub(crate) mod job_abilities;
pub(crate) mod ninja;
pub(crate) mod sage;

pub(crate) type SkillTable = HashMap<IdType, AttackSkill>;
