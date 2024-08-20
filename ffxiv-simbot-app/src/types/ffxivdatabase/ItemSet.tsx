import {
  EMPTY_EQUIPMENT_ID,
  EQUIPMENT_DATABASE_BY_ID,
  TOTAL_SLOTS,
} from "./Equipment";
import { convertEquipmentToItemStat } from "./ItemStats";
import { getBaseMainStat } from "../../const/StartStats";

import { FOOD_DATABASE } from "./Food";
import { addMateriaStatToTotalStat, GearSetMaterias } from "./Materia";
import {
  CharacterEquipmentsData,
  defaultPlayerPower,
  isTank,
  PlayerPower,
} from "./PlayerPower";
import {
  calculateCriticalStrikePercentIncrease,
  calculateDeterminationPercentIncrease,
  calculateDirectHitPercentIncrease,
  calculateMainStatPercentIncrease,
  calculateSpeedPercentIncrease,
  calculateTenacityPercentIncrease,
  calculateWeaponMultiplierPercent,
} from "./StatCalculator";
import { CRIT_BASE_DAMAGE, CRIT_BASE_PERCENT } from "./Stats";

export const WEAPON_SLOT_ID = 0;
export const HEAD_SLOT_ID = 1;
export const BODY_SLOT_ID = 2;
export const HANDS_SLOT_ID = 3;
export const LEGS_SLOT_ID = 4;
export const FEET_SLOT_ID = 5;
export const NECK_SLOT_ID = 6;
export const EAR_SLOT_ID = 7;
export const WRIST_SLOT_ID = 8;
export const FINGER1_SLOT_ID = 9;
export const FINGER2_SLOT_ID = 10;
export const OFFHAND_SLOT_ID = 11;

export type ItemSet = number[];

export function slotNameToSlotIndex(slotName: string): number {
  switch (slotName) {
    case "weapon":
      return WEAPON_SLOT_ID;
    case "head":
      return HEAD_SLOT_ID;
    case "body":
      return BODY_SLOT_ID;
    case "hands":
      return HANDS_SLOT_ID;
    case "legs":
      return LEGS_SLOT_ID;
    case "feet":
      return FEET_SLOT_ID;
    case "neck":
      return NECK_SLOT_ID;
    case "ears":
      return EAR_SLOT_ID;
    case "wrists":
      return WRIST_SLOT_ID;
    case "finger1":
      return FINGER1_SLOT_ID;
    case "finger2":
      return FINGER2_SLOT_ID;
    case "offHand":
      return OFFHAND_SLOT_ID;
    default:
      return -1;
  }
}

export function defaultItemSet(): ItemSet {
  let itemSet: number[] = [];

  TOTAL_SLOTS.forEach((_) => {
    itemSet.push(EMPTY_EQUIPMENT_ID);
  });

  return itemSet;
}

export function calculatePlayerPowerFromInputs(
  itemSet: ItemSet,
  jobAbbrev: string,
  race: string,
  foodId: number,
  gearSetMaterias: GearSetMaterias
): PlayerPower {
  let power: PlayerPower = defaultPlayerPower();

  power.mainStat += getBaseMainStat(jobAbbrev, race);
  console.log(power.mainStat);

  itemSet.forEach((equipmentId) => {
    let equipment = EQUIPMENT_DATABASE_BY_ID.get(equipmentId);
    if (equipment === undefined) {
      return;
    }

    let itemStat = convertEquipmentToItemStat(equipment);

    power.weaponDamage += itemStat.weaponDamage;
    power.mainStat += itemStat.mainStat;
    power.criticalStrike += itemStat.criticalStrike;
    power.directHit += itemStat.directHit;
    power.determination += itemStat.determination;
    power.skillSpeed += itemStat.skillSpeed;
    power.spellSpeed += itemStat.spellSpeed;
    power.tenacity += itemStat.tenacity;
    power.piety += itemStat.piety;
  });

  let food = FOOD_DATABASE.get(foodId);
  if (food !== undefined) {
    power.criticalStrike += Math.min(
      food.criticalStrike,
      Math.floor(power.criticalStrike / 10)
    );
    power.directHit += Math.min(
      food.directHit,
      Math.floor(power.directHit / 10)
    );
    power.determination += Math.min(
      food.determination,
      Math.floor(power.determination / 10)
    );
    power.skillSpeed += Math.min(
      food.skillSpeed,
      Math.floor(power.skillSpeed / 10)
    );
    power.spellSpeed += Math.min(
      food.spellSpeed,
      Math.floor(power.spellSpeed / 10)
    );
    power.tenacity += Math.min(food.tenacity, Math.floor(power.tenacity / 10));
  }

  gearSetMaterias.forEach((materiasInSlot) => {
    materiasInSlot.forEach((materia) => {
      addMateriaStatToTotalStat(power, materia);
    });
  });

  calculatePowerByStat(power, jobAbbrev);

  return power;
}

export function calculatePowerByStat(power: PlayerPower, jobAbbrev: string) {
  power.weaponDamageMultiplier =
    calculateWeaponMultiplierPercent(power.weaponDamage, jobAbbrev) / 100;

  power.mainStatMultiplier =
    1 +
    calculateMainStatPercentIncrease(power.mainStat, isTank(jobAbbrev)) / 100;
  let criticalStrikeIncrease =
    calculateCriticalStrikePercentIncrease(power.criticalStrike) / 100;
  power.criticalStrikeRate = CRIT_BASE_PERCENT + criticalStrikeIncrease;
  power.criticalStrikeDamage = CRIT_BASE_DAMAGE + criticalStrikeIncrease;

  power.directHitRate =
    calculateDirectHitPercentIncrease(power.directHit) / 100;
  power.determinationMultiplier =
    calculateDeterminationPercentIncrease(power.determination) / 100 + 1;

  power.speedMultiplier = isCaster(jobAbbrev)
    ? 1 + calculateSpeedPercentIncrease(power.spellSpeed) / 100
    : 1 + calculateSpeedPercentIncrease(power.skillSpeed) / 100;

  power.tenacityMultiplier =
    1 + calculateTenacityPercentIncrease(power.tenacity) / 100;
}

export function isCaster(jobAbbrev: string) {
  switch (jobAbbrev) {
    case "WHM":
    case "SCH":
    case "AST":
    case "SGE":
    case "BLM":
    case "SMN":
    case "RDM":
    case "PCT":
      return true;
    default:
      return false;
  }
}

export function updatePlayerPower(
  data: CharacterEquipmentsData,
  setData: Function
) {
  let updatedPower = calculatePlayerPowerFromInputs(
    data.itemSet,
    data.jobAbbrev,
    data.race,
    data.foodId,
    data.gearSetMaterias
  );

  setData({
    ...data,
    power: updatedPower,
  });
}
