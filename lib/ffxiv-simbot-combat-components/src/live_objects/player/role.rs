#[derive(Hash, Copy, Clone, Eq, PartialEq)]
pub enum Role {
    Tank,
    Healer,
    MeleeDps,
    RangedDps,
    Caster,
}

pub fn job_abbrev_to_role(job_abbrev: &String) -> Role {
    match job_abbrev.as_str() {
        "PLD" | "WAR" | "DRK" | "GNB" => Role::Tank,
        "WHM" | "SCH" | "AST" | "SGE" => Role::Healer,
        "DRG" | "MNK" | "NIN" | "SAM" | "RPR" | "VPR" => Role::MeleeDps,
        "BRD" | "MCH" | "DNC" => Role::RangedDps,
        _ => Role::Caster,
    }
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Tank => String::from("Tank"),
            Role::Healer => String::from("Healer"),
            Role::MeleeDps => String::from("MeleeDps"),
            Role::RangedDps => String::from("RangedDps"),
            Role::Caster => String::from("Caster"),
        }
    }
}
