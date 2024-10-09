import { AST_EN_NAME, BLM_EN_NAME, BRD_EN_NAME, DNC_EN_NAME, DRG_EN_NAME, DRK_EN_NAME, GNB_EN_NAME, MCH_EN_NAME, MNK_EN_NAME, NIN_EN_NAME, PCT_EN_NAME, PLD_EN_NAME, RDM_EN_NAME, RPR_EN_NAME, SAM_EN_NAME, SCH_EN_NAME, SGE_EN_NAME, SMN_EN_NAME, VPR_EN_NAME, WAR_EN_NAME, WHM_EN_NAME } from "../../const/languageTexts";
import {
  DEFAULT_CRITICAL_STRIKE,
  DEFAULT_DETERMINATION,
  DEFAULT_DIRECT_HIT,
  DEFAULT_MAIN_STAT_NON_TANK,
  DEFAULT_MAIN_STAT_TANK,
  DEFAULT_SPEED,
  DEFAULT_TENACITY,
} from "../../const/StatValue";

const DEFAULT_DIVIDEND = 2780.0;
export const DEFAULT_GCD = 250;

export const NIN_GCD_SPEED_BUFF = 85;
export const MNK_GCD_SPEED_BUFF = 80;
export const VPR_GCD_SPEED_BUFF = 85;
export const SAM_GCD_SPEED_BUFF = 87;

export const AUTO_DH_SLOPE = 140;

const MAIN_STAT_SLOPE_NON_TANK = 237 / 440;
const MAIN_STAT_SLOPE_TANK = 190 / 440;
const CRIT_SLOPE = 200;
const DH_SLOPE = 550;
const DET_SLOPE = 140;
const SPEED_SLOPE = 130;
const TENACITY_SLOPE = 112;

export const BASE_WEAPON_DAMAGE_PER_JOB = new Map([
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


export function calculateHasteBuff(jobAbbrev: string) {
  switch (jobAbbrev) {
    case NIN_EN_NAME:
      return NIN_GCD_SPEED_BUFF;
    case MNK_EN_NAME:
      return MNK_GCD_SPEED_BUFF;
    case VPR_EN_NAME:
      return VPR_GCD_SPEED_BUFF;
    case SAM_EN_NAME:
      return SAM_GCD_SPEED_BUFF;
    default:
      return 100;
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


// calculate GCD in percent. ex) GCD 1.94 -> 194
export function calculateGCD(
  speedMultiplier: number,
  hasteBuffPercent: number
) {
  let gcdMultiplier = 2 - speedMultiplier;

  return Math.floor(Math.floor(DEFAULT_GCD * gcdMultiplier * 10) * hasteBuffPercent / 1000)
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

export function getMinNeededStatForStatLadder(
  currentIncreasePercent: number,
  slope: number,
  baseValue: number
) {
  currentIncreasePercent = Math.round(currentIncreasePercent * 10) / 10;
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
  let hasteBuff = calculateHasteBuff(jobAbbrev);

  // multiplier * 1000
  let speedIncrease = 0;

  while (speedIncrease < 500) {
    let nextSpeedLadderStat = getMinNeededStatForCurrentSpeed(speedIncrease / 10);
    let nextGCD = calculateGCD(1 + speedIncrease / 1000, hasteBuff) / 100;

    if (nextGCD <= currentGCD) {
      return nextSpeedLadderStat;
    }

    speedIncrease += 1;
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