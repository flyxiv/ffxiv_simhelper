import { CURRENT_MAX_ITEM_LEVEL } from "../types/ffxivdatabase/Equipment";
import { calculatePowerByStat, isCaster } from "../types/ffxivdatabase/ItemSet";
import { defaultPlayerPower } from "../types/ffxivdatabase/PlayerPower";
import {
  AST_EN_NAME,
  BLM_EN_NAME,
  BRD_EN_NAME,
  DNC_EN_NAME,
  DRG_EN_NAME,
  DRK_EN_NAME,
  GNB_EN_NAME,
  MCH_EN_NAME,
  MNK_EN_NAME,
  NIN_EN_NAME,
  PCT_EN_NAME,
  PLD_EN_NAME,
  RDM_EN_NAME,
  RPR_EN_NAME,
  SAM_EN_NAME,
  SCH_EN_NAME,
  SGE_EN_NAME,
  SMN_EN_NAME,
  VPR_EN_NAME,
  WAR_EN_NAME,
  WHM_EN_NAME,
} from "./languageTexts";

export const DEFAULT_WEAPON_DAMAGE = 100;
export const DEFAULT_MAIN_STAT_NON_TANK = 440;
export const DEFAULT_MAIN_STAT_TANK = 440;
export const DEFAULT_CRITICAL_STRIKE = 420;
export const DEFAULT_DIRECT_HIT = 420;
export const DEFAULT_DETERMINATION = 440;
export const DEFAULT_SPEED = 420;
export const DEFAULT_TENACITY = 420;

export function calculateIlvlAdjustment(partyiLvl: number) {
  return 1 - (CURRENT_MAX_ITEM_LEVEL - partyiLvl) / 100;
}

export const mapJobAbbrevToJobDefaultStat = (jobAbbrev: string) => {
  switch (jobAbbrev) {
    case PLD_EN_NAME:
      return PLD_BIS_STATS;
    case WAR_EN_NAME:
      return WAR_BIS_STATS;
    case DRK_EN_NAME:
      return DRK_BIS_STATS;
    case GNB_EN_NAME:
      return GNB_BIS_STATS;
    case WHM_EN_NAME:
      return WHM_BIS_STATS;
    case SCH_EN_NAME:
      return SCH_BIS_STATS;
    case AST_EN_NAME:
      return AST_BIS_STATS;
    case SGE_EN_NAME:
      return SGE_BIS_STATS;
    case DRG_EN_NAME:
      return DRG_BIS_STATS;
    case MNK_EN_NAME:
      return MNK_BIS_STATS;
    case NIN_EN_NAME:
      return NIN_BIS_STATS;
    case SAM_EN_NAME:
      return SAM_BIS_STATS;
    case RPR_EN_NAME:
      return RPR_BIS_STATS;
    case VPR_EN_NAME:
      return VPR_BIS_STATS;
    case BRD_EN_NAME:
      return BRD_BIS_STATS;
    case DNC_EN_NAME:
      return DNC_BIS_STATS;
    case MCH_EN_NAME:
      return MCH_BIS_STATS;
    case BLM_EN_NAME:
      return BLM_BIS_STATS;
    case SMN_EN_NAME:
      return SMN_BIS_STAT;
    case RDM_EN_NAME:
      return RDM_BIS_STATS;
    case PCT_EN_NAME:
      return PCT_BIS_STATS;
    default:
      Error("Invalid job abbreviation");
  }
};

export interface PlayerStats {
  weaponDamage: number;
  mainStat: number;
  criticalStrike: number;
  directHit: number;
  determination: number;
  speed: number;
  tenacity: number;
}

export const PLD_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4820,
  criticalStrike: 3174,
  directHit: 1470,
  determination: 2310,
  speed: 420,
  tenacity: 868,
};

export const WAR_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4842,
  criticalStrike: 3174,
  directHit: 1470,
  determination: 2310,
  speed: 420,
  tenacity: 868,
};

export const DRK_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 5084,
  criticalStrike: 3174,
  directHit: 1470,
  determination: 2310,
  speed: 420,
  tenacity: 868,
};

export const GNB_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4820,
  criticalStrike: 3174,
  directHit: 1470,
  determination: 2310,
  speed: 420,
  tenacity: 868,
};

export const WHM_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4886,
  criticalStrike: 3147,
  directHit: 1320,
  determination: 2803,
  speed: 420,
  tenacity: DEFAULT_TENACITY,
};

export const SCH_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4886,
  criticalStrike: 3090,
  directHit: 636,
  determination: 2329,
  speed: 1251,
  tenacity: DEFAULT_TENACITY,
};

export const AST_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4886,
  criticalStrike: 2922,
  directHit: 1158,
  determination: 3043,
  speed: 420,
  tenacity: DEFAULT_TENACITY,
};

export const SGE_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4886,
  criticalStrike: 3041,
  directHit: 906,
  determination: 2831,
  speed: 900,
  tenacity: DEFAULT_TENACITY,
};

export const DRG_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4886,
  criticalStrike: 3120,
  directHit: 2132,
  determination: 2150,
  speed: DEFAULT_SPEED,
  tenacity: DEFAULT_TENACITY,
};

export const MNK_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4860,
  criticalStrike: 3156,
  directHit: 1639,
  determination: 2071,
  speed: 956,
  tenacity: DEFAULT_TENACITY,
};

export const NIN_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4861,
  criticalStrike: 3173,
  directHit: 1842,
  determination: 2387,
  speed: 420,
  tenacity: DEFAULT_TENACITY,
};

export const SAM_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4872,
  criticalStrike: 3103,
  directHit: 1933,
  determination: 2093,
  speed: 693,
  tenacity: DEFAULT_TENACITY,
};

export const RPR_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4882,
  criticalStrike: 3120,
  directHit: 2078,
  determination: 2150,
  speed: 474,
  tenacity: DEFAULT_TENACITY,
};
export const VPR_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4861,
  criticalStrike: 3173,
  directHit: 1734,
  determination: 2387,
  speed: 528,
  tenacity: DEFAULT_TENACITY,
};

export const BRD_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4886,
  criticalStrike: 3177,
  directHit: 2134,
  determination: 2091,
  speed: 420,
  tenacity: DEFAULT_TENACITY,
};

export const DNC_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4886,
  criticalStrike: 3177,
  directHit: 2134,
  determination: 2091,
  speed: 420,
  tenacity: DEFAULT_TENACITY,
};

export const MCH_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4886,
  criticalStrike: 3177,
  directHit: 2134,
  determination: 2091,
  speed: 420,
  tenacity: DEFAULT_TENACITY,
};

export const BLM_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4882,
  criticalStrike: 3193,
  directHit: 1666,
  determination: 1751,
  speed: 1212,
  tenacity: DEFAULT_TENACITY,
};

export const SMN_BIS_STAT = {
  weaponDamage: 146,
  mainStat: 4883,
  criticalStrike: 3061,
  directHit: 2125,
  determination: 2108,
  speed: 528,
  tenacity: DEFAULT_TENACITY,
};

export const RDM_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4883,
  criticalStrike: 3149,
  directHit: 1993,
  determination: 2269,
  speed: 420,
  tenacity: DEFAULT_TENACITY,
};

export const PCT_BIS_STATS = {
  weaponDamage: 146,
  mainStat: 4883,
  criticalStrike: 3140,
  directHit: 1993,
  determination: 2269,
  speed: 420,
  tenacity: DEFAULT_TENACITY,
};

export function playerStatToPlayerPower(
  playerStats: PlayerStats,
  jobAbbrev: string
) {
  let power = defaultPlayerPower();
  power.weaponDamage = playerStats.weaponDamage;
  power.mainStat = playerStats.mainStat;
  power.criticalStrike = playerStats.criticalStrike;
  power.directHit = playerStats.directHit;
  power.determination = playerStats.determination;

  if (isCaster(jobAbbrev)) {
    power.spellSpeed = playerStats.speed;
  } else {
    power.skillSpeed = playerStats.speed;
  }
  power.tenacity = playerStats.tenacity;

  calculatePowerByStat(power, jobAbbrev);
  return power;
}
