import { PlayerPower } from "./PlayerPower";
export const CRIT_BASE_PERCENT = 0.05;
export const CRIT_BASE_DAMAGE = 1.4;

interface SubStatInfo {
  name: string;
  value: number;
}

export const SubstatNames = ["CRT", "DET", "DH", "SKS", "SPS", "TEN"];

export function getStatNames(jobAbbrev: string) {
  switch (jobAbbrev) {
    case "PLD":
    case "WAR":
    case "DRK":
    case "GNB":
      return ["WD", "STR", "CRT", "DH", "DET", "GCD", "SKS", "TEN"];
    case "WHM":
    case "SCH":
    case "AST":
    case "SGE":
      return ["WD", "MND", "CRT", "DH", "DET", "GCD", "SPS"];
    case "DRG":
    case "MNK":
    case "SAM":
    case "RPR":
      return ["WD", "STR", "CRT", "DH", "DET", "GCD", "SKS"];
    case "NIN":
    case "VPR":
    case "BRD":
    case "MCH":
    case "DNC":
      return ["WD", "DEX", "CRT", "DH", "DET", "GCD", "SKS"];
    case "BLM":
    case "SMN":
    case "RDM":
    case "PCT":
      return ["WD", "INT", "CRT", "DH", "DET", "GCD", "SPS"];
    default:
      return [];
  }
}

export function convertToSubStatInfos(totalStats: PlayerPower) {
  let subStats: SubStatInfo[] = [];
  if (totalStats.criticalStrike > 0) {
    subStats.push({ name: "CRT", value: totalStats.criticalStrike });
  }
  if (totalStats.directHit > 0) {
    subStats.push({ name: "DH", value: totalStats.directHit });
  }
  if (totalStats.determination > 0) {
    subStats.push({ name: "DET", value: totalStats.determination });
  }
  if (totalStats.skillSpeed > 0) {
    subStats.push({ name: "SKS", value: totalStats.skillSpeed });
  }
  if (totalStats.spellSpeed > 0) {
    subStats.push({ name: "SPS", value: totalStats.spellSpeed });
  }
  if (totalStats.tenacity > 0) {
    subStats.push({ name: "TEN", value: totalStats.tenacity });
  }

  return subStats;
}
