import { AST_EN_NAME, BLM_EN_NAME, BRD_EN_NAME, CRIT_STAT_NAME, DET_STAT_NAME, DEX_STAT_NAME, DH_STAT_NAME, DNC_EN_NAME, DRG_EN_NAME, DRK_EN_NAME, GCD_NAME, GNB_EN_NAME, INT_STAT_NAME, MCH_EN_NAME, MIND_STAT_NAME, MNK_EN_NAME, NIN_EN_NAME, PCT_EN_NAME, PLD_EN_NAME, RDM_EN_NAME, RPR_EN_NAME, SAM_EN_NAME, SCH_EN_NAME, SGE_EN_NAME, SKS_STAT_NAME, SMN_EN_NAME, SPS_STAT_NAME, STR_STAT_NAME, TEN_STAT_NAME, VPR_EN_NAME, WAR_EN_NAME, WD_STAT_NAME, WHM_EN_NAME } from "../../const/languageTexts";
import { PlayerPower } from "./PlayerPower";
export const CRIT_BASE_PERCENT = 0.05;
export const CRIT_BASE_DAMAGE = 1.4;

interface SubStatInfo {
  name: string;
  value: number;
}

export const SubstatNames = [CRIT_STAT_NAME, DET_STAT_NAME, DH_STAT_NAME, SKS_STAT_NAME, SPS_STAT_NAME, TEN_STAT_NAME];

export function getStatNames(jobAbbrev: string) {
  switch (jobAbbrev) {
    case PLD_EN_NAME:
    case WAR_EN_NAME:
    case DRK_EN_NAME:
    case GNB_EN_NAME:
      return [WD_STAT_NAME, STR_STAT_NAME, CRIT_STAT_NAME, DH_STAT_NAME, DET_STAT_NAME, GCD_NAME, SKS_STAT_NAME, TEN_STAT_NAME];
    case WHM_EN_NAME:
    case SCH_EN_NAME:
    case AST_EN_NAME:
    case SGE_EN_NAME:
      return [WD_STAT_NAME, MIND_STAT_NAME, CRIT_STAT_NAME, DH_STAT_NAME, DET_STAT_NAME, GCD_NAME, SPS_STAT_NAME];
    case DRG_EN_NAME:
    case MNK_EN_NAME:
    case SAM_EN_NAME:
    case RPR_EN_NAME:
      return [WD_STAT_NAME, STR_STAT_NAME, CRIT_STAT_NAME, DH_STAT_NAME, DET_STAT_NAME, GCD_NAME, SKS_STAT_NAME];
    case NIN_EN_NAME:
    case VPR_EN_NAME:
    case BRD_EN_NAME:
    case MCH_EN_NAME:
    case DNC_EN_NAME:
      return [WD_STAT_NAME, DEX_STAT_NAME, CRIT_STAT_NAME, DH_STAT_NAME, DET_STAT_NAME, GCD_NAME, SKS_STAT_NAME]
    case BLM_EN_NAME:
    case SMN_EN_NAME:
    case RDM_EN_NAME:
    case PCT_EN_NAME:
      return [WD_STAT_NAME, INT_STAT_NAME, CRIT_STAT_NAME, DH_STAT_NAME, DET_STAT_NAME, GCD_NAME, SPS_STAT_NAME]
    default:
      return [];
  }
}

export function getStatWeightNames(jobAbbrev: string) {
  let statNames = getStatNames(jobAbbrev);
  return statNames.filter(statName => statName !== GCD_NAME);
}

export function convertToSubStatInfos(totalStats: PlayerPower) {
  let subStats: SubStatInfo[] = [];
  if (totalStats.criticalStrike > 0) {
    subStats.push({ name: CRIT_STAT_NAME, value: totalStats.criticalStrike });
  }
  if (totalStats.directHit > 0) {
    subStats.push({ name: DH_STAT_NAME, value: totalStats.directHit });
  }
  if (totalStats.determination > 0) {
    subStats.push({ name: DET_STAT_NAME, value: totalStats.determination });
  }
  if (totalStats.skillSpeed > 0) {
    subStats.push({ name: SKS_STAT_NAME, value: totalStats.skillSpeed });
  }
  if (totalStats.spellSpeed > 0) {
    subStats.push({ name: SPS_STAT_NAME, value: totalStats.spellSpeed });
  }
  if (totalStats.tenacity > 0) {
    subStats.push({ name: TEN_STAT_NAME, value: totalStats.tenacity });
  }

  return subStats;
}
