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
} from "../../const/languageTexts";
import {
  DEFAULT_CRITICAL_STRIKE,
  DEFAULT_DETERMINATION,
  DEFAULT_DIRECT_HIT,
  DEFAULT_MAIN_STAT_NON_TANK,
  DEFAULT_MAIN_STAT_TANK,
  DEFAULT_SPEED,
  DEFAULT_TENACITY,
} from "../../const/StatValue";
import { PlayerPower } from "./PlayerPower";

const DEFAULT_DIVIDEND = 2780.0;
export const DEFAULT_GCD = 250;

export const NIN_GCD_SPEED_BUFF = 0.85;
export const MNK_GCD_SPEED_BUFF = 0.8;
export const VPR_GCD_SPEED_BUFF = 0.85;
export const SAM_GCD_SPEED_BUFF = 0.87;

export const AUTO_DH_SLOPE = 140;

const MAIN_STAT_SLOPE_NON_TANK = 237 / 440;
const MAIN_STAT_SLOPE_TANK = 190 / 440;
const CRIT_SLOPE = 200;
const DH_SLOPE = 550;
const DET_SLOPE = 140;
const SPEED_SLOPE = 130;
const TENACITY_SLOPE = 112;

const BASE_WEAPON_DAMAGE_PER_JOB = new Map([
  [PLD_EN_NAME, 44],
  [WAR_EN_NAME, 46],
  [DRK_EN_NAME, 46],
  [GNB_EN_NAME, 44],
  [WHM_EN_NAME, 50],
  [SCH_EN_NAME, 50],
  [AST_EN_NAME, 50],
  [SGE_EN_NAME, 50],
  [DRG_EN_NAME, 50],
  [MNK_EN_NAME, 48],
  [NIN_EN_NAME, 48],
  [SAM_EN_NAME, 49],
  [RPR_EN_NAME, 50],
  [VPR_EN_NAME, 48],
  [BRD_EN_NAME, 50],
  [MCH_EN_NAME, 50],
  [DNC_EN_NAME, 50],
  [BLM_EN_NAME, 50],
  [SMN_EN_NAME, 50],
  [RDM_EN_NAME, 50],
  [PCT_EN_NAME, 50],
]);

export function calculateModifiedGCD(gcd: number, jobAbbrev: string) {
  switch (jobAbbrev) {
    case NIN_EN_NAME:
      return gcd * NIN_GCD_SPEED_BUFF;
    case MNK_EN_NAME:
      return gcd * MNK_GCD_SPEED_BUFF;
    case VPR_EN_NAME:
      return gcd * VPR_GCD_SPEED_BUFF;
    case SAM_EN_NAME:
      return gcd * SAM_GCD_SPEED_BUFF;
    default:
      return gcd;
  }
}

// returns the percent increase of a stat
export function calculateMultiplierPercentIncrease(
  stat: number,
  baseValue: number,
  slope: number
) {
  return Math.floor((slope * (stat - baseValue)) / DEFAULT_DIVIDEND) / 10;
}


export function calculateGCDMultiplierPercentIncrease(
  stat: number,
  baseValue: number,
  slope: number
) {
  return Math.floor((slope * (baseValue - stat)) / DEFAULT_DIVIDEND) / 10;
}

export function calculateWeaponMultiplierPercent(
  weaponDamage: number,
  jobAbbrev: string
) {
  let baseWeaponDamage = BASE_WEAPON_DAMAGE_PER_JOB.get(jobAbbrev);
  if (baseWeaponDamage === undefined || weaponDamage === 0) {
    return 0;
  }
  return baseWeaponDamage + weaponDamage;
}

export function calculateMainStatPercentIncrease(
  mainStat: number,
  isTank: boolean
) {
  if (isTank) {
    return Math.floor(
      MAIN_STAT_SLOPE_TANK * (mainStat - DEFAULT_MAIN_STAT_TANK)
    );
  }
  return Math.floor(
    MAIN_STAT_SLOPE_NON_TANK * (mainStat - DEFAULT_MAIN_STAT_NON_TANK)
  );
}

export function calculateCriticalStrikePercentIncrease(critStat: number) {
  return calculateMultiplierPercentIncrease(
    critStat,
    DEFAULT_CRITICAL_STRIKE,
    CRIT_SLOPE
  );
}

export function calculateDirectHitPercentIncrease(dhStat: number) {
  return calculateMultiplierPercentIncrease(
    dhStat,
    DEFAULT_DIRECT_HIT,
    DH_SLOPE
  );
}
export function calculateDeterminationPercentIncrease(detStat: number) {
  return calculateMultiplierPercentIncrease(
    detStat,
    DEFAULT_DETERMINATION,
    DET_SLOPE
  );
}
export function calculateTenacityPercentIncrease(tenacityStat: number) {
  return calculateMultiplierPercentIncrease(
    tenacityStat,
    DEFAULT_TENACITY,
    TENACITY_SLOPE
  );
}

export function calculateSpeedPercentIncrease(speedStat: number) {
  return calculateMultiplierPercentIncrease(
    speedStat,
    DEFAULT_SPEED,
    SPEED_SLOPE
  );
}

// MNK Speed doesn't match the formula. Just hard coding it.
export function MONK_SKS_TIER(gcd: number) {
  switch (gcd) {
    case 200:
      return 420;
    case 199:
      return 442;
    case 198:
      return 527;
    case 197:
      return 656;
    case 196:
      return 741;
    case 195:
      return 870;
    case 194:
      return 955;
    case 193:
      return 1083;
    case 192:
      return 1169;
    case 191:
      return 1297;
    case 190:
      return 1383;
    case 189:
      return 1511;
    case 188:
      return 1597;
    case 187:
      return 1725;
    case 186:
      return 1810;
    case 185:
      return 1939;
    case 184:
      return 2024;
    case 183:
      return 2153;
    case 182:
      return 2238;
    case 181:
      return 2366;
    case 180:
      return 2452;
    case 179:
      return 2580;
    case 178:
      return 2666;
    case 177:
      return 2794;
    default:
      return 2880;

  }
}


export function calculateGCDPercentIncrease(speedStat: number) {
  return calculateGCDMultiplierPercentIncrease(
    speedStat,
    DEFAULT_SPEED,
    SPEED_SLOPE
  );
}

export function calculateGCDByMultiplier(speedMultiplier: number) {
  return Math.floor(DEFAULT_GCD / speedMultiplier) / 100;
}

export function calculateGCD(power: PlayerPower, jobAbbrev: string) {
  return (
    Math.floor(calculateModifiedGCD(DEFAULT_GCD, jobAbbrev) * power.gcdMultiplier) /
    100
  );
}

export function getMinNeededStatForStatLadder(
  currentIncreasePercent: number,
  slope: number,
  baseValue: number
) {
  if (currentIncreasePercent === 0) {
    return baseValue;
  }
  return Math.ceil(
    ((currentIncreasePercent * 10) / slope) * DEFAULT_DIVIDEND + baseValue
  );
}

export function getMinNeededStatForCurrentMainStat(
  currentMultiplierPercent: number,
  isTank: boolean
) {
  let baseValue = isTank ? DEFAULT_MAIN_STAT_TANK : DEFAULT_MAIN_STAT_NON_TANK;
  let slope = isTank ? MAIN_STAT_SLOPE_TANK : MAIN_STAT_SLOPE_NON_TANK;
  return Math.ceil((currentMultiplierPercent - 100) / slope + baseValue);
}

export function getMinNeededStatForCurrentCriticalStrike(
  currentIncreasePercent: number
) {
  return getMinNeededStatForStatLadder(
    currentIncreasePercent,
    CRIT_SLOPE,
    DEFAULT_CRITICAL_STRIKE
  );
}

export function getMinNeededStatForCurrentDirectHit(
  currentIncreasePercent: number
) {
  return getMinNeededStatForStatLadder(
    currentIncreasePercent,
    DH_SLOPE,
    DEFAULT_DIRECT_HIT
  );
}

export function getMinNeededStatForCurrentDetermination(
  currentIncreasePercent: number
) {
  return getMinNeededStatForStatLadder(
    currentIncreasePercent,
    DET_SLOPE,
    DEFAULT_DETERMINATION
  );
}

export function getMinNeededStatForCurrentTenacity(currentPercent: number) {
  return getMinNeededStatForStatLadder(
    currentPercent,
    TENACITY_SLOPE,
    DEFAULT_TENACITY
  );
}

export function getMinNeededStatForCurrentSpeed(currentPercent: number) {
  return getMinNeededStatForStatLadder(
    currentPercent,
    SPEED_SLOPE,
    DEFAULT_SPEED
  );
}

export function getMinNeededStatForCurrentGCD(
  currentGCD: number,
  jobAbbrev: string
) {
  if (jobAbbrev === MNK_EN_NAME) {
    return MONK_SKS_TIER(currentGCD * 100);
  }

  let base_gcd = calculateModifiedGCD(DEFAULT_GCD, jobAbbrev);
  let defaultGcdSeconds = base_gcd / 100;
  if (currentGCD >= defaultGcdSeconds) {
    return DEFAULT_SPEED;
  }

  let speedMultiplier = 0;

  while (speedMultiplier < 500) {
    let nextSpeedLadderStat = getMinNeededStatForCurrentSpeed(speedMultiplier / 10);
    let nextGCDMultiplier = 1 + calculateGCDPercentIncrease(nextSpeedLadderStat) / 100;

    let nextGCD = Math.floor(base_gcd * nextGCDMultiplier) / 100;

    if (nextGCD <= currentGCD) {
      return nextSpeedLadderStat;
    }

    speedMultiplier += 1;
  }

  return DEFAULT_SPEED;
}

export function calculateAutoDirectHitIncrease(directHit: number) {
  return (
    Math.floor(
      ((directHit - DEFAULT_DIRECT_HIT) * AUTO_DH_SLOPE) / DEFAULT_DIVIDEND
    ) / 1000
  );
}