import {
  EMPTY_EQUIPMENT_ID,
  EQUIPMENT_DATABASE_BY_ID,
  TOTAL_SLOTS,
} from "./Equipment";
import { convertEquipmentToItemStat } from "./ItemStats";

export type ItemSet = Map<string, number>;

export function defaultItemSet(): ItemSet {
  let itemSet = new Map();

  TOTAL_SLOTS.forEach((slot) => {
    itemSet.set(slot, EMPTY_EQUIPMENT_ID);
  });

  return itemSet;
}

export function calculateTotalStatsOfItemSet(itemSet: ItemSet) {
  let totalStats = {
    weaponDamage: 0,
    mainStat: 0,
    criticalStrike: 0,
    directHit: 0,
    determination: 0,
    skillSpeed: 0,
    spellSpeed: 0,
    tenacity: 0,
    piety: 0,
  };

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

  return totalStats;
}
