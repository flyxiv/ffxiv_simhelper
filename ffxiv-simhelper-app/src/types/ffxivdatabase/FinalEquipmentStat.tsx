import { AppLanguageTexts } from "../../const/languageTexts";
import { Equipment } from "./Equipment";
import { Materia, updateMateriaValueStatToFinalStat } from "./Materia";

// final stat of equipment after all materia has been added
export interface FinalEquipmentStat {
  weaponDamage: number;
  mainStat: number;
  criticalStrike: number;
  directHit: number;
  determination: number;
  skillSpeed: number;
  spellSpeed: number;
  tenacity: number;
  piety: number;
  maxSubstat: number;
}

export function convertEquipmentToFinalStat(equipment: Equipment) {
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
    maxSubstat: equipment.maxSubstat,
  };
}

export function addMateriaMaxValueToEquipment(
  equipment: Equipment,
  materias: Materia[] | undefined
) {
  let finalEquipmentStat = convertEquipmentToFinalStat(equipment);

  if (materias === undefined) {
    return finalEquipmentStat;
  }

  let LANGUAGE_TEXTS = AppLanguageTexts();

  materias.forEach((materia) => {
    switch (materia.statName) {
      case LANGUAGE_TEXTS.CRIT_STAT_EN_NAME:
        finalEquipmentStat.criticalStrike += materia.maxValue;
        break;
      case LANGUAGE_TEXTS.DH_STAT_EN_NAME:
        finalEquipmentStat.directHit += materia.maxValue;
        break;
      case LANGUAGE_TEXTS.DET_STAT_EN_NAME:
        finalEquipmentStat.determination += materia.maxValue;
        break;
      case LANGUAGE_TEXTS.SKS_STAT_EN_NAME:
        finalEquipmentStat.skillSpeed += materia.maxValue;
        break;
      case LANGUAGE_TEXTS.SPS_STAT_EN_NAME:
        finalEquipmentStat.spellSpeed += materia.maxValue;
        break;
      default:
        finalEquipmentStat.tenacity += materia.maxValue;
        break;
    }
  });
  return finalEquipmentStat;
}

function getEquipmentFinalStatsNoClip(
  equipment: Equipment,
  materias: Materia[]
) {
  let finalEquipmentStat = convertEquipmentToFinalStat(equipment);

  materias.forEach((materia) => {
    updateMateriaValueStatToFinalStat(finalEquipmentStat, materia);
  });
  return finalEquipmentStat;
}

export function getEquipmentFinalStats(
  equipment: Equipment,
  materias: Materia[]
) {
  return trimExcessStats(getEquipmentFinalStatsNoClip(equipment, materias));
}

function trimExcessStats(finalStats: FinalEquipmentStat) {
  if (finalStats.criticalStrike > finalStats.maxSubstat) {
    finalStats.criticalStrike = finalStats.maxSubstat;
  }
  if (finalStats.directHit > finalStats.maxSubstat) {
    finalStats.directHit = finalStats.maxSubstat;
  }
  if (finalStats.determination > finalStats.maxSubstat) {
    finalStats.determination = finalStats.maxSubstat;
  }
  if (finalStats.skillSpeed > finalStats.maxSubstat) {
    finalStats.skillSpeed = finalStats.maxSubstat;
  }
  if (finalStats.spellSpeed > finalStats.maxSubstat) {
    finalStats.spellSpeed = finalStats.maxSubstat;
  }
  if (finalStats.tenacity > finalStats.maxSubstat) {
    finalStats.tenacity = finalStats.maxSubstat;
  }
  return finalStats;
}

export function accessSubStatByKey(
  finalStats: FinalEquipmentStat,
  key: string
) {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  switch (key) {
    case LANGUAGE_TEXTS.CRIT_STAT_EN_NAME:
      return finalStats.criticalStrike;
    case LANGUAGE_TEXTS.DH_STAT_EN_NAME:
      return finalStats.directHit;
    case LANGUAGE_TEXTS.DET_STAT_EN_NAME:
      return finalStats.determination;
    case LANGUAGE_TEXTS.SKS_STAT_EN_NAME:
      return finalStats.skillSpeed;
    case LANGUAGE_TEXTS.SPS_STAT_EN_NAME:
      return finalStats.spellSpeed;
    default:
      return finalStats.tenacity;
  }
}
