interface RaceInfo {
  STR: number;
  DEX: number;
  INT: number;
  MND: number;
}

const MAIN_STAT_BASE_PER_JOB = new Map([
  ["PLD", 100],
  ["WAR", 105],
  ["DRK", 105],
  ["GNB", 100],
  ["WHM", 115],
  ["SCH", 115],
  ["AST", 115],
  ["SGE", 115],
  ["MNK", 110],
  ["DRG", 115],
  ["NIN", 110],
  ["SAM", 112],
  ["RPR", 115],
  ["VPR", 100],
  ["BRD", 115],
  ["MCH", 115],
  ["DNC", 115],
  ["BLM", 115],
  ["SMN", 115],
  ["RDM", 115],
  ["PCT", 115],
]);

interface RaceInfo {
  STR: number;
  DEX: number;
  INT: number;
  MND: number;
}

export const RACES = [
  "Midlander Hyur",
  "Highlander Hyur",
  "Wildwood Elezen",
  "Duskwight Elezen",
  "Plainsfolk Lalafell",
  "Dunesfolk Lalafell",
  "Seeker of the Sun Miqo'te",
  "Keeper of the Moon Miqo'te",
  "Sea Wolves Roegadyn",
  "Hellsguard Roegadyn",
  "Raen Au Ra",
  "Xaela Au Ra",
  "Helions Hrothgar",
  "The Lost Hrothgar",
  "Rava Viera",
  "Veena Viera",
];

export const RACE_MAIN_STAT = new Map([
  ["Midlander Hyur", { STR: 22, DEX: 19, INT: 23, MND: 19 }],
  ["Highlander Hyur", { STR: 23, DEX: 20, INT: 18, MND: 20 }],
  ["Wildwood Elezen", { STR: 20, DEX: 23, INT: 22, MND: 19 }],
  ["Duskwight Elezen", { STR: 20, DEX: 22, INT: 23, MND: 21 }],
  [
    "Plainsfolk Lalafell",
    {
      STR: 19,
      DEX: 23,
      INT: 22,
      MND: 20,
    },
  ],
  ["Dunesfolk Lalafell", { STR: 19, DEX: 21, INT: 22, MND: 23 }],
  [
    "Seeker of the Sun Miqo'te",
    {
      STR: 22,
      DEX: 23,
      INT: 29,
      MND: 19,
    },
  ],
  [
    "Keeper of the Moon Miqo'te",
    {
      STR: 19,
      DEX: 22,
      INT: 21,
      MND: 23,
    },
  ],
  [
    "Sea Wolves Roegadyn",
    {
      STR: 22,
      DEX: 19,
      INT: 18,
      MND: 21,
    },
  ],
  [
    "Hellsguard Roegadyn",
    {
      STR: 20,
      DEX: 18,
      INT: 20,
      MND: 22,
    },
  ],
  [
    "Raen Au Ra",
    {
      STR: 19,
      DEX: 22,
      INT: 20,
      MND: 23,
    },
  ],
  [
    "Xaela Au Ra",
    {
      STR: 23,
      DEX: 20,
      INT: 20,
      MND: 18,
    },
  ],
  [
    "Helions Hrothgar",
    {
      STR: 23,
      DEX: 17,
      INT: 17,
      MND: 23,
    },
  ],
  [
    "The Lost Hrothgar",
    {
      STR: 23,
      DEX: 17,
      INT: 17,
      MND: 23,
    },
  ],
  [
    "Rava Viera",
    {
      STR: 20,
      DEX: 23,
      INT: 21,
      MND: 21,
    },
  ],
  [
    "Veena Viera",
    {
      STR: 19,
      DEX: 20,
      INT: 23,
      MND: 22,
    },
  ],
]);

export function getBaseMainStat(jobAbbrev: string, race: string): number {
  let mainStatBase = MAIN_STAT_BASE_PER_JOB.get(jobAbbrev);
  if (mainStatBase === undefined) {
    return 0;
  }
  let mainStat = Math.floor(mainStatBase * 4.4 - 20);

  let raceMainStat = RACE_MAIN_STAT.get(race);
  if (raceMainStat === undefined) {
    return mainStat;
  }

  return (
    mainStat +
    getRaceMainStatByName(getMainStatNameByJob(jobAbbrev), raceMainStat)
  );
}

export function getMainStatOfRace(race: string, mainStatName: string): number {
  let stats = RACE_MAIN_STAT.get(race);
  if (stats === undefined) {
    return 0;
  }

  return getRaceMainStatByName(mainStatName, stats);
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

export function getMainStatNameByJob(job: string): string {
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
