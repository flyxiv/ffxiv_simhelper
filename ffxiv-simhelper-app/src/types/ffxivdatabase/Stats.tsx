import { AppLanguageTexts } from "../../const/languageTexts";
import { PlayerPower } from "./PlayerPower";
export const CRIT_BASE_PERCENT = 0.05;
export const CRIT_BASE_DAMAGE = 1.4;

interface SubStatInfo {
  name: string;
  value: number;
}

export const loadSubStatNames = () => {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  const SubstatNames = [LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, LANGUAGE_TEXTS.DET_STAT_EN_NAME, LANGUAGE_TEXTS.DH_STAT_EN_NAME, LANGUAGE_TEXTS.SKS_STAT_EN_NAME, LANGUAGE_TEXTS.SPS_STAT_EN_NAME, LANGUAGE_TEXTS.TEN_STAT_EN_NAME, LANGUAGE_TEXTS.GCD_NAME];
  return { SubstatNames };

}


export function getStatNames(jobAbbrev: string) {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  switch (jobAbbrev) {
    case LANGUAGE_TEXTS.PLD_EN_NAME:
    case LANGUAGE_TEXTS.WAR_EN_NAME:
    case LANGUAGE_TEXTS.DRK_EN_NAME:
    case LANGUAGE_TEXTS.GNB_EN_NAME:
      return [LANGUAGE_TEXTS.WD_STAT_EN_NAME, LANGUAGE_TEXTS.STR_STAT_EN_NAME, LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, LANGUAGE_TEXTS.DH_STAT_EN_NAME, LANGUAGE_TEXTS.DET_STAT_EN_NAME, LANGUAGE_TEXTS.SKS_STAT_EN_NAME, LANGUAGE_TEXTS.TEN_STAT_EN_NAME, LANGUAGE_TEXTS.GCD_NAME];
    case LANGUAGE_TEXTS.WHM_EN_NAME:
    case LANGUAGE_TEXTS.SCH_EN_NAME:
    case LANGUAGE_TEXTS.AST_EN_NAME:
    case LANGUAGE_TEXTS.SGE_EN_NAME:
      return [LANGUAGE_TEXTS.WD_STAT_EN_NAME, LANGUAGE_TEXTS.MIND_STAT_EN_NAME, LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, LANGUAGE_TEXTS.DH_STAT_EN_NAME, LANGUAGE_TEXTS.DET_STAT_EN_NAME, LANGUAGE_TEXTS.SPS_STAT_EN_NAME, LANGUAGE_TEXTS.GCD_NAME];
    case LANGUAGE_TEXTS.DRG_EN_NAME:
    case LANGUAGE_TEXTS.MNK_EN_NAME:
    case LANGUAGE_TEXTS.SAM_EN_NAME:
    case LANGUAGE_TEXTS.RPR_EN_NAME:
      return [LANGUAGE_TEXTS.WD_STAT_EN_NAME, LANGUAGE_TEXTS.STR_STAT_EN_NAME, LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, LANGUAGE_TEXTS.DH_STAT_EN_NAME, LANGUAGE_TEXTS.DET_STAT_EN_NAME, LANGUAGE_TEXTS.SKS_STAT_EN_NAME, LANGUAGE_TEXTS.GCD_NAME];
    case LANGUAGE_TEXTS.NIN_EN_NAME:
    case LANGUAGE_TEXTS.VPR_EN_NAME:
    case LANGUAGE_TEXTS.BRD_EN_NAME:
    case LANGUAGE_TEXTS.MCH_EN_NAME:
    case LANGUAGE_TEXTS.DNC_EN_NAME:
      return [LANGUAGE_TEXTS.WD_STAT_EN_NAME, LANGUAGE_TEXTS.DEX_STAT_EN_NAME, LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, LANGUAGE_TEXTS.DH_STAT_EN_NAME, LANGUAGE_TEXTS.DET_STAT_EN_NAME, LANGUAGE_TEXTS.SKS_STAT_EN_NAME, LANGUAGE_TEXTS.GCD_NAME]
    case LANGUAGE_TEXTS.BLM_EN_NAME:
    case LANGUAGE_TEXTS.SMN_EN_NAME:
    case LANGUAGE_TEXTS.RDM_EN_NAME:
    case LANGUAGE_TEXTS.PCT_EN_NAME:
      return [LANGUAGE_TEXTS.WD_STAT_EN_NAME, LANGUAGE_TEXTS.INT_STAT_EN_NAME, LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, LANGUAGE_TEXTS.DH_STAT_EN_NAME, LANGUAGE_TEXTS.DET_STAT_EN_NAME, LANGUAGE_TEXTS.SPS_STAT_EN_NAME, LANGUAGE_TEXTS.GCD_NAME]
    default:
      return [];
  }
}

export function getStatWeightNames(jobAbbrev: string) {
  let statNames = getStatNames(jobAbbrev);
  let LANGUAGE_TEXTS = AppLanguageTexts();
  return statNames.filter(statName => statName !== LANGUAGE_TEXTS.GCD_NAME);
}

export function convertToSubStatInfos(totalStats: PlayerPower) {
  let subStats: SubStatInfo[] = [];
  let LANGUAGE_TEXTS = AppLanguageTexts();
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
