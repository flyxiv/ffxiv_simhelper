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
