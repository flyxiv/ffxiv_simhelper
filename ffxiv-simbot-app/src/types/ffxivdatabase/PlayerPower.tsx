import {
  DEFAULT_CRITICAL_STRIKE,
  DEFAULT_DETERMINATION,
  DEFAULT_DIRECT_HIT,
  DEFAULT_SPEED,
  DEFAULT_TENACITY,
} from "../../const/StatValue";
import {
  calculateGCD,
  DEFAULT_GCD,
  getMinNeededStatForCurrentCriticalStrike,
  getMinNeededStatForCurrentDetermination,
  getMinNeededStatForCurrentDirectHit,
  getMinNeededStatForCurrentGCD,
  getMinNeededStatForCurrentMainStat,
  getMinNeededStatForCurrentSpeed,
  getMinNeededStatForCurrentTenacity,
} from "./StatCalculator";
import { CRIT_BASE_DAMAGE } from "./Stats";
import { ItemSet } from "./ItemSet";
import { GearSetMaterias } from "./Materia";
import {
  AST_EN_NAME,
  BRD_EN_NAME,
  CRIT_STAT_NAME,
  CRT_POWER_NAME,
  CRT_RATE_POWER_NAME,
  DET_POWER_NAME,
  DET_STAT_NAME,
  DEX_STAT_NAME,
  DH_RATE_POWER_NAME,
  DH_STAT_NAME,
  DNC_EN_NAME,
  DRK_EN_NAME,
  GCD_NAME,
  GNB_EN_NAME,
  INT_STAT_NAME,
  MAIN_STAT_POWER_NAME,
  MCH_EN_NAME,
  MIND_STAT_NAME,
  MNK_EN_NAME,
  NIN_EN_NAME,
  PLD_EN_NAME,
  RPR_EN_NAME,
  SAM_EN_NAME,
  SCH_EN_NAME,
  SGE_EN_NAME,
  SKS_STAT_NAME,
  SPEED_POWER_NAME,
  SPS_STAT_NAME,
  STR_STAT_NAME,
  TEN_STAT_NAME,
  TENACITY_POWER_NAME,
  VPR_EN_NAME,
  WAR_EN_NAME,
  WD_POWER_NAME,
  WD_STAT_NAME,
  WHM_EN_NAME,
} from "../../const/languageTexts";

export const POWER_NAMES = [
  WD_POWER_NAME,
  MAIN_STAT_POWER_NAME,
  CRT_RATE_POWER_NAME,
  CRT_POWER_NAME,
  DH_RATE_POWER_NAME,
  DET_POWER_NAME,
  SPEED_POWER_NAME,
  TENACITY_POWER_NAME,
  GCD_NAME,
];

export interface PlayerPower {
  weaponDamage: number;
  mainStat: number;
  criticalStrike: number;
  directHit: number;
  determination: number;
  skillSpeed: number;
  spellSpeed: number;
  tenacity: number;
  piety: number;
  weaponDamageMultiplier: number;
  mainStatMultiplier: number;
  criticalStrikeRate: number;
  criticalStrikeDamage: number;
  directHitRate: number;
  determinationMultiplier: number;
  speedMultiplier: number;
  tenacityMultiplier: number;
  autoAttackDelays: number;

  // https://www.akhmorning.com/allagan-studies/stats/dh/#formulae
  // 6.2 introduced increased damage for guaranteed direct hits
  autoDirectHitIncrease: number;
  gcd: number;
}

export interface CharacterEquipmentsData {
  power: PlayerPower;
  itemSet: ItemSet;
  jobAbbrev: string;
  race: string;
  foodId: number;
  gearSetMaterias: GearSetMaterias;
}

export function defaultPlayerPower(): PlayerPower {
  return {
    weaponDamage: 0,
    mainStat: 0,
    criticalStrike: DEFAULT_CRITICAL_STRIKE,
    directHit: DEFAULT_DIRECT_HIT,
    determination: DEFAULT_DETERMINATION,
    skillSpeed: DEFAULT_SPEED,
    spellSpeed: DEFAULT_SPEED,
    tenacity: DEFAULT_TENACITY,
    piety: 0,
    weaponDamageMultiplier: 0,
    mainStatMultiplier: 0,
    criticalStrikeRate: 0,
    criticalStrikeDamage: 0,
    directHitRate: 0,
    determinationMultiplier: 0,
    speedMultiplier: 0,
    tenacityMultiplier: 0,
    autoDirectHitIncrease: 0,
    autoAttackDelays: 0,
    gcd: DEFAULT_GCD,
  };
}

export function getStatByStatName(
  playerPower: PlayerPower,
  statName: string,
  jobAbbrev: string
) {
  switch (statName) {
    case WD_STAT_NAME:
      return `${playerPower.weaponDamage}`;
    case STR_STAT_NAME:
    case DEX_STAT_NAME:
    case INT_STAT_NAME:
    case MIND_STAT_NAME:
      return `${playerPower.mainStat}`;
    case CRIT_STAT_NAME:
      return `${playerPower.criticalStrike}`;
    case DH_STAT_NAME:
      return `${playerPower.directHit}`;
    case DET_STAT_NAME:
      return `${playerPower.determination}`;
    case SKS_STAT_NAME:
      return `${playerPower.skillSpeed}`;
    case SPS_STAT_NAME:
      return `${playerPower.spellSpeed}`;
    case TEN_STAT_NAME:
      return `${playerPower.tenacity}`;
    case GCD_NAME: {
      playerPower.gcd = calculateGCD(
        getSpeedStatByJobAbbrev(playerPower, jobAbbrev),
        jobAbbrev
      );
      return `${playerPower.gcd.toFixed(2)}`;
    }
    default:
      return "";
  }
}

export function getStatPower(power: PlayerPower, powerName: string) {
  switch (powerName) {
    case WD_POWER_NAME: {
      return `${(power.weaponDamageMultiplier * 100).toFixed(0)}%`;
    }
    case MAIN_STAT_POWER_NAME: {
      return `${(power.mainStatMultiplier * 100).toFixed(0)}%`;
    }
    case CRT_RATE_POWER_NAME: {
      return `+${(power.criticalStrikeRate * 100).toFixed(1)}%`;
    }
    case CRT_POWER_NAME: {
      return `+${(power.criticalStrikeDamage * 100).toFixed(1)}%`;
    }
    case DH_RATE_POWER_NAME: {
      return `${(power.directHitRate * 100).toFixed(1)}%`;
    }
    case DET_POWER_NAME: {
      return `${(100 * power.determinationMultiplier).toFixed(1)}%`;
    }
    case SPEED_POWER_NAME: {
      return `${(100 * power.speedMultiplier).toFixed(1)}%`;
    }
    case TENACITY_POWER_NAME: {
      return `${(power.tenacityMultiplier * 100).toFixed(1)}%`;
    }
    case GCD_NAME: {
      return `${power.gcd.toFixed(2)}`;
    }
    default:
      return "";
  }
}

export function isTank(jobAbbrev: string) {
  switch (jobAbbrev) {
    case PLD_EN_NAME:
    case WAR_EN_NAME:
    case DRK_EN_NAME:
    case GNB_EN_NAME:
      return true;
    default:
      return false;
  }
}

export function isHealer(jobAbbrev: string) {
  switch (jobAbbrev) {
    case WHM_EN_NAME:
    case SCH_EN_NAME:
    case AST_EN_NAME:
    case SGE_EN_NAME:
      return true;
    default:
      return false;
  }
}

export function getSpeedStatByJobAbbrev(
  totalStats: PlayerPower,
  jobAbbrev: string
) {
  switch (jobAbbrev) {
    case PLD_EN_NAME:
    case WAR_EN_NAME:
    case DRK_EN_NAME:
    case GNB_EN_NAME:
    case DRK_EN_NAME:
    case MNK_EN_NAME:
    case SAM_EN_NAME:
    case RPR_EN_NAME:
    case NIN_EN_NAME:
    case VPR_EN_NAME:
    case BRD_EN_NAME:
    case MCH_EN_NAME:
    case DNC_EN_NAME:
      return totalStats.skillSpeed;
    default:
      return totalStats.spellSpeed;
  }
}

export function getStatLostByStatName(
  totalStats: PlayerPower,
  statName: string,
  jobAbbrev: string
) {
  switch (statName) {
    case WD_STAT_NAME:
      return 0;
    case STR_STAT_NAME:
    case DEX_STAT_NAME:
    case INT_STAT_NAME:
    case MIND_STAT_NAME:
      return (
        getMinNeededStatForCurrentMainStat(
          totalStats.mainStatMultiplier * 100,
          isTank(jobAbbrev)
        ) - totalStats.mainStat
      );
    case CRIT_STAT_NAME:
      return (
        getMinNeededStatForCurrentCriticalStrike(
          100 * (totalStats.criticalStrikeDamage - CRIT_BASE_DAMAGE)
        ) - totalStats.criticalStrike
      );
    case DH_STAT_NAME:
      return (
        getMinNeededStatForCurrentDirectHit(100 * totalStats.directHitRate) -
        totalStats.directHit
      );
    case DET_STAT_NAME:
      return (
        getMinNeededStatForCurrentDetermination(
          totalStats.determinationMultiplier * 100 - 100
        ) - totalStats.determination
      );
    case SKS_STAT_NAME:
      return (
        getMinNeededStatForCurrentSpeed(
          totalStats.speedMultiplier * 100 - 100
        ) - totalStats.skillSpeed
      );
    case SPS_STAT_NAME:
      return (
        getMinNeededStatForCurrentSpeed(
          totalStats.speedMultiplier * 100 - 100
        ) - totalStats.spellSpeed
      );
    case TEN_STAT_NAME:
      return (
        getMinNeededStatForCurrentTenacity(
          totalStats.tenacityMultiplier * 100 - 100
        ) - totalStats.tenacity
      );
    case GCD_NAME:
      return (
        getMinNeededStatForCurrentGCD(totalStats.gcd, jobAbbrev) -
        getSpeedStatByJobAbbrev(totalStats, jobAbbrev)
      );
    default:
      return -1;
  }
}

export function getStatNeededByStatName(
  totalStats: PlayerPower,
  statName: string,
  jobAbbrev: string
) {
  switch (statName) {
    case WD_STAT_NAME:
      return 0;
    case STR_STAT_NAME:
    case DEX_STAT_NAME:
    case INT_STAT_NAME:
    case MIND_STAT_NAME:
      return (
        getMinNeededStatForCurrentMainStat(
          totalStats.mainStatMultiplier * 100 + 1,
          isTank(jobAbbrev)
        ) - totalStats.mainStat
      );
    case CRIT_STAT_NAME:
      return (
        getMinNeededStatForCurrentCriticalStrike(
          100 * (totalStats.criticalStrikeDamage - CRIT_BASE_DAMAGE) + 0.1
        ) - totalStats.criticalStrike
      );
    case DH_STAT_NAME:
      return (
        getMinNeededStatForCurrentDirectHit(
          100 * totalStats.directHitRate + 0.1
        ) - totalStats.directHit
      );
    case DET_STAT_NAME:
      return (
        getMinNeededStatForCurrentDetermination(
          totalStats.determinationMultiplier * 100 - 100 + 0.1
        ) - totalStats.determination
      );
    case SKS_STAT_NAME:
      return (
        getMinNeededStatForCurrentSpeed(
          totalStats.speedMultiplier * 100 - 100 + 0.1
        ) - totalStats.skillSpeed
      );
    case SPS_STAT_NAME:
      return (
        getMinNeededStatForCurrentSpeed(
          totalStats.speedMultiplier * 100 - 100 + 0.1
        ) - totalStats.spellSpeed
      );
    case TEN_STAT_NAME:
      return (
        getMinNeededStatForCurrentTenacity(
          totalStats.tenacityMultiplier * 100 - 100 + 0.1
        ) - totalStats.tenacity
      );
    case GCD_NAME:
      return (
        getMinNeededStatForCurrentGCD(totalStats.gcd - 0.01, jobAbbrev) -
        getSpeedStatByJobAbbrev(totalStats, jobAbbrev)
      );
    default:
      return -1;
  }
}

export function getStatNeededByStatNameLadderAmount(
  totalStats: PlayerPower,
  statName: string,
  jobAbbrev: string,
  amount: number
) {
  switch (statName) {
    case WD_STAT_NAME:
      return 0;
    case STR_STAT_NAME:
    case DEX_STAT_NAME:
    case INT_STAT_NAME:
    case MIND_STAT_NAME:
      return (
        getMinNeededStatForCurrentMainStat(
          totalStats.mainStatMultiplier * 100 + 1 * amount,
          isTank(jobAbbrev)
        ) - totalStats.mainStat
      );
    case CRIT_STAT_NAME:
      return (
        getMinNeededStatForCurrentCriticalStrike(
          100 * (totalStats.criticalStrikeDamage - CRIT_BASE_DAMAGE) +
            0.1 * amount
        ) - totalStats.criticalStrike
      );
    case DH_STAT_NAME:
      return (
        getMinNeededStatForCurrentDirectHit(
          100 * totalStats.directHitRate + 0.1 * amount
        ) - totalStats.directHit
      );
    case DET_STAT_NAME:
      return (
        getMinNeededStatForCurrentDetermination(
          totalStats.determinationMultiplier * 100 - 100 + 0.1 * amount
        ) - totalStats.determination
      );
    case TEN_STAT_NAME:
      return (
        getMinNeededStatForCurrentTenacity(
          totalStats.tenacityMultiplier * 100 - 100 + 0.1 * amount
        ) - totalStats.tenacity
      );
    case SKS_STAT_NAME:
    case SPS_STAT_NAME:
    case GCD_NAME:
      return (
        getMinNeededStatForCurrentGCD(
          totalStats.gcd - 0.01 * amount,
          jobAbbrev
        ) - getSpeedStatByJobAbbrev(totalStats, jobAbbrev)
      );
    default:
      return -1;
  }
}
