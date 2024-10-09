import {
  DEFAULT_CRITICAL_STRIKE,
  DEFAULT_DETERMINATION,
  DEFAULT_DIRECT_HIT,
  DEFAULT_SPEED,
  DEFAULT_TENACITY,
} from "../../const/StatValue";
import {
  calculateGCD,
  calculateHasteBuff,
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
import { SingleEquipmentInputSaveState } from "../EquipmentInput";
import { AppLanguageTexts } from "../../const/languageTexts";

export const loadPowerNames = () => {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  const POWER_NAMES = [
    LANGUAGE_TEXTS.WD_POWER_NAME,
    LANGUAGE_TEXTS.MAIN_STAT_POWER_NAME,
    LANGUAGE_TEXTS.CRT_RATE_POWER_NAME,
    LANGUAGE_TEXTS.CRT_POWER_NAME,
    LANGUAGE_TEXTS.DH_RATE_POWER_NAME,
    LANGUAGE_TEXTS.DET_POWER_NAME,
    LANGUAGE_TEXTS.SPEED_POWER_NAME,
    LANGUAGE_TEXTS.TENACITY_POWER_NAME,
    LANGUAGE_TEXTS.GCD_NAME,
  ]

  return { POWER_NAMES };
}


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
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (statName) {
    case LANGUAGE_TEXTS.WD_STAT_EN_NAME:
      return `${playerPower.weaponDamage}`;
    case LANGUAGE_TEXTS.STR_STAT_EN_NAME:
    case LANGUAGE_TEXTS.DEX_STAT_EN_NAME:
    case LANGUAGE_TEXTS.INT_STAT_EN_NAME:
    case LANGUAGE_TEXTS.MIND_STAT_EN_NAME:
      return `${playerPower.mainStat}`;
    case LANGUAGE_TEXTS.CRIT_STAT_EN_NAME:
      return `${playerPower.criticalStrike}`;
    case LANGUAGE_TEXTS.DH_STAT_EN_NAME:
      return `${playerPower.directHit}`;
    case LANGUAGE_TEXTS.DET_STAT_EN_NAME:
      return `${playerPower.determination}`;
    case LANGUAGE_TEXTS.SKS_STAT_EN_NAME:
      return `${playerPower.skillSpeed}`;
    case LANGUAGE_TEXTS.SPS_STAT_EN_NAME:
      return `${playerPower.spellSpeed}`;
    case LANGUAGE_TEXTS.TEN_STAT_EN_NAME:
      return `${playerPower.tenacity}`;
    case LANGUAGE_TEXTS.GCD_NAME: {
      playerPower.gcd = calculateGCD(
        playerPower.speedMultiplier,
        calculateHasteBuff(jobAbbrev)
      ) / 100;
      return `${playerPower.gcd.toFixed(2)}`;
    }
    default:
      return "";
  }
}

export function getStatPower(power: PlayerPower, powerName: string) {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (powerName) {
    case LANGUAGE_TEXTS.WD_POWER_NAME: {
      return `${(power.weaponDamageMultiplier * 100).toFixed(0)}%`;
    }
    case LANGUAGE_TEXTS.MAIN_STAT_POWER_NAME: {
      return `${(power.mainStatMultiplier * 100).toFixed(0)}%`;
    }
    case LANGUAGE_TEXTS.CRT_RATE_POWER_NAME: {
      return `+${(power.criticalStrikeRate * 100).toFixed(1)}%`;
    }
    case LANGUAGE_TEXTS.CRT_POWER_NAME: {
      return `+${(power.criticalStrikeDamage * 100).toFixed(1)}%`;
    }
    case LANGUAGE_TEXTS.DH_RATE_POWER_NAME: {
      return `${(power.directHitRate * 100).toFixed(1)}%`;
    }
    case LANGUAGE_TEXTS.DET_POWER_NAME: {
      return `${(100 * power.determinationMultiplier).toFixed(1)}%`;
    }
    case LANGUAGE_TEXTS.SPEED_POWER_NAME: {
      return `${(100 * power.speedMultiplier).toFixed(1)}%`;
    }
    case LANGUAGE_TEXTS.TENACITY_POWER_NAME: {
      return `${(power.tenacityMultiplier * 100).toFixed(1)}%`;
    }
    case LANGUAGE_TEXTS.GCD_NAME: {
      return `${power.gcd.toFixed(2)}`;
    }
    default:
      return "";
  }
}

export function isTank(jobAbbrev: string) {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (jobAbbrev) {
    case LANGUAGE_TEXTS.PLD_EN_NAME:
    case LANGUAGE_TEXTS.WAR_EN_NAME:
    case LANGUAGE_TEXTS.DRK_EN_NAME:
    case LANGUAGE_TEXTS.GNB_EN_NAME:
      return true;
    default:
      return false;
  }
}

export function isHealer(jobAbbrev: string) {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (jobAbbrev) {
    case LANGUAGE_TEXTS.WHM_EN_NAME:
    case LANGUAGE_TEXTS.SCH_EN_NAME:
    case LANGUAGE_TEXTS.AST_EN_NAME:
    case LANGUAGE_TEXTS.SGE_EN_NAME:
      return true;
    default:
      return false;
  }
}

function isMelee(jobAbbrev: string) {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (jobAbbrev) {
    case LANGUAGE_TEXTS.DRG_EN_NAME:
    case LANGUAGE_TEXTS.MNK_EN_NAME:
    case LANGUAGE_TEXTS.NIN_EN_NAME:
    case LANGUAGE_TEXTS.SAM_EN_NAME:
    case LANGUAGE_TEXTS.RPR_EN_NAME:
    case LANGUAGE_TEXTS.VPR_EN_NAME:
      return true;
    default:
      return false;
  }
}

function isPhysRanged(jobAbbrev: string) {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (jobAbbrev) {
    case LANGUAGE_TEXTS.BRD_EN_NAME:
    case LANGUAGE_TEXTS.MCH_EN_NAME:
    case LANGUAGE_TEXTS.DNC_EN_NAME:
      return true;
    default:
      return false;
  }
}

function isDpsCaster(jobAbbrev: string) {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (jobAbbrev) {
    case LANGUAGE_TEXTS.SMN_EN_NAME:
    case LANGUAGE_TEXTS.BLM_EN_NAME:
    case LANGUAGE_TEXTS.RDM_EN_NAME:
    case LANGUAGE_TEXTS.PCT_EN_NAME:
      return true;
    default:
      return false;
  }
}

export function setPartyCompositionBuffPercent(
  input: SingleEquipmentInputSaveState
) {
  let partyMembers = [...input.partyMemberJobAbbrevs];
  partyMembers.push(input.mainPlayerJobAbbrev);

  let hasMelee = false;
  let hasPhysRanged = false;
  let hasCaster = false;
  let hasTank = false;
  let hasHealer = false;

  for (let jobAbbrev of partyMembers) {
    if (isMelee(jobAbbrev)) {
      hasMelee = true;
    } else if (isPhysRanged(jobAbbrev)) {
      hasPhysRanged = true;
    } else if (isDpsCaster(jobAbbrev)) {
      hasCaster = true;
    } else if (isTank(jobAbbrev)) {
      hasTank = true;
    } else if (isHealer(jobAbbrev)) {
      hasHealer = true;
    }
  }

  let partyBuffPercent = 0;

  if (hasTank) {
    partyBuffPercent += 1;
  }
  if (hasHealer) {
    partyBuffPercent += 1;
  }
  if (hasMelee) {
    partyBuffPercent += 1;
  }
  if (hasPhysRanged) {
    partyBuffPercent += 1;
  }
  if (hasCaster) {
    partyBuffPercent += 1;
  }

  if (partyBuffPercent >= 3) {
    input.compositionBuffPercent = partyBuffPercent;
  } else {
    input.compositionBuffPercent = 0;
  }
}


export function getSpeedStatByJobAbbrev(
  totalStats: PlayerPower,
  jobAbbrev: string
) {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (jobAbbrev) {
    case LANGUAGE_TEXTS.PLD_EN_NAME:
    case LANGUAGE_TEXTS.WAR_EN_NAME:
    case LANGUAGE_TEXTS.DRK_EN_NAME:
    case LANGUAGE_TEXTS.GNB_EN_NAME:
    case LANGUAGE_TEXTS.DRK_EN_NAME:
    case LANGUAGE_TEXTS.MNK_EN_NAME:
    case LANGUAGE_TEXTS.SAM_EN_NAME:
    case LANGUAGE_TEXTS.RPR_EN_NAME:
    case LANGUAGE_TEXTS.NIN_EN_NAME:
    case LANGUAGE_TEXTS.VPR_EN_NAME:
    case LANGUAGE_TEXTS.BRD_EN_NAME:
    case LANGUAGE_TEXTS.MCH_EN_NAME:
    case LANGUAGE_TEXTS.DNC_EN_NAME:
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
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (statName) {
    case LANGUAGE_TEXTS.WD_STAT_EN_NAME:
      return 0;
    case LANGUAGE_TEXTS.STR_STAT_EN_NAME:
    case LANGUAGE_TEXTS.DEX_STAT_EN_NAME:
    case LANGUAGE_TEXTS.INT_STAT_EN_NAME:
    case LANGUAGE_TEXTS.MIND_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentMainStat(
          totalStats.mainStatMultiplier * 100,
          isTank(jobAbbrev)
        ) - totalStats.mainStat
      );
    case LANGUAGE_TEXTS.CRIT_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentCriticalStrike(
          100 * (totalStats.criticalStrikeDamage - CRIT_BASE_DAMAGE)
        ) - totalStats.criticalStrike
      );
    case LANGUAGE_TEXTS.DH_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentDirectHit(100 * totalStats.directHitRate) -
        totalStats.directHit
      );
    case LANGUAGE_TEXTS.DET_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentDetermination(
          totalStats.determinationMultiplier * 100 - 100
        ) - totalStats.determination
      );
    case LANGUAGE_TEXTS.SKS_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentSpeed(
          totalStats.speedMultiplier * 100 - 100
        ) - totalStats.skillSpeed
      );
    case LANGUAGE_TEXTS.SPS_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentSpeed(
          totalStats.speedMultiplier * 100 - 100
        ) - totalStats.spellSpeed
      );
    case LANGUAGE_TEXTS.TEN_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentTenacity(
          totalStats.tenacityMultiplier * 100 - 100
        ) - totalStats.tenacity
      );
    case LANGUAGE_TEXTS.GCD_NAME:
      return (
        Math.max(getMinNeededStatForCurrentGCD(totalStats.gcd, jobAbbrev), DEFAULT_SPEED) -
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
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (statName) {
    case LANGUAGE_TEXTS.WD_STAT_EN_NAME:
      return 0;
    case LANGUAGE_TEXTS.STR_STAT_EN_NAME:
    case LANGUAGE_TEXTS.DEX_STAT_EN_NAME:
    case LANGUAGE_TEXTS.INT_STAT_EN_NAME:
    case LANGUAGE_TEXTS.MIND_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentMainStat(
          totalStats.mainStatMultiplier * 100 + 1,
          isTank(jobAbbrev)
        ) - totalStats.mainStat
      );
    case LANGUAGE_TEXTS.CRIT_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentCriticalStrike(
          100 * (totalStats.criticalStrikeDamage - CRIT_BASE_DAMAGE) + 0.1
        ) - totalStats.criticalStrike
      );
    case LANGUAGE_TEXTS.DH_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentDirectHit(
          100 * totalStats.directHitRate + 0.1
        ) - totalStats.directHit
      );
    case LANGUAGE_TEXTS.DET_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentDetermination(
          totalStats.determinationMultiplier * 100 - 100 + 0.1
        ) - totalStats.determination
      );
    case LANGUAGE_TEXTS.SKS_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentSpeed(
          totalStats.speedMultiplier * 100 - 100 + 0.1
        ) - totalStats.skillSpeed
      );
    case LANGUAGE_TEXTS.SPS_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentSpeed(
          totalStats.speedMultiplier * 100 - 100 + 0.1
        ) - totalStats.spellSpeed
      );
    case LANGUAGE_TEXTS.TEN_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentTenacity(
          totalStats.tenacityMultiplier * 100 - 100 + 0.1
        ) - totalStats.tenacity
      );
    case LANGUAGE_TEXTS.GCD_NAME:
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
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (statName) {
    case LANGUAGE_TEXTS.WD_STAT_EN_NAME:
      return 0;
    case LANGUAGE_TEXTS.STR_STAT_EN_NAME:
    case LANGUAGE_TEXTS.DEX_STAT_EN_NAME:
    case LANGUAGE_TEXTS.INT_STAT_EN_NAME:
    case LANGUAGE_TEXTS.MIND_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentMainStat(
          totalStats.mainStatMultiplier * 100 + 1 * amount,
          isTank(jobAbbrev)
        ) - totalStats.mainStat
      );
    case LANGUAGE_TEXTS.CRIT_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentCriticalStrike(
          100 * (totalStats.criticalStrikeDamage - CRIT_BASE_DAMAGE) +
          0.1 * amount
        ) - totalStats.criticalStrike
      );
    case LANGUAGE_TEXTS.DH_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentDirectHit(
          100 * totalStats.directHitRate + 0.1 * amount
        ) - totalStats.directHit
      );
    case LANGUAGE_TEXTS.DET_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentDetermination(
          totalStats.determinationMultiplier * 100 - 100 + 0.1 * amount
        ) - totalStats.determination
      );
    case LANGUAGE_TEXTS.TEN_STAT_EN_NAME:
      return (
        getMinNeededStatForCurrentTenacity(
          totalStats.tenacityMultiplier * 100 - 100 + 0.1 * amount
        ) - totalStats.tenacity
      );
    case LANGUAGE_TEXTS.SKS_STAT_EN_NAME:
    case LANGUAGE_TEXTS.SPS_STAT_EN_NAME:
    case LANGUAGE_TEXTS.GCD_NAME:
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
