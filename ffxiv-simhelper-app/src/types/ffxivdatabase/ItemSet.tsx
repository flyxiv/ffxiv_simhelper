import {
  EMPTY_EQUIPMENT_ID,
  EQUIPMENT_DATABASE_BY_ID,
  EQUIPMENT_DATABASE_BY_KEYS,
  toEquipmentKeyString,
  TOTAL_SLOTS,
} from "./Equipment";
import { convertEquipmentToItemStat } from "./ItemStats";
import { getBaseMainStat } from "../../const/StartStats";

import { addMateriaStatToTotalStat } from "./Materia";
import {
  defaultPlayerPower,
  isTank,
  PlayerPower,
  setPartyCompositionBuffPercent,
} from "./PlayerPower";
import {
  calculateAutoDirectHitIncrease,
  calculateCriticalStrikePercentIncrease,
  calculateDeterminationPercentIncrease,
  calculateDirectHitPercentIncrease,
  calculateGCD,
  calculateMainStatPercentIncrease,
  calculateSpeedPercentIncrease,
  calculateTenacityPercentIncrease,
  calculateWeaponMultiplierPercent,
} from "./StatCalculator";
import { CRIT_BASE_DAMAGE, CRIT_BASE_PERCENT } from "./Stats";
import {
  EquipmentInput,
  SingleEquipmentInputSaveState,
} from "../EquipmentInput";
import {
  AST_EN_NAME,
  BLM_EN_NAME,
  BODY_SLOT_EN_TEXT,
  EARS_SLOT_EN_TEXT,
  FEET_SLOT_EN_TEXT,
  FINGER1_SLOT_EN_TEXT,
  FINGER2_SLOT_EN_TEXT,
  HANDS_SLOT_EN_TEXT,
  HEAD_SLOT_EN_TEXT,
  LEGS_SLOT_EN_TEXT,
  NECK_SLOT_EN_TEXT,
  OFFHAND_SLOT_EN_TEXT,
  PCT_EN_NAME,
  RDM_EN_NAME,
  SCH_EN_NAME,
  SGE_EN_NAME,
  SMN_EN_NAME,
  WEAPON_SLOT_EN_TEXT,
  WHM_EN_NAME,
  WRIST_SLOT_EN_TEXT,
} from "../../const/languageTexts";
import { FOOD_DATABASE } from "./Food";

export const WEAPON_SLOT_ID = 0;
export const HEAD_SLOT_ID = 1;
export const BODY_SLOT_ID = 2;
export const HANDS_SLOT_ID = 3;
export const LEGS_SLOT_ID = 4;
export const FEET_SLOT_ID = 5;
export const EAR_SLOT_ID = 6;
export const NECK_SLOT_ID = 7;
export const WRIST_SLOT_ID = 8;
export const FINGER1_SLOT_ID = 9;
export const FINGER2_SLOT_ID = 10;
export const OFFHAND_SLOT_ID = 11;

export type ItemSet = number[];

export function slotNameToSlotIndex(slotName: string): number {
  switch (slotName) {
    case WEAPON_SLOT_EN_TEXT:
      return WEAPON_SLOT_ID;
    case HEAD_SLOT_EN_TEXT:
      return HEAD_SLOT_ID;
    case BODY_SLOT_EN_TEXT:
      return BODY_SLOT_ID;
    case HANDS_SLOT_EN_TEXT:
      return HANDS_SLOT_ID;
    case LEGS_SLOT_EN_TEXT:
      return LEGS_SLOT_ID;
    case FEET_SLOT_EN_TEXT:
      return FEET_SLOT_ID;
    case EARS_SLOT_EN_TEXT:
      return EAR_SLOT_ID;
    case NECK_SLOT_EN_TEXT:
      return NECK_SLOT_ID;
    case WRIST_SLOT_EN_TEXT:
      return WRIST_SLOT_ID;
    case FINGER1_SLOT_EN_TEXT:
      return FINGER1_SLOT_ID;
    case FINGER2_SLOT_EN_TEXT:
      return FINGER2_SLOT_ID;
    case OFFHAND_SLOT_EN_TEXT:
      return OFFHAND_SLOT_ID;
    default:
      return EMPTY_EQUIPMENT_ID;
  }
}

export function slotIndexToSlotName(slotIndex: number): string {
  switch (slotIndex) {
    case WEAPON_SLOT_ID:
      return WEAPON_SLOT_EN_TEXT;
    case HEAD_SLOT_ID:
      return HEAD_SLOT_EN_TEXT;
    case BODY_SLOT_ID:
      return BODY_SLOT_EN_TEXT;
    case HANDS_SLOT_ID:
      return HANDS_SLOT_EN_TEXT;
    case LEGS_SLOT_ID:
      return LEGS_SLOT_EN_TEXT;
    case FEET_SLOT_ID:
      return FEET_SLOT_EN_TEXT;
    case EAR_SLOT_ID:
      return EARS_SLOT_EN_TEXT;
    case NECK_SLOT_ID:
      return NECK_SLOT_EN_TEXT;
    case WRIST_SLOT_ID:
      return WRIST_SLOT_EN_TEXT;
    case FINGER1_SLOT_ID:
      return FINGER1_SLOT_EN_TEXT;
    case FINGER2_SLOT_ID:
      return FINGER2_SLOT_EN_TEXT;
    case OFFHAND_SLOT_ID:
      return OFFHAND_SLOT_EN_TEXT;
    default:
      return "";
  }
}

export function defaultItemSet(jobAbbrev: string): [ItemSet, number] {
  let itemSet: number[] = [];
  let materiaCount = 0;

  TOTAL_SLOTS.forEach((_) => {
    itemSet.push(EMPTY_EQUIPMENT_ID);
  });

  let weaponsOfJob = EQUIPMENT_DATABASE_BY_KEYS.get(
    toEquipmentKeyString(jobAbbrev, WEAPON_SLOT_EN_TEXT)
  );
  if (weaponsOfJob !== undefined) {
    itemSet[WEAPON_SLOT_ID] = weaponsOfJob[0].id;
    materiaCount = weaponsOfJob[0].pentameldable
      ? 5
      : weaponsOfJob[0].materiaSlotCount;
  }

  return [itemSet, materiaCount];
}

export function calculatePlayerPowerFromInputs(
  totalState: SingleEquipmentInputSaveState
): PlayerPower {
  let power: PlayerPower = defaultPlayerPower();

  power.mainStat += getBaseMainStat(
    totalState.mainPlayerJobAbbrev,
    totalState.race
  );

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

  totalState.gearSetMaterias.forEach((materiasInSlot) => {
    materiasInSlot.forEach((materia) => {
      addMateriaStatToTotalStat(power, materia);
    });
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

  setPartyCompositionBuffPercent(totalState);
  power.mainStat = Math.floor(
    power.mainStat * (1 + totalState.compositionBuffPercent / 100)
  );

  calculatePowerByStat(power, totalState.mainPlayerJobAbbrev);
  power.gcd = calculateGCD(power, totalState.mainPlayerJobAbbrev) / 100;

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
    case WHM_EN_NAME:
    case SCH_EN_NAME:
    case AST_EN_NAME:
    case SGE_EN_NAME:
    case BLM_EN_NAME:
    case SMN_EN_NAME:
    case RDM_EN_NAME:
    case PCT_EN_NAME:
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
  console.log("newTotalState: ", newTotalState);

  setTotalState({
    ...newTotalState,
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
  newTotalState.equipmentDatas.forEach(
    (data: SingleEquipmentInputSaveState) => {
      data.power = updatedPower;
    }
  );

  setTotalState({
    ...newTotalState,
  });
}
