import { Equipment } from "./Equipment";
import {
  convertEquipmentToFinalStat,
  FinalEquipmentStat,
} from "./FinalEquipmentStat";
import { TotalStats } from "./ItemSet";

export const EMPTY_MATERIA = {
  statName: "",
  maxValue: 0,
  effectiveValue: 0,
};

export const NON_PENTAMELDABLE_MATERIAS = [
  "CRT+54",
  "DH+54",
  "DET+54",
  "SKS+54",
  "SPS+54",
  "TEN+54",
];

export const PENTAMELDABLE_MATERIAS = [
  "CRT+18",
  "DH+18",
  "DET+18",
  "SKS+18",
  "SPS+18",
  "TEN+18",
];

export interface Materia {
  statName: string;
  maxValue: number;
  effectiveValue: number;
}

function getStatByStatName(equipment: Equipment, statName: string) {
  switch (statName) {
    case "CRT":
      return equipment.criticalStrike;
    case "DH":
      return equipment.directHit;
    case "DET":
      return equipment.determination;
    case "SKS":
      return equipment.skillSpeed;
    case "SPS":
      return equipment.spellSpeed;
    default:
      return equipment.tenacity;
  }
}

export function updateMateriaValueStatToFinalStat(
  finalStats: FinalEquipmentStat,
  materia: Materia
) {
  switch (materia.statName) {
    case "CRT":
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
    case "DH":
      if (finalStats.directHit + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue = finalStats.maxSubstat - finalStats.directHit;
        finalStats.directHit = finalStats.maxSubstat;
      } else {
        finalStats.directHit += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    case "DET":
      if (finalStats.determination + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue =
          finalStats.maxSubstat - finalStats.determination;
        finalStats.determination = finalStats.maxSubstat;
      } else {
        finalStats.determination += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    case "SKS":
      if (finalStats.skillSpeed + materia.maxValue > finalStats.maxSubstat) {
        materia.effectiveValue = finalStats.maxSubstat - finalStats.skillSpeed;
        finalStats.skillSpeed = finalStats.maxSubstat;
      } else {
        finalStats.skillSpeed += materia.maxValue;
        materia.effectiveValue = materia.maxValue;
      }
      break;
    case "SPS":
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
    return null;
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
  materiaSlot: number
) {
  if (materiaSlot < equipment.materiaSlotCount) {
    return NON_PENTAMELDABLE_MATERIAS;
  } else if (equipment.pentameldable && materiaSlot <= 5) {
    if (materiaSlot === equipment.materiaSlotCount) {
      return NON_PENTAMELDABLE_MATERIAS;
    } else {
      return PENTAMELDABLE_MATERIAS;
    }
  }

  return [];
}

export function toMateriaKey(materia: Materia | null) {
  if (materia === null) {
    return "empty";
  }
  return `${materia.statName}+${materia.maxValue}`;
}

export function toMateriaDescription(materia: Materia) {
  return `${materia.statName}+${materia.effectiveValue}`;
}

export function addMateriaStatToTotalStat(
  totalStats: TotalStats,
  materia: Materia
) {
  switch (materia.statName) {
    case "CRT":
      totalStats.criticalStrike += materia.effectiveValue;
      break;
    case "DH":
      totalStats.directHit += materia.effectiveValue;
      break;
    case "DET":
      totalStats.determination += materia.effectiveValue;
      break;
    case "SKS":
      totalStats.skillSpeed += materia.effectiveValue;
      break;
    case "SPS":
      totalStats.spellSpeed += materia.effectiveValue;
      break;
    default:
      totalStats.tenacity += materia.effectiveValue;
      break;
  }
}
