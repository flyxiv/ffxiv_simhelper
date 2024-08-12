export const DEFAULT_WEAPON_DAMAGE = 100;
export const DEFAULT_MAIN_STAT_NON_TANK = 437;
export const DEFAULT_MAIN_STAT_TANK = 440;
export const DEFAULT_CRITICAL_STRIKE = 420;
export const DEFAULT_DIRECT_HIT = 420;
export const DEFAULT_DETERMINATION = 440;
export const DEFAULT_SPEED = 420;
export const DEFAULT_TENACITY = 420;

export const MapJobAbbrevToJobDefaultStat = (jobAbbrev: string) => {
  switch (jobAbbrev) {
    case "PLD":
      return PLD_BIS_STATS;
    case "WAR":
      return WAR_BIS_STATS;
    case "DRK":
      return DRK_BIS_STAT;
    case "GNB":
      return GNB_BIS_STATS;
    case "AST":
      return AST_BIS_STATS;
    case "SGE":
      return SGE_BIS_STATS;
    case "WHM":
      return WHM_BIS_STATS;
    case "SCH":
      return SCH_BIS_STAT;
    case "DRG":
      return DRG_BIS_STATS;
    case "NIN":
      return NIN_BIS_STATS;
    case "MNK":
      return MNK_BIS_STATS;
    case "SAM":
      return SAM_BIS_STATS;
    case "RPR":
      return RPR_BIS_STATS;
    case "BRD":
      return BRD_BIS_STATS;
    case "DNC":
      return DNC_BIS_STATS;
    case "MCH":
      return MCH_BIS_STAT;
    case "BLM":
      return BLM_BIS_STATS;
    case "SMN":
      return SMN_BIS_STAT;
    case "RDM":
      return RDM_BIS_STATS;
    default:
      Error("Invalid job abbreviation");
  }
};
export const PLD_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3311,
  criticalStrike: 2540,
  directHit: 976,
  determination: 2182,
  speed: 400,
  tenacity: 529,
};

export const WAR_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3330,
  criticalStrike: 2576,
  directHit: 940,
  determination: 2182,
  speed: 400,
  tenacity: 529,
};

export const WHM_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3365,
  criticalStrike: 2502,
  directHit: 1012,
  determination: 2047,
  speed: 664,
  tenacity: 400,
};

export const AST_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3369,
  criticalStrike: 2430,
  directHit: 400,
  determination: 2032,
  speed: 1350,
  tenacity: 400,
};

export const SGE_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3369,
  criticalStrike: 2502,
  directHit: 1012,
  determination: 2047,
  speed: 664,
  tenacity: 400,
};

export const DRG_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3379,
  criticalStrike: 2567,
  directHit: 1396,
  determination: 1870,
  speed: 400,
  tenacity: 400,
};

export const NIN_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3360,
  criticalStrike: 2554,
  directHit: 1582,
  determination: 1697,
  speed: 400,
  tenacity: 400,
};

export const BRD_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3379,
  criticalStrike: 2598,
  directHit: 1252,
  determination: 1885,
  speed: 479,
  tenacity: 400,
};

export const DNC_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3379,
  criticalStrike: 2557,
  directHit: 1288,
  determination: 1952,
  speed: 436,
  tenacity: 400,
};

export const BLM_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3375,
  criticalStrike: 2514,
  directHit: 1402,
  determination: 1493,
  speed: 824,
  tenacity: 400,
};

export const MNK_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3360,
  criticalStrike: 2647,
  directHit: 1202,
  determination: 1613,
  speed: 771,
  tenacity: 400,
};

export const SAM_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3367,
  criticalStrike: 2587,
  directHit: 1458,
  determination: 1680,
  speed: 508,
  tenacity: 400,
};

export const GNB_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3311,
  criticalStrike: 2576,
  directHit: 940,
  determination: 2182,
  speed: 400,
  tenacity: 529,
};

export const RDM_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3379,
  criticalStrike: 2514,
  directHit: 1616,
  determination: 1601,
  speed: 502,
  tenacity: 400,
};

export const RPR_BIS_STATS = {
  weaponDamage: 132,
  mainStat: 3379,
  criticalStrike: 2567,
  directHit: 1396,
  determination: 1870,
  speed: 400,
  tenacity: 400,
};

export const AST_BIS_STAT = {
  weaponDamage: 132,
  mainStat: 3369,
  criticalStrike: 2502,
  directHit: 904,
  determination: 2047,
  speed: 774,
  tenacity: 400,
};

export const SCH_BIS_STAT = {
  weaponDamage: 132,
  mainStat: 3369,
  criticalStrike: 2301,
  directHit: 400,
  determination: 2047,
  speed: 1479,
  tenacity: 400,
};

export const MCH_BIS_STAT = {
  weaponDamage: 132,
  mainStat: 3547,
  criticalStrike: 2557,
  directHit: 1432,
  determination: 1844,
  speed: 400,
  tenacity: 400,
};

export const SMN_BIS_STAT = {
  weaponDamage: 132,
  mainStat: 3379,
  criticalStrike: 2514,
  directHit: 1616,
  determination: 1601,
  speed: 502,
  tenacity: 400,
};

export const DRK_BIS_STAT = {
  weaponDamage: 132,
  mainStat: 3330,
  criticalStrike: 2576,
  directHit: 940,
  determination: 2182,
  speed: 400,
  tenacity: 529,
};
