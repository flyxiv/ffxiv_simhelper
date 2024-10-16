import { AST_EN_NAME, BLM_EN_NAME, BRD_EN_NAME, CRIT_STAT_EN_NAME, DET_STAT_EN_NAME, DEX_STAT_EN_NAME, DH_STAT_EN_NAME, DNC_EN_NAME, DRG_EN_NAME, DRK_EN_NAME, GNB_EN_NAME, INT_STAT_EN_NAME, MCH_EN_NAME, MIND_STAT_EN_NAME, MNK_EN_NAME, NIN_EN_NAME, PCT_EN_NAME, PLD_EN_NAME, RDM_EN_NAME, RPR_EN_NAME, SAM_EN_NAME, SCH_EN_NAME, SGE_EN_NAME, SKS_STAT_EN_NAME, SMN_EN_NAME, SPS_STAT_EN_NAME, STR_STAT_EN_NAME, TEN_STAT_EN_NAME, TextDictionary, VPR_EN_NAME, WAR_EN_NAME, WD_STAT_EN_NAME, WHM_EN_NAME } from "../../const/languageTexts";
import { PlayerPower } from "./PlayerPower";
export const CRIT_BASE_PERCENT = 0.05;
export const CRIT_BASE_DAMAGE = 1.4;

interface SubStatInfo {
  name: string;
  value: number;
}

export function getStatNames(jobAbbrev: string, gcdName: string) {
  switch (jobAbbrev) {
    case PLD_EN_NAME:
    case WAR_EN_NAME:
    case DRK_EN_NAME:
    case GNB_EN_NAME:
      return [WD_STAT_EN_NAME, STR_STAT_EN_NAME, CRIT_STAT_EN_NAME, DH_STAT_EN_NAME, DET_STAT_EN_NAME, SKS_STAT_EN_NAME, TEN_STAT_EN_NAME, gcdName];
    case WHM_EN_NAME:
    case SCH_EN_NAME:
    case AST_EN_NAME:
    case SGE_EN_NAME:
      return [WD_STAT_EN_NAME, MIND_STAT_EN_NAME, CRIT_STAT_EN_NAME, DH_STAT_EN_NAME, DET_STAT_EN_NAME, SPS_STAT_EN_NAME, gcdName];
    case DRG_EN_NAME:
    case MNK_EN_NAME:
    case SAM_EN_NAME:
    case RPR_EN_NAME:
      return [WD_STAT_EN_NAME, STR_STAT_EN_NAME, CRIT_STAT_EN_NAME, DH_STAT_EN_NAME, DET_STAT_EN_NAME, SKS_STAT_EN_NAME, gcdName];
    case NIN_EN_NAME:
    case VPR_EN_NAME:
    case BRD_EN_NAME:
    case MCH_EN_NAME:
    case DNC_EN_NAME:
      return [WD_STAT_EN_NAME, DEX_STAT_EN_NAME, CRIT_STAT_EN_NAME, DH_STAT_EN_NAME, DET_STAT_EN_NAME, SKS_STAT_EN_NAME, gcdName]
    case BLM_EN_NAME:
    case SMN_EN_NAME:
    case RDM_EN_NAME:
    case PCT_EN_NAME:
      return [WD_STAT_EN_NAME, INT_STAT_EN_NAME, CRIT_STAT_EN_NAME, DH_STAT_EN_NAME, DET_STAT_EN_NAME, SPS_STAT_EN_NAME, gcdName]
    default:
      return [];
  }
}

export function getStatWeightNames(jobAbbrev: string, gcdName: string) {
  let statNames = getStatNames(jobAbbrev, gcdName);
  return statNames.filter(statName => statName !== gcdName);
}

export function convertToSubStatInfos(totalStats: PlayerPower, LANGUAGE_TEXTS: TextDictionary) {
  let subStats: SubStatInfo[] = [];
  if (totalStats.criticalStrike > 0) {
    subStats.push({ name: LANGUAGE_TEXTS.CRIT_STAT_NAME, value: totalStats.criticalStrike });
  }
  if (totalStats.directHit > 0) {
    subStats.push({ name: LANGUAGE_TEXTS.DH_STAT_NAME, value: totalStats.directHit });
  }
  if (totalStats.determination > 0) {
    subStats.push({ name: LANGUAGE_TEXTS.DET_STAT_NAME, value: totalStats.determination });
  }
  if (totalStats.skillSpeed > 0) {
    subStats.push({ name: LANGUAGE_TEXTS.SKS_STAT_NAME, value: totalStats.skillSpeed });
  }
  if (totalStats.spellSpeed > 0) {
    subStats.push({ name: LANGUAGE_TEXTS.SPS_STAT_NAME, value: totalStats.spellSpeed });
  }
  if (totalStats.tenacity > 0) {
    subStats.push({ name: LANGUAGE_TEXTS.TEN_STAT_NAME, value: totalStats.tenacity });
  }

  return subStats;
}
