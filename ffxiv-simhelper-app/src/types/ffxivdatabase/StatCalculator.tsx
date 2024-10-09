import { AppLanguageTexts } from "../../const/languageTexts";
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

export const LoadJobRelatedStats = () => {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  const BASE_WEAPON_DAMAGE_PER_JOB = new Map([
    [LANGUAGE_TEXTS.PLD_EN_NAME, 44],
    [LANGUAGE_TEXTS.WAR_EN_NAME, 46],
    [LANGUAGE_TEXTS.DRK_EN_NAME, 46],
    [LANGUAGE_TEXTS.GNB_EN_NAME, 44],
    [LANGUAGE_TEXTS.WHM_EN_NAME, 50],
    [LANGUAGE_TEXTS.SCH_EN_NAME, 50],
    [LANGUAGE_TEXTS.AST_EN_NAME, 50],
    [LANGUAGE_TEXTS.SGE_EN_NAME, 50],
    [LANGUAGE_TEXTS.DRG_EN_NAME, 50],
    [LANGUAGE_TEXTS.MNK_EN_NAME, 48],
    [LANGUAGE_TEXTS.NIN_EN_NAME, 48],
    [LANGUAGE_TEXTS.SAM_EN_NAME, 49],
    [LANGUAGE_TEXTS.RPR_EN_NAME, 50],
    [LANGUAGE_TEXTS.VPR_EN_NAME, 48],
    [LANGUAGE_TEXTS.BRD_EN_NAME, 50],
    [LANGUAGE_TEXTS.MCH_EN_NAME, 50],
    [LANGUAGE_TEXTS.DNC_EN_NAME, 50],
    [LANGUAGE_TEXTS.BLM_EN_NAME, 50],
    [LANGUAGE_TEXTS.SMN_EN_NAME, 50],
    [LANGUAGE_TEXTS.RDM_EN_NAME, 50],
    [LANGUAGE_TEXTS.PCT_EN_NAME, 50],
  ]);

  return { BASE_WEAPON_DAMAGE_PER_JOB };
}


export function calculateHasteBuff(jobAbbrev: string) {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (jobAbbrev) {
    case LANGUAGE_TEXTS.NIN_EN_NAME:
      return NIN_GCD_SPEED_BUFF;
    case LANGUAGE_TEXTS.MNK_EN_NAME:
      return MNK_GCD_SPEED_BUFF;
    case LANGUAGE_TEXTS.VPR_EN_NAME:
      return VPR_GCD_SPEED_BUFF;
    case LANGUAGE_TEXTS.SAM_EN_NAME:
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