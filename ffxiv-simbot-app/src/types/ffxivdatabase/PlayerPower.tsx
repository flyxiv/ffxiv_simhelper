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

export const WEAPON_MULTIPLIER_NAME = "Weapon";
export const MAIN_STAT_MULTIPLIER_NAME = "Main Stat";
export const CRIT_RATE_NAME = "Crit Rate";
export const CRIT_DAMAGE_NAME = "Crit Dmg";
export const DH_RATE_NAME = "DH Rate";
export const DET_MULTIPLIER_NAME = "Det";
export const SPEED_MULTIPLIER_NAME = "Speed";
export const TENACITY_MULTIPLIER_NAME = "Tenacity";
export const GCD_NAME = "GCD";

export const POWER_NAMES = [WEAPON_MULTIPLIER_NAME, MAIN_STAT_MULTIPLIER_NAME, CRIT_RATE_NAME, CRIT_DAMAGE_NAME, DH_RATE_NAME, DET_MULTIPLIER_NAME, SPEED_MULTIPLIER_NAME, TENACITY_MULTIPLIER_NAME, GCD_NAME];

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
    case "WD":
      return `${playerPower.weaponDamage}`;
    case "STR":
    case "DEX":
    case "INT":
    case "MND":
      return `${playerPower.mainStat}`;
    case "CRT":
      return `${playerPower.criticalStrike}`;
    case "DH":
      return `${playerPower.directHit}`;
    case "DET":
      return `${playerPower.determination}`;
    case "SKS":
      return `${playerPower.skillSpeed}`;
    case "SPS":
      return `${playerPower.spellSpeed}`;
    case "TEN":
      return `${playerPower.tenacity}`;
    case "GCD": {
      playerPower.gcd = calculateGCD(
        getSpeedStatByJobAbbrev(playerPower, jobAbbrev)
      );
      return `${playerPower.gcd.toFixed(2)}`;
    }
    default: return "";
  }
}

export function getStatPower(power: PlayerPower, powerName: string) {
  switch (powerName) {
    case WEAPON_MULTIPLIER_NAME: {
      return `${(power.weaponDamageMultiplier * 100).toFixed(0)}%`;
    }
    case MAIN_STAT_MULTIPLIER_NAME: {
      return `${(power.mainStatMultiplier * 100).toFixed(0)}%`;
    }
    case CRIT_RATE_NAME: {
      return `+${(power.criticalStrikeRate * 100).toFixed(1)}%`;
    }
    case CRIT_DAMAGE_NAME: {
      return `+${(power.criticalStrikeDamage * 100).toFixed(1)}%`;

    }
    case DH_RATE_NAME: {
      return `${(power.directHitRate * 100).toFixed(1)}%`;
    }
    case DET_MULTIPLIER_NAME: {
      return `${(100 * power.determinationMultiplier).toFixed(1)}%`;
    }
    case SPEED_MULTIPLIER_NAME: {
      return `${(100 * power.speedMultiplier).toFixed(1)}%`;
    }
    case TENACITY_MULTIPLIER_NAME: {
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
    case "PLD":
    case "WAR":
    case "DRK":
    case "GNB":
      return true;
    default:
      return false;
  }
}

function getSpeedStatByJobAbbrev(totalStats: PlayerPower, jobAbbrev: string) {
  switch (jobAbbrev) {
    case "PLD":
    case "WAR":
    case "DRK":
    case "GNB":
    case "DRG":
    case "MNK":
    case "SAM":
    case "RPR":
    case "NIN":
    case "VPR":
    case "BRD":
    case "MCH":
    case "DNC":
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
    case "WD":
      return 0;
    case "STR":
    case "DEX":
    case "INT":
    case "MND":
      return (
        getMinNeededStatForCurrentMainStat(
          totalStats.mainStatMultiplier * 100,
          isTank(jobAbbrev)
        ) - totalStats.mainStat
      );
    case "CRT":
      return (
        getMinNeededStatForCurrentCriticalStrike(
          100 * (totalStats.criticalStrikeDamage - CRIT_BASE_DAMAGE)
        ) - totalStats.criticalStrike
      );
    case "DH":
      return (
        getMinNeededStatForCurrentDirectHit(100 * totalStats.directHitRate) -
        totalStats.directHit
      );
    case "DET":
      return (
        getMinNeededStatForCurrentDetermination(
          totalStats.determinationMultiplier * 100 - 100
        ) - totalStats.determination
      );
    case "SKS":
      return (
        getMinNeededStatForCurrentSpeed(
          totalStats.speedMultiplier * 100 - 100
        ) - totalStats.skillSpeed
      );
    case "SPS":
      return (
        getMinNeededStatForCurrentSpeed(
          totalStats.speedMultiplier * 100 - 100
        ) - totalStats.spellSpeed
      );
    case "TEN":
      return (
        getMinNeededStatForCurrentTenacity(
          totalStats.tenacityMultiplier * 100 - 100
        ) - totalStats.tenacity
      );
    case "GCD":
      return (
        getMinNeededStatForCurrentGCD(totalStats.gcd) -
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
    case "WD":
      return 0;
    case "STR":
    case "DEX":
    case "INT":
    case "MND":
      return (
        getMinNeededStatForCurrentMainStat(
          totalStats.mainStatMultiplier * 100 + 1,
          isTank(jobAbbrev)
        ) - totalStats.mainStat
      );
    case "CRT":
      return (
        getMinNeededStatForCurrentCriticalStrike(
          100 * (totalStats.criticalStrikeDamage - CRIT_BASE_DAMAGE) + 0.1
        ) - totalStats.criticalStrike
      );
    case "DH":
      return (
        getMinNeededStatForCurrentDirectHit(
          100 * totalStats.directHitRate + 0.1
        ) - totalStats.directHit
      );
    case "DET":
      return (
        getMinNeededStatForCurrentDetermination(
          totalStats.determinationMultiplier * 100 - 100 + 0.1
        ) - totalStats.determination
      );
    case "SKS":
      return (
        getMinNeededStatForCurrentSpeed(
          totalStats.speedMultiplier * 100 - 100 + 0.1
        ) - totalStats.skillSpeed
      );
    case "SPS":
      return (
        getMinNeededStatForCurrentSpeed(
          totalStats.speedMultiplier * 100 - 100
        ) - totalStats.spellSpeed
      );
    case "TEN":
      return (
        getMinNeededStatForCurrentTenacity(
          totalStats.tenacityMultiplier * 100 - 100 + 0.1
        ) - totalStats.tenacity
      );
    case "GCD":
      return (
        getMinNeededStatForCurrentGCD(totalStats.gcd - 0.01) -
        getSpeedStatByJobAbbrev(totalStats, jobAbbrev)
      );
    default:
      return -1;
  }
}
