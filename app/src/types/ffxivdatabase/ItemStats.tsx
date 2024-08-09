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
  return {
    weaponDamage: equipment.weaponDamage,
    mainStat: equipment.mainStat,
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
