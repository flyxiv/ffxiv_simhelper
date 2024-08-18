import { Equipment } from "./Equipment";
import { Food } from "./Food";

export interface ItemStats {
  weaponDamage: number;
  mainStat: number;
  criticalStrike: number;
  directHit: number;
  determination: number;
  skillSpeed: number;
  spellSpeed: number;
  tenacity: number;
  piety: number;
}

export function convertEquipmentToItemStat(equipment: Equipment) {
  let mainStat = 0;
  if (equipment.STR > 0) {
    mainStat = equipment.STR;
  } else if (equipment.DEX > 0) {
    mainStat = equipment.DEX;
  } else if (equipment.INT > 0) {
    mainStat = equipment.INT;
  } else if (equipment.MND > 0) {
    mainStat = equipment.MND;
  }

  return {
    weaponDamage: equipment.weaponDamage,
    mainStat: mainStat,
    criticalStrike: equipment.criticalStrike,
    directHit: equipment.directHit,
    determination: equipment.determination,
    skillSpeed: equipment.skillSpeed,
    spellSpeed: equipment.spellSpeed,
    tenacity: equipment.tenacity,
    piety: equipment.piety,
  };
}

export function convertFoodToItemStat(food: Food) {
  return {
    weaponDamage: 0,
    mainStat: 0,
    criticalStrike: food.criticalStrike,
    directHit: food.directHit,
    determination: food.determination,
    skillSpeed: food.skillSpeed,
    spellSpeed: food.spellSpeed,
    tenacity: food.tenacity,
    piety: food.piety,
  };
}

export function addItemStats(stats1: ItemStats, stats2: ItemStats) {
  return {
    weaponDamage: stats1.weaponDamage + stats2.weaponDamage,
    mainStat: stats1.mainStat + stats2.mainStat,
    criticalStrike: stats1.criticalStrike + stats2.criticalStrike,
    directHit: stats1.directHit + stats2.directHit,
    determination: stats1.determination + stats2.determination,
    skillSpeed: stats1.skillSpeed + stats2.skillSpeed,
    spellSpeed: stats1.spellSpeed + stats2.spellSpeed,
    tenacity: stats1.tenacity + stats2.tenacity,
    piety: stats1.piety + stats2.piety,
  };
}
