import {
  EMPTY_EQUIPMENT_ID,
  EQUIPMENT_DATABASE_BY_ID,
  TOTAL_SLOTS,
} from "./Equipment";
import { convertEquipmentToItemStat } from "./ItemStats";
import { getBaseMainStat } from "../../const/StartStats";

import { FOOD_DATABASE } from "./Food";
import { addMateriaStatToTotalStat } from "./Materia";
import {
  defaultPlayerPower,
  isTank,
  PlayerPower,
} from "./PlayerPower";
import {
  calculateAutoDirectHitIncrease,
  calculateCriticalStrikePercentIncrease,
  calculateDeterminationPercentIncrease,
  calculateDirectHitPercentIncrease,
  calculateMainStatPercentIncrease,
  calculateSpeedPercentIncrease,
  calculateTenacityPercentIncrease,
  calculateWeaponMultiplierPercent,
} from "./StatCalculator";
import { CRIT_BASE_DAMAGE, CRIT_BASE_PERCENT } from "./Stats";
import { EquipmentInput, SingleEquipmentInputSaveState } from "../EquipmentInput";

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

export const WEAPON_SLOT_NAME = "weapon";
export const HEAD_SLOT_NAME = "head";
export const BODY_SLOT_NAME = "body";
export const HANDS_SLOT_NAME = "hands";
export const LEGS_SLOT_NAME = "legs";
export const FEET_SLOT_NAME = "feet";
export const NECK_SLOT_NAME = "neck";
export const EAR_SLOT_NAME = "ears";
export const WRIST_SLOT_NAME = "wrists";
export const FINGER1_SLOT_NAME = "finger1";
export const FINGER2_SLOT_NAME = "finger2";
export const OFFHAND_SLOT_NAME = "offHand";

export type ItemSet = number[];

export function slotNameToSlotIndex(slotName: string): number {
  switch (slotName) {
    case WEAPON_SLOT_NAME:
      return WEAPON_SLOT_ID;
    case HEAD_SLOT_NAME:
      return HEAD_SLOT_ID;
    case BODY_SLOT_NAME:
      return BODY_SLOT_ID;
    case HANDS_SLOT_NAME:
      return HANDS_SLOT_ID;
    case LEGS_SLOT_NAME:
      return LEGS_SLOT_ID;
    case FEET_SLOT_NAME:
      return FEET_SLOT_ID;
    case NECK_SLOT_NAME:
      return NECK_SLOT_ID;
    case EAR_SLOT_NAME:
      return EAR_SLOT_ID;
    case WRIST_SLOT_NAME:
      return WRIST_SLOT_ID;
    case FINGER1_SLOT_NAME:
      return FINGER1_SLOT_ID;
    case FINGER2_SLOT_NAME:
      return FINGER2_SLOT_ID;
    case OFFHAND_SLOT_NAME:
      return OFFHAND_SLOT_ID;
    default:
      return -1;
  }
}

export function slotIndexToSlotName(slotIndex: number): string {
  switch (slotIndex) {
    case WEAPON_SLOT_ID:
      return WEAPON_SLOT_NAME;
    case HEAD_SLOT_ID:
      return HEAD_SLOT_NAME;
    case BODY_SLOT_ID:
      return BODY_SLOT_NAME;
    case HANDS_SLOT_ID:
      return HANDS_SLOT_NAME;
    case LEGS_SLOT_ID:
      return LEGS_SLOT_NAME;
    case FEET_SLOT_ID:
      return FEET_SLOT_NAME;
    case EAR_SLOT_ID:
      return EAR_SLOT_NAME;
    case NECK_SLOT_ID:
      return NECK_SLOT_NAME;
    case WRIST_SLOT_ID:
      return WRIST_SLOT_NAME;
    case FINGER1_SLOT_ID:
      return FINGER1_SLOT_NAME;
    case FINGER2_SLOT_ID:
      return FINGER2_SLOT_NAME;
    case OFFHAND_SLOT_ID:
      return OFFHAND_SLOT_NAME;
    default:
      return "";
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
  totalState: SingleEquipmentInputSaveState
): PlayerPower {
  let power: PlayerPower = defaultPlayerPower();

  power.mainStat += getBaseMainStat(totalState.mainPlayerJobAbbrev, totalState.race);

  totalState.itemSet.forEach((equipmentId) => {
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

  let food = FOOD_DATABASE.get(totalState.foodId);
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

  totalState.gearSetMaterias.forEach((materiasInSlot) => {
    materiasInSlot.forEach((materia) => {
      addMateriaStatToTotalStat(power, materia);
    });
  });

  calculatePowerByStat(power, totalState.mainPlayerJobAbbrev);

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

  power.autoDirectHitIncrease = calculateAutoDirectHitIncrease(power.directHit);
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

export function updateOnePlayerPower(
  id: number,
  totalState: EquipmentInput,
  setTotalState: Function
) {
  let updatedPower = calculatePlayerPowerFromInputs(
    totalState.equipmentDatas[id]
  );

  let newTotalState = { ...totalState };
  newTotalState.equipmentDatas[id].power = updatedPower;

  setTotalState({
    ...newTotalState
  });
}

export function updateAllPlayerPower(
  totalState: EquipmentInput,
  setTotalState: Function
) {
  let updatedPower = calculatePlayerPowerFromInputs(
    totalState.equipmentDatas[0]
  );

  let newTotalState = { ...totalState };
  newTotalState.equipmentDatas.forEach((data: SingleEquipmentInputSaveState) => {
    data.power = updatedPower;
  })

  setTotalState({
    ...newTotalState
  });
}
