import { LANGUAGE_TEXTS } from "./languageTexts";

interface RaceInfo {
  STR: number;
  DEX: number;
  INT: number;
  MND: number;
}

const MAIN_STAT_BASE_PER_JOB = new Map([
  [LANGUAGE_TEXTS.PLD_EN_NAME, 100],
  [LANGUAGE_TEXTS.WAR_EN_NAME, 105],
  [LANGUAGE_TEXTS.DRK_EN_NAME, 105],
  [LANGUAGE_TEXTS.GNB_EN_NAME, 100],
  [LANGUAGE_TEXTS.WHM_EN_NAME, 115],
  [LANGUAGE_TEXTS.SCH_EN_NAME, 115],
  [LANGUAGE_TEXTS.AST_EN_NAME, 115],
  [LANGUAGE_TEXTS.SGE_EN_NAME, 115],
  [LANGUAGE_TEXTS.MNK_EN_NAME, 110],
  [LANGUAGE_TEXTS.DRG_EN_NAME, 115],
  [LANGUAGE_TEXTS.NIN_EN_NAME, 110],
  [LANGUAGE_TEXTS.SAM_EN_NAME, 112],
  [LANGUAGE_TEXTS.RPR_EN_NAME, 115],
  [LANGUAGE_TEXTS.VPR_EN_NAME, 100],
  [LANGUAGE_TEXTS.BRD_EN_NAME, 115],
  [LANGUAGE_TEXTS.MCH_EN_NAME, 115],
  [LANGUAGE_TEXTS.DNC_EN_NAME, 115],
  [LANGUAGE_TEXTS.BLM_EN_NAME, 115],
  [LANGUAGE_TEXTS.SMN_EN_NAME, 115],
  [LANGUAGE_TEXTS.RDM_EN_NAME, 115],
  [LANGUAGE_TEXTS.PCT_EN_NAME, 115],
]);

interface RaceInfo {
  STR: number;
  DEX: number;
  INT: number;
  MND: number;
}


export const RACE_MAIN_STAT = new Map([
  [LANGUAGE_TEXTS.MIDLANDER_HYUR_EN_NAME, { STR: 22, DEX: 19, INT: 23, MND: 19 }],
  [LANGUAGE_TEXTS.HIGHLANDER_HYUR_EN_NAME, { STR: 23, DEX: 20, INT: 18, MND: 20 }],
  [LANGUAGE_TEXTS.WILDWOOD_ELEZEN_EN_NAME, { STR: 20, DEX: 23, INT: 22, MND: 19 }],
  [LANGUAGE_TEXTS.DUSKWIGHT_ELEZEN_EN_NAME, { STR: 20, DEX: 22, INT: 23, MND: 21 }],
  [
    LANGUAGE_TEXTS.PLAINSFOLK_LALAFELL_EN_NAME,
    {
      STR: 19,
      DEX: 23,
      INT: 22,
      MND: 20,
    },
  ],
  [LANGUAGE_TEXTS.DUNESFOLK_LALAFELL_EN_NAME, { STR: 19, DEX: 21, INT: 22, MND: 23 }],
  [
    LANGUAGE_TEXTS.SEEKER_OF_THE_SUN_MIQOTE_EN_NAME,
    {
      STR: 22,
      DEX: 23,
      INT: 29,
      MND: 19,
    },
  ],
  [
    LANGUAGE_TEXTS.KEEPER_OF_THE_MOON_MIQOTE_EN_NAME,
    {
      STR: 19,
      DEX: 22,
      INT: 21,
      MND: 23,
    },
  ],
  [
    LANGUAGE_TEXTS.SEA_WOLVES_ROEGADYN_EN_NAME,
    {
      STR: 22,
      DEX: 19,
      INT: 18,
      MND: 21,
    },
  ],
  [
    LANGUAGE_TEXTS.HELLSGUARD_ROEGADYN_EN_NAME,
    {
      STR: 20,
      DEX: 18,
      INT: 20,
      MND: 22,
    },
  ],
  [
    LANGUAGE_TEXTS.RAEN_AU_RA_EN_NAME,
    {
      STR: 19,
      DEX: 22,
      INT: 20,
      MND: 23,
    },
  ],
  [
    LANGUAGE_TEXTS.XAELA_AU_RA_EN_NAME,
    {
      STR: 23,
      DEX: 20,
      INT: 20,
      MND: 18,
    },
  ],
  [
    LANGUAGE_TEXTS.HELIONS_HROTHGAR_EN_NAME,
    {
      STR: 23,
      DEX: 17,
      INT: 17,
      MND: 23,
    },
  ],
  [
    LANGUAGE_TEXTS.THE_LOST_HROTHGAR_EN_NAME,
    {
      STR: 23,
      DEX: 17,
      INT: 17,
      MND: 23,
    },
  ],
  [
    LANGUAGE_TEXTS.RAVA_VIERA_EN_NAME,
    {
      STR: 20,
      DEX: 23,
      INT: 21,
      MND: 21,
    },
  ],
  [
    LANGUAGE_TEXTS.VEENA_VIERA_EN_NAME,
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
    case LANGUAGE_TEXTS.STR_STAT_NAME:
      return stats.STR;
    case LANGUAGE_TEXTS.DEX_STAT_NAME:
      return stats.DEX;
    case LANGUAGE_TEXTS.INT_STAT_NAME:
      return stats.INT;
    default:
      return stats.MND;
  }
}

export function getMainStatNameByJob(job: string): string {
  switch (job) {
    case LANGUAGE_TEXTS.PLD_EN_NAME:
    case LANGUAGE_TEXTS.WAR_EN_NAME:
    case LANGUAGE_TEXTS.DRK_EN_NAME:
    case LANGUAGE_TEXTS.GNB_EN_NAME:
    case LANGUAGE_TEXTS.MNK_EN_NAME:
    case LANGUAGE_TEXTS.DRG_EN_NAME:
    case LANGUAGE_TEXTS.SAM_EN_NAME:
    case LANGUAGE_TEXTS.RPR_EN_NAME:
      return LANGUAGE_TEXTS.STR_STAT_NAME;
    case LANGUAGE_TEXTS.NIN_EN_NAME:
    case LANGUAGE_TEXTS.VPR_EN_NAME:
    case LANGUAGE_TEXTS.BRD_EN_NAME:
    case LANGUAGE_TEXTS.MCH_EN_NAME:
    case LANGUAGE_TEXTS.DNC_EN_NAME:
      return LANGUAGE_TEXTS.DEX_STAT_NAME;
    case LANGUAGE_TEXTS.WHM_EN_NAME:
    case LANGUAGE_TEXTS.SCH_EN_NAME:
    case LANGUAGE_TEXTS.AST_EN_NAME:
    case LANGUAGE_TEXTS.SGE_EN_NAME:
      return LANGUAGE_TEXTS.MIND_STAT_NAME;
    default:
      return LANGUAGE_TEXTS.INT_STAT_NAME;
  }
}
