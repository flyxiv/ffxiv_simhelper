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
export const AUTO_DH_SLOPE = 140;

const MAIN_STAT_SLOPE_NON_TANK = 237 / 440;
const MAIN_STAT_SLOPE_TANK = 190 / 440;
const CRIT_SLOPE = 200;
const DH_SLOPE = 550;
const DET_SLOPE = 140;
const SPEED_SLOPE = 130;
const TENACITY_SLOPE = 112;

const BASE_WEAPON_DAMAGE_PER_JOB = new Map([
  ["PLD", 44],
  ["WAR", 46],
  ["DRK", 46],
  ["GNB", 44],
  ["WHM", 50],
  ["SCH", 50],
  ["AST", 50],
  ["SGE", 50],
  ["DRG", 50],
  ["MNK", 48],
  ["NIN", 48],
  ["SAM", 49],
  ["RPR", 50],
  ["VPR", 48],
  ["BRD", 50],
  ["MCH", 50],
  ["DNC", 50],
  ["BLM", 50],
  ["SMN", 50],
  ["RDM", 50],
  ["PCT", 50],
]);

// returns the percent increase of a stat
export function calculateMultiplierPercentIncrease(
  stat: number,
  baseValue: number,
  slope: number
) {
  return Math.floor((slope * (stat - baseValue)) / DEFAULT_DIVIDEND) / 10;
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
export function calculateGCDByMultiplier(speedMultiplier: number) {
  return Math.floor(DEFAULT_GCD / speedMultiplier) / 100;
}

export function calculateGCD(speedStat: number) {
  let speedMultiplier = 1 + calculateSpeedPercentIncrease(speedStat) / 100;
  return Math.floor(DEFAULT_GCD / speedMultiplier) / 100;
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

export function getMinNeededStatForCurrentGCD(currentGCD: number) {
  let defaultGcdMinutes = DEFAULT_GCD / 100;
  if (currentGCD >= DEFAULT_GCD / 100) {
    return DEFAULT_SPEED;
  }
  let minimumNeededSpeedMultiplierPercentForGcd =
    Math.floor((defaultGcdMinutes / (currentGCD + 0.01) - 1) * 1000) / 10 + 0.1;
  return getMinNeededStatForCurrentSpeed(
    minimumNeededSpeedMultiplierPercentForGcd
  );
}

export function calculateAutoDirectHitIncrease(directHit: number) {
  return (
    Math.floor(
      ((directHit - DEFAULT_DIRECT_HIT) * AUTO_DH_SLOPE) / DEFAULT_DIVIDEND
    ) / 1000
  );
}
