import { AppLanguageTexts } from "../../const/languageTexts";
import { Equipment, getFirstSecondSubStat } from "./Equipment";
import {
  convertEquipmentToFinalStat,
  FinalEquipmentStat,
} from "./FinalEquipmentStat";
import { isCaster } from "./ItemSet";
import { PlayerPower } from "./PlayerPower";

export type GearSetMaterias = Materia[][];

export const EMPTY_MATERIA = {
  statName: "",
  maxValue: 0,
  effectiveValue: 0,
};

export const loadMaterias = () => {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  const NON_PENTAMELDABLE_MATERIAS = [
    `${LANGUAGE_TEXTS.CRIT_STAT_EN_NAME}+54`,
    `${LANGUAGE_TEXTS.DH_STAT_EN_NAME}+54`,
    `${LANGUAGE_TEXTS.DET_STAT_EN_NAME}+54`,
    `${LANGUAGE_TEXTS.SKS_STAT_EN_NAME}+54`,
    `${LANGUAGE_TEXTS.SPS_STAT_EN_NAME}+54`,
    `${LANGUAGE_TEXTS.TEN_STAT_EN_NAME}+54`,
  ];

  const PENTAMELDABLE_MATERIAS = [
    `${LANGUAGE_TEXTS.CRIT_STAT_EN_NAME}+18`,
    `${LANGUAGE_TEXTS.DH_STAT_EN_NAME}+18`,
    `${LANGUAGE_TEXTS.DET_STAT_EN_NAME}+18`,
    `${LANGUAGE_TEXTS.SKS_STAT_EN_NAME}+18`,
    `${LANGUAGE_TEXTS.SPS_STAT_EN_NAME}+18`,
    `${LANGUAGE_TEXTS.TEN_STAT_EN_NAME}+18`,
  ];

  return { NON_PENTAMELDABLE_MATERIAS, PENTAMELDABLE_MATERIAS };
}


export interface Materia {
  statName: string;
  maxValue: number;
  effectiveValue: number;
}

export function updateMateriaValueStatToFinalStat(
  finalStats: FinalEquipmentStat,
  materia: Materia
) {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  switch (materia.statName) {
    case LANGUAGE_TEXTS.CRIT_STAT_EN_NAME:
      if (
        finalStats.criticalStrike + materia.maxValue >
        finalStats.maxSubstat
      ) {
        materia.effectiveValue =
          finalStats.maxSubstat - finalStats.criticalStrike;
        finalStats.criticalStrike = finalStats.maxSubstat;
      } else {
        finalStats.criticalStrike += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    case LANGUAGE_TEXTS.DH_STAT_EN_NAME:
      if (finalStats.directHit + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue = finalStats.maxSubstat - finalStats.directHit;
        finalStats.directHit = finalStats.maxSubstat;
      } else {
        finalStats.directHit += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    case LANGUAGE_TEXTS.DET_STAT_EN_NAME:
      if (finalStats.determination + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue =
          finalStats.maxSubstat - finalStats.determination;
        finalStats.determination = finalStats.maxSubstat;
      } else {
        finalStats.determination += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    case LANGUAGE_TEXTS.SKS_STAT_EN_NAME:
      if (finalStats.skillSpeed + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue = finalStats.maxSubstat - finalStats.skillSpeed;
        finalStats.skillSpeed = finalStats.maxSubstat;
      } else {
        finalStats.skillSpeed += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    case LANGUAGE_TEXTS.SPS_STAT_EN_NAME:
      if (finalStats.spellSpeed + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue = finalStats.maxSubstat - finalStats.spellSpeed;
        finalStats.spellSpeed = finalStats.maxSubstat;
      } else {
        finalStats.spellSpeed += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    default:
      if (finalStats.tenacity + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue = finalStats.maxSubstat - finalStats.tenacity;
        finalStats.tenacity = finalStats.maxSubstat;
      } else {
        finalStats.tenacity += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
  }
}

export function updateMateriaList(
  materiaKey: string, // {statName}+{maxValue} ex) CRT+54
  equipment: Equipment,
  materiasOfSlot: Materia[],
  materiaSlot: number
) {
  if (materiaKey === "empty") {
    materiasOfSlot[materiaSlot] = EMPTY_MATERIA;
    return;
  }

  let [statName, maxValue] = materiaKey.split("+");
  let maxValueInt = parseInt(maxValue);
  let materia = {
    statName: statName,
    maxValue: maxValueInt,
    effectiveValue: maxValueInt,
  };
  materiasOfSlot[materiaSlot] = materia;

  let finalStat = convertEquipmentToFinalStat(equipment);

  for (let i = 0; i < materiasOfSlot.length; i++) {
    let materia = materiasOfSlot[i];
    if (materia.statName === "") {
      continue;
    }

    updateMateriaValueStatToFinalStat(finalStat, materia);
  }
}

export function getPossibleMateriasForEquipmentSlot(
  equipment: Equipment,
  materiaSlot: number,
  jobAbbrev: string
) {
  let { NON_PENTAMELDABLE_MATERIAS, PENTAMELDABLE_MATERIAS } = loadMaterias();

  let possibleMaterias: Array<string> = [];
  if (materiaSlot < equipment.materiaSlotCount) {
    possibleMaterias = [...NON_PENTAMELDABLE_MATERIAS];
  } else if (equipment.pentameldable && materiaSlot <= 5) {
    if (materiaSlot === equipment.materiaSlotCount) {
      possibleMaterias = [...NON_PENTAMELDABLE_MATERIAS];
    } else {
      possibleMaterias = [...PENTAMELDABLE_MATERIAS];
    }
  }

  let [firstSubStat, secondSubStat] = getFirstSecondSubStat(equipment);

  let secondSubStatIdx = -1;
  let casterJob = isCaster(jobAbbrev);
  let LANGUAGE_TEXTS = AppLanguageTexts();

  for (let i = 0; i < possibleMaterias.length; i++) {
    let [statName, _] = possibleMaterias[i].split("+");

    if (statName === firstSubStat) {
      possibleMaterias.splice(i, 1);
      i = i - 1;
    }

    if (!casterJob && statName === LANGUAGE_TEXTS.SPS_STAT_EN_NAME) {
      possibleMaterias.splice(i, 1);
      i = i - 1;
    }
    if (casterJob && statName === LANGUAGE_TEXTS.SKS_STAT_EN_NAME) {
      possibleMaterias.splice(i, 1);
      i = i - 1;
    }
  }

  for (let i = 0; i < possibleMaterias.length; i++) {
    let [statName, _] = possibleMaterias[i].split("+");

    if (statName === secondSubStat) {
      secondSubStatIdx = i;
    }
  }

  if (secondSubStatIdx >= 0) {
    possibleMaterias.push(possibleMaterias[secondSubStatIdx]);
    possibleMaterias.splice(secondSubStatIdx, 1);
  }

  return possibleMaterias;
}

export function toMateriaKey(materia: Materia | null) {
  if (materia === null || materia.statName === "") {
    return "empty";
  }
  return `${materia.statName}+${materia.maxValue}`;
}

export function toMateriaDescription(materia: Materia) {
  return `${materia.statName} + ${materia.effectiveValue}`;
}

export function addMateriaStatToTotalStat(
  totalStats: PlayerPower,
  materia: Materia
) {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  switch (materia.statName) {
    case LANGUAGE_TEXTS.CRIT_STAT_EN_NAME:
      totalStats.criticalStrike += materia.effectiveValue;
      break;
    case LANGUAGE_TEXTS.DH_STAT_EN_NAME:
      totalStats.directHit += materia.effectiveValue;
      break;
    case LANGUAGE_TEXTS.DET_STAT_EN_NAME:
      totalStats.determination += materia.effectiveValue;
      break;
    case LANGUAGE_TEXTS.SKS_STAT_EN_NAME:
      totalStats.skillSpeed += materia.effectiveValue;
      break;
    case LANGUAGE_TEXTS.SPS_STAT_EN_NAME:
      totalStats.spellSpeed += materia.effectiveValue;
      break;
    default:
      totalStats.tenacity += materia.effectiveValue;
      break;
  }
}
