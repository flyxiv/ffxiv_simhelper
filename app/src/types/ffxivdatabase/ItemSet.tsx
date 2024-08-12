import {
  DEFAULT_CRITICAL_STRIKE,
  DEFAULT_DETERMINATION,
  DEFAULT_DIRECT_HIT,
  DEFAULT_SPEED,
  DEFAULT_TENACITY,
} from "src/const/StatValue";
import {
  EMPTY_EQUIPMENT_ID,
  EQUIPMENT_DATABASE_BY_ID,
  TOTAL_SLOTS,
} from "./Equipment";
import { convertEquipmentToItemStat } from "./ItemStats";
import { getBaseMainStat } from "src/const/StartStats";
import {
  calculateCriticalStrikePercentIncrease,
  calculateDeterminationPercentIncrease,
  calculateDirectHitPercentIncrease,
  calculateGCD,
  calculateMainStatPercentMultiplier,
  calculateSpeedPercentIncrease,
  calculateTenacityPercentIncrease,
  calculateWeaponMultiplierPercent,
  getMinNeededStatForCurrentCriticalStrike,
  getMinNeededStatForCurrentDetermination,
  getMinNeededStatForCurrentDirectHit,
  getMinNeededStatForCurrentGCD,
  getMinNeededStatForCurrentMainStat,
  getMinNeededStatForCurrentSpeed,
  getMinNeededStatForCurrentTenacity,
} from "./StatCalculator";
import { FOOD_DATABASE } from "./Food";
import {
  updateMateriaValueStatToFinalStat,
  addMateriaStatToTotalStat,
  Materia,
} from "./Materia";

export type ItemSet = Map<string, number>;
export interface TotalStats {
  weaponDamage: number;
  mainStat: number;
  criticalStrike: number;
  directHit: number;
  determination: number;
  skillSpeed: number;
  spellSpeed: number;
  tenacity: number;
  piety: number;
  mainStatIncreasePercent: number;
  criticalStrikeIncreasePercent: number;
  directHitIncreasePercent: number;
  determinationIncreasePercent: number;
  speedIncreasePercent: number;
  tenacityIncreasePercent: number;
  gcd: number;
}

export function defaultItemSet(): ItemSet {
  let itemSet = new Map();

  TOTAL_SLOTS.forEach((slot) => {
    itemSet.set(slot, EMPTY_EQUIPMENT_ID);
  });

  return itemSet;
}

export function calculateTotalStatsOfItemSet(
  itemSet: ItemSet,
  jobAbbrev: string,
  race: string,
  foodId: number,
  materiasInEachSlot: Map<string, Materia[]>
): TotalStats {
  let totalStats: TotalStats = {
    weaponDamage: 0,
    mainStat: 0,
    criticalStrike: DEFAULT_CRITICAL_STRIKE,
    directHit: DEFAULT_DIRECT_HIT,
    determination: DEFAULT_DETERMINATION,
    skillSpeed: DEFAULT_SPEED,
    spellSpeed: DEFAULT_SPEED,
    tenacity: DEFAULT_TENACITY,
    piety: 0,
    mainStatIncreasePercent: -1,
    criticalStrikeIncreasePercent: -1,
    directHitIncreasePercent: -1,
    determinationIncreasePercent: -1,
    speedIncreasePercent: -1,
    tenacityIncreasePercent: -1,
    gcd: 250,
  };

  totalStats.mainStat += getBaseMainStat(jobAbbrev, race);

  itemSet.forEach((equipmentId) => {
    let equipment = EQUIPMENT_DATABASE_BY_ID.get(equipmentId);
    if (equipment === undefined) {
      return;
    }

    let itemStat = convertEquipmentToItemStat(equipment);

    totalStats.weaponDamage += itemStat.weaponDamage;
    totalStats.mainStat += itemStat.mainStat;
    totalStats.criticalStrike += itemStat.criticalStrike;
    totalStats.directHit += itemStat.directHit;
    totalStats.determination += itemStat.determination;
    totalStats.skillSpeed += itemStat.skillSpeed;
    totalStats.spellSpeed += itemStat.spellSpeed;
    totalStats.tenacity += itemStat.tenacity;
    totalStats.piety += itemStat.piety;
  });

  let food = FOOD_DATABASE.get(foodId);
  if (food !== undefined) {
    totalStats.criticalStrike += Math.min(
      food.criticalStrike,
      Math.floor(totalStats.criticalStrike / 10)
    );
    totalStats.directHit += Math.min(
      food.directHit,
      Math.floor(totalStats.directHit / 10)
    );
    totalStats.determination += Math.min(
      food.determination,
      Math.floor(totalStats.determination / 10)
    );
    totalStats.skillSpeed += Math.min(
      food.skillSpeed,
      Math.floor(totalStats.skillSpeed / 10)
    );
    totalStats.spellSpeed += Math.min(
      food.spellSpeed,
      Math.floor(totalStats.spellSpeed / 10)
    );
    totalStats.tenacity += Math.min(
      food.tenacity,
      Math.floor(totalStats.tenacity / 10)
    );
  }

  materiasInEachSlot.forEach((materiasInSlot) => {
    materiasInSlot.forEach((materia) => {
      addMateriaStatToTotalStat(totalStats, materia);
    });
  });

  return totalStats;
}

export function getStatByStatName(
  totalStats: TotalStats,
  statName: string,
  jobAbbrev: string
) {
  switch (statName) {
    case "WD":
      return `${totalStats.weaponDamage}`;
    case "STR":
    case "DEX":
    case "INT":
    case "MND":
      return `${totalStats.mainStat}`;
    case "CRT":
      return `${totalStats.criticalStrike}`;
    case "DH":
      return `${totalStats.directHit}`;
    case "DET":
      return `${totalStats.determination}`;
    case "SKS":
      return `${totalStats.skillSpeed}`;
    case "SPS":
      return `${totalStats.spellSpeed}`;
    case "TEN":
      return `${totalStats.tenacity}`;
    case "GCD": {
      totalStats.gcd = calculateGCD(
        getSpeedStatByJobAbbrev(totalStats, jobAbbrev)
      );
      return `${totalStats.gcd.toFixed(2)}`;
    }
  }
}

export function getStatPowerByStatName(
  totalStats: TotalStats,
  statName: string,
  jobAbbrev: string
) {
  switch (statName) {
    case "WD":
      return `${calculateWeaponMultiplierPercent(
        totalStats.weaponDamage,
        jobAbbrev
      )}%`;
    case "STR":
    case "DEX":
    case "INT":
    case "MND": {
      totalStats.mainStatIncreasePercent = calculateMainStatPercentMultiplier(
        totalStats.mainStat,
        isTank(jobAbbrev)
      );
      return `${totalStats.mainStatIncreasePercent}%`;
    }
    case "CRT": {
      totalStats.criticalStrikeIncreasePercent =
        calculateCriticalStrikePercentIncrease(totalStats.criticalStrike);
      return `+${totalStats.criticalStrikeIncreasePercent}%`;
    }
    case "DH": {
      totalStats.directHitIncreasePercent = calculateDirectHitPercentIncrease(
        totalStats.directHit
      );
      return `${totalStats.directHitIncreasePercent}%`;
    }
    case "DET": {
      totalStats.determinationIncreasePercent =
        calculateDeterminationPercentIncrease(totalStats.determination);
      return `${100 + totalStats.determinationIncreasePercent}%`;
    }
    case "SKS": {
      totalStats.speedIncreasePercent = calculateSpeedPercentIncrease(
        totalStats.skillSpeed
      );
      return `${100 + totalStats.speedIncreasePercent}%`;
    }
    case "SPS": {
      totalStats.speedIncreasePercent = calculateSpeedPercentIncrease(
        totalStats.spellSpeed
      );
      return `${100 + totalStats.speedIncreasePercent}%`;
    }
    case "TEN": {
      totalStats.tenacityIncreasePercent = calculateTenacityPercentIncrease(
        totalStats.tenacity
      );
      return `${100 + totalStats.tenacityIncreasePercent}%`;
    }
    case "GCD": {
      return `${totalStats.gcd.toFixed(2)}`;
    }
    default:
      return statName;
  }
}

function isTank(jobAbbrev: string) {
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

function getSpeedStatByJobAbbrev(totalStats: TotalStats, jobAbbrev: string) {
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
  totalStats: TotalStats,
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
          totalStats.mainStatIncreasePercent,
          isTank(jobAbbrev)
        ) - totalStats.mainStat
      );
    case "CRT":
      return (
        getMinNeededStatForCurrentCriticalStrike(
          totalStats.criticalStrikeIncreasePercent
        ) - totalStats.criticalStrike
      );
    case "DH":
      return (
        getMinNeededStatForCurrentDirectHit(
          totalStats.directHitIncreasePercent
        ) - totalStats.directHit
      );
    case "DET":
      return (
        getMinNeededStatForCurrentDetermination(
          totalStats.determinationIncreasePercent
        ) - totalStats.determination
      );
    case "SKS":
      return (
        getMinNeededStatForCurrentSpeed(totalStats.speedIncreasePercent) -
        totalStats.skillSpeed
      );
    case "SPS":
      return (
        getMinNeededStatForCurrentSpeed(totalStats.speedIncreasePercent) -
        totalStats.spellSpeed
      );
    case "TEN":
      return (
        getMinNeededStatForCurrentTenacity(totalStats.tenacityIncreasePercent) -
        totalStats.tenacity
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
  totalStats: TotalStats,
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
          totalStats.mainStatIncreasePercent + 1,
          isTank(jobAbbrev)
        ) - totalStats.mainStat
      );
    case "CRT":
      return (
        getMinNeededStatForCurrentCriticalStrike(
          totalStats.criticalStrikeIncreasePercent + 0.1
        ) - totalStats.criticalStrike
      );
    case "DH":
      return (
        getMinNeededStatForCurrentDirectHit(
          totalStats.directHitIncreasePercent + 0.1
        ) - totalStats.directHit
      );
    case "DET":
      return (
        getMinNeededStatForCurrentDetermination(
          totalStats.determinationIncreasePercent + 0.1
        ) - totalStats.determination
      );
    case "SKS":
      return (
        getMinNeededStatForCurrentSpeed(totalStats.speedIncreasePercent + 0.1) -
        totalStats.skillSpeed
      );
    case "SPS":
      return (
        getMinNeededStatForCurrentSpeed(totalStats.speedIncreasePercent + 0.1) -
        totalStats.spellSpeed
      );
    case "TEN":
      return (
        getMinNeededStatForCurrentTenacity(
          totalStats.tenacityIncreasePercent + 0.1
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
