pub(crate) enum Role {
    Tank,
    Healer,
    MeleeDps,
    RangedDps,
    Caster,
}

pub(crate) fn job_abbrev_to_role(job_abbrev: &String) -> Role {
    match job_abbrev.as_str() {
        "PLD" | "WAR" | "DRK" | "GNB" => Role::Tank,
        "WHM" | "SCH" | "AST" | "SGE" => Role::Healer,
        "DRG" | "MNK" | "NIN" | "SAM" | "RPR" | "VPR" => Role::MeleeDps,
        "BRD" | "MCH" | "DNC" => Role::RangedDps,
        _ => Role::Caster,
    }
}
