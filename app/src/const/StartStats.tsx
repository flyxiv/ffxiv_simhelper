interface RaceInfo {
  name: string;
  STR: number;
  DEX: number;
  INT: number;
  MND: number;
}

const MAIN_STAT_BASE_PER_JOB = {
  PLD: 100,
  WAR: 105,
  DRK: 105,
  GNB: 100,
  WHM: 115,
  SCH: 115,
  AST: 115,
  SGE: 115,
  MNK: 110,
  DRG: 115,
  NIN: 110,
  SAM: 112,
  RPR: 115,
  VPR: 100,
  BRD: 115,
  MCH: 115,
  DNC: 115,
  BLM: 115,
  SMN: 115,
  RDM: 115,
  PCT: 115,
};

const RACE_MAIN_STAT = {
  midlander: {
    name: "Midlander Hyur",
    STR: 22,
    DEX: 19,
    INT: 23,
    MND: 19,
  },
  highlander: {
    name: "Highlander Hyur",
    STR: 23,
    DEX: 20,
    INT: 18,
    MND: 20,
  },
  wildwood: {
    name: "Wildwood Elezen",
    STR: 20,
    DEX: 23,
    INT: 22,
    MND: 19,
  },
  duskwight: {
    name: "Duskwight Elezen",
    STR: 20,
    DEX: 22,
    INT: 23,
    MND: 21,
  },
  plainsfolk: {
    name: "Plainsfolk Lalafell",
    STR: 19,
    DEX: 23,
    INT: 22,
    MND: 20,
  },
  dunesfolk: {
    name: "Dunesfolk Lalafell",
    STR: 19,
    DEX: 21,
    INT: 22,
    MND: 23,
  },
  seekersun: {
    name: "Seeker of the Sun Miqo'te",
    STR: 22,
    DEX: 23,
    INT: 29,
    MND: 19,
  },
  keepermoon: {
    name: "Keeper of the Moon Miqo'te",
    STR: 19,
    DEX: 22,
    INT: 21,
    MND: 23,
  },
  seawolves: {
    name: "Sea Wolves Roegadyn",
    STR: 22,
    DEX: 19,
    INT: 18,
    MND: 21,
  },
  hellsguard: {
    name: "Hellsguard Roegadyn",
    STR: 20,
    DEX: 18,
    INT: 20,
    MND: 22,
  },
  raen: {
    name: "Raen Au Ra",
    STR: 19,
    DEX: 22,
    INT: 20,
    MND: 23,
  },
  xaela: {
    name: "Xaela Au Ra",
    STR: 23,
    DEX: 20,
    INT: 20,
    MND: 18,
  },
  helions: {
    name: "Helions Hrothgar",
    STR: 23,
    DEX: 17,
    INT: 17,
    MND: 23,
  },
  lost: {
    name: "The Lost Hrothgar",
    STR: 23,
    DEX: 17,
    INT: 17,
    MND: 23,
  },
  rava: {
    name: "Rava Viera",
    STR: 20,
    DEX: 23,
    INT: 21,
    MND: 21,
  },
  veena: {
    name: "Veena Viera",
    STR: 19,
    DEX: 20,
    INT: 23,
    MND: 22,
  },
};

export function getMainStatBaseByJobAndRace(job: string, race: string) {
  getMainStatByJob(job) + getJobMainStatForRace(job, race);
}

function getMainStatByJob(job: string) {
  switch (job) {
    case "PLD":
      return MAIN_STAT_BASE_PER_JOB.PLD;
  }
}
function getRaceMainStatByName(mainStatName: string, stats: RaceInfo): number {
  switch (mainStatName) {
    case "STR":
      return stats.STR;
    case "DEX":
      return stats.DEX;
    case "INT":
      return stats.INT;
    default:
      return stats.MND;
  }
}

function mainStatNameByJob(job: string): string {
  switch (job) {
    case "PLD":
    case "WAR":
    case "DRK":
    case "GNB":
    case "MNK":
    case "DRG":
    case "SAM":
    case "RPR":
      return "STR";
    case "NIN":
    case "VPR":
    case "BRD":
    case "MCH":
    case "DNC":
      return "DEX";
    case "WHM":
    case "AST":
    case "SCH":
    case "SGE":
      return "MND";
    default:
      return "INT";
  }
}
