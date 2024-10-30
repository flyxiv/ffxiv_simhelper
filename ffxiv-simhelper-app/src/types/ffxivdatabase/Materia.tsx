import { convertToStatName, CRIT_STAT_EN_NAME, DET_STAT_EN_NAME, DH_STAT_EN_NAME, PIE_STAT_EN_NAME, SKS_STAT_EN_NAME, SPS_STAT_EN_NAME, TEN_STAT_EN_NAME, TextDictionary } from "../../const/languageTexts";
import { Equipment, getFirstSubStat } from "./Equipment";
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

const BIG_MATERIA_STAT = 54;
const SMALL_MATERIA_STAT = 18;

export const NON_PENTAMELDABLE_MATERIAS = [
  `${CRIT_STAT_EN_NAME}+${BIG_MATERIA_STAT}`,
  `${DH_STAT_EN_NAME}+${BIG_MATERIA_STAT}`,
  `${DET_STAT_EN_NAME}+${BIG_MATERIA_STAT}`,
  `${SKS_STAT_EN_NAME}+${BIG_MATERIA_STAT}`,
  `${SPS_STAT_EN_NAME}+${BIG_MATERIA_STAT}`,
  `${TEN_STAT_EN_NAME}+${BIG_MATERIA_STAT}`,
  `${PIE_STAT_EN_NAME}+${BIG_MATERIA_STAT}`,
];

export const PENTAMELDABLE_MATERIAS = [
  `${CRIT_STAT_EN_NAME}+${SMALL_MATERIA_STAT}`,
  `${DH_STAT_EN_NAME}+${SMALL_MATERIA_STAT}`,
  `${DET_STAT_EN_NAME}+${SMALL_MATERIA_STAT}`,
  `${SKS_STAT_EN_NAME}+${SMALL_MATERIA_STAT}`,
  `${SPS_STAT_EN_NAME}+${SMALL_MATERIA_STAT}`,
  `${TEN_STAT_EN_NAME}+${SMALL_MATERIA_STAT}`,
  `${PIE_STAT_EN_NAME}+${SMALL_MATERIA_STAT}`,
];



export interface Materia {
  statName: string;
  maxValue: number;
  effectiveValue: number;
}

export function updateMateriaValueStatToFinalStat(
  finalStats: FinalEquipmentStat,
  materia: Materia
) {
  switch (materia.statName) {
    case CRIT_STAT_EN_NAME:
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
    case DH_STAT_EN_NAME:
      if (finalStats.directHit + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue = finalStats.maxSubstat - finalStats.directHit;
        finalStats.directHit = finalStats.maxSubstat;
      } else {
        finalStats.directHit += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    case DET_STAT_EN_NAME:
      if (finalStats.determination + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue =
          finalStats.maxSubstat - finalStats.determination;
        finalStats.determination = finalStats.maxSubstat;
      } else {
        finalStats.determination += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    case SKS_STAT_EN_NAME:
      if (finalStats.skillSpeed + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue = finalStats.maxSubstat - finalStats.skillSpeed;
        finalStats.skillSpeed = finalStats.maxSubstat;
      } else {
        finalStats.skillSpeed += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    case SPS_STAT_EN_NAME:
      if (finalStats.spellSpeed + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue = finalStats.maxSubstat - finalStats.spellSpeed;
        finalStats.spellSpeed = finalStats.maxSubstat;
      } else {
        finalStats.spellSpeed += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    case TEN_STAT_EN_NAME:
      if (finalStats.tenacity + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue = finalStats.maxSubstat - finalStats.tenacity;
        finalStats.tenacity = finalStats.maxSubstat;
      } else {
        finalStats.tenacity += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    default:
      if (finalStats.piety + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue = finalStats.maxSubstat - finalStats.piety;
        finalStats.piety = finalStats.maxSubstat;
      } else {
        finalStats.piety += materia.maxValue;
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

  let firstSubStat = getFirstSubStat(equipment);

  let casterJob = isCaster(jobAbbrev);

  for (let i = 0; i < possibleMaterias.length; i++) {
    let [statName, _] = possibleMaterias[i].split("+");

    if (statName === firstSubStat) {
      possibleMaterias.splice(i, 1);
      i = i - 1;
    }

    if (!casterJob && statName === SPS_STAT_EN_NAME) {
      possibleMaterias.splice(i, 1);
      i = i - 1;
    }
    if (casterJob && statName === SKS_STAT_EN_NAME) {
      possibleMaterias.splice(i, 1);
      i = i - 1;
    }
  }

  return possibleMaterias;
}

export function toMateriaKey(materia: Materia | null) {
  if (materia === null || materia.statName === "") {
    return "empty";
  }
  return `${materia.statName}+${materia.maxValue}`;
}

export function toMateriaDescription(materia: Materia, LANGUAGE_TEXTS: TextDictionary) {
  return `${convertToStatName(materia.statName, LANGUAGE_TEXTS)}+${materia.effectiveValue}`;
}

// {StatName}+{Value} -> {StatNameText(KR/EN)}+{Value}
export function materiaKeyToText(materiaKey: string, LANGUAGE_TEXTS: TextDictionary) {
  let [statName, value] = materiaKey.split("+");
  return `${convertToStatName(statName, LANGUAGE_TEXTS)}+${value}`;
}

export function addMateriaStatToTotalStat(
  totalStats: PlayerPower,
  materia: Materia
) {
  switch (materia.statName) {
    case CRIT_STAT_EN_NAME:
      totalStats.criticalStrike += materia.effectiveValue;
      break;
    case DH_STAT_EN_NAME:
      totalStats.directHit += materia.effectiveValue;
      break;
    case DET_STAT_EN_NAME:
      totalStats.determination += materia.effectiveValue;
      break;
    case SKS_STAT_EN_NAME:
      totalStats.skillSpeed += materia.effectiveValue;
      break;
    case SPS_STAT_EN_NAME:
      totalStats.spellSpeed += materia.effectiveValue;
      break;
    case PIE_STAT_EN_NAME:
      totalStats.piety += materia.effectiveValue;
      break;
    default:
      totalStats.tenacity += materia.effectiveValue;
      break;
  }
}
