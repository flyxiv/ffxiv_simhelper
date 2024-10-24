import { AST_EN_NAME, BLM_EN_NAME, BRD_EN_NAME, DEX_STAT_EN_NAME, DNC_EN_NAME, DRG_EN_NAME, DRK_EN_NAME, DUNESFOLK_LALAFELL_EN_NAME, DUSKWIGHT_ELEZEN_EN_NAME, GNB_EN_NAME, HELIONS_HROTHGAR_EN_NAME, HELLSGUARD_ROEGADYN_EN_NAME, HIGHLANDER_HYUR_EN_NAME, INT_STAT_EN_NAME, KEEPER_OF_THE_MOON_MIQOTE_EN_NAME, MCH_EN_NAME, MIDLANDER_HYUR_EN_NAME, MIND_STAT_EN_NAME, MNK_EN_NAME, NIN_EN_NAME, PCT_EN_NAME, PLAINSFOLK_LALAFELL_EN_NAME, PLD_EN_NAME, RAEN_AU_RA_EN_NAME, RAVA_VIERA_EN_NAME, RDM_EN_NAME, RPR_EN_NAME, SAM_EN_NAME, SCH_EN_NAME, SEA_WOLVES_ROEGADYN_EN_NAME, SEEKER_OF_THE_SUN_MIQOTE_EN_NAME, SGE_EN_NAME, SMN_EN_NAME, STR_STAT_EN_NAME, TextDictionary, THE_LOST_HROTHGAR_EN_NAME, VEENA_VIERA_EN_NAME, VPR_EN_NAME, WAR_EN_NAME, WHM_EN_NAME, WILDWOOD_ELEZEN_EN_NAME, XAELA_AU_RA_EN_NAME } from "./languageTexts";

interface RaceInfo {
  STR: number;
  DEX: number;
  INT: number;
  MND: number;
}


export const RACE_MAIN_STAT = new Map([
  [MIDLANDER_HYUR_EN_NAME, { STR: 22, DEX: 19, INT: 23, MND: 19 }],
  [HIGHLANDER_HYUR_EN_NAME, { STR: 23, DEX: 20, INT: 18, MND: 20 }],
  [WILDWOOD_ELEZEN_EN_NAME, { STR: 20, DEX: 23, INT: 22, MND: 19 }],
  [DUSKWIGHT_ELEZEN_EN_NAME, { STR: 20, DEX: 22, INT: 23, MND: 21 }],
  [
    PLAINSFOLK_LALAFELL_EN_NAME,
    {
      STR: 19,
      DEX: 23,
      INT: 22,
      MND: 20,
    },
  ],
  [DUNESFOLK_LALAFELL_EN_NAME, { STR: 19, DEX: 21, INT: 22, MND: 23 }],
  [
    SEEKER_OF_THE_SUN_MIQOTE_EN_NAME,
    {
      STR: 22,
      DEX: 23,
      INT: 29,
      MND: 19,
    },
  ],
  [
    KEEPER_OF_THE_MOON_MIQOTE_EN_NAME,
    {
      STR: 19,
      DEX: 22,
      INT: 21,
      MND: 23,
    },
  ],
  [
    SEA_WOLVES_ROEGADYN_EN_NAME,
    {
      STR: 22,
      DEX: 19,
      INT: 18,
      MND: 21,
    },
  ],
  [
    HELLSGUARD_ROEGADYN_EN_NAME,
    {
      STR: 20,
      DEX: 18,
      INT: 20,
      MND: 22,
    },
  ],
  [
    RAEN_AU_RA_EN_NAME,
    {
      STR: 19,
      DEX: 22,
      INT: 20,
      MND: 23,
    },
  ],
  [
    XAELA_AU_RA_EN_NAME,
    {
      STR: 23,
      DEX: 20,
      INT: 20,
      MND: 18,
    },
  ],
  [
    HELIONS_HROTHGAR_EN_NAME,
    {
      STR: 23,
      DEX: 17,
      INT: 17,
      MND: 23,
    },
  ],
  [
    THE_LOST_HROTHGAR_EN_NAME,
    {
      STR: 23,
      DEX: 17,
      INT: 17,
      MND: 23,
    },
  ],
  [
    RAVA_VIERA_EN_NAME,
    {
      STR: 20,
      DEX: 23,
      INT: 21,
      MND: 21,
    },
  ],
  [
    VEENA_VIERA_EN_NAME,
    {
      STR: 19,
      DEX: 20,
      INT: 23,
      MND: 22,
    },
  ],
]);



export const MAIN_STAT_BASE_PER_JOB = new Map([
  [PLD_EN_NAME, 100],
  [WAR_EN_NAME, 105],
  [DRK_EN_NAME, 105],
  [GNB_EN_NAME, 100],
  [WHM_EN_NAME, 115],
  [SCH_EN_NAME, 115],
  [AST_EN_NAME, 115],
  [SGE_EN_NAME, 115],
  [MNK_EN_NAME, 110],
  [DRG_EN_NAME, 115],
  [NIN_EN_NAME, 110],
  [SAM_EN_NAME, 112],
  [RPR_EN_NAME, 115],
  [VPR_EN_NAME, 100],
  [BRD_EN_NAME, 115],
  [MCH_EN_NAME, 115],
  [DNC_EN_NAME, 115],
  [BLM_EN_NAME, 115],
  [SMN_EN_NAME, 115],
  [RDM_EN_NAME, 115],
  [PCT_EN_NAME, 115],
]);



interface RaceInfo {
  STR: number;
  DEX: number;
  INT: number;
  MND: number;
}

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
    getRaceMainStatByName(getMainStatKeyByJob(jobAbbrev), raceMainStat)
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
    case STR_STAT_EN_NAME:
      return stats.STR;
    case DEX_STAT_EN_NAME:
      return stats.DEX;
    case INT_STAT_EN_NAME:
      return stats.INT;
    default:
      return stats.MND;
  }
}

export function getMainStatKeyByJob(job: string): string {
  switch (job) {
    case PLD_EN_NAME:
    case WAR_EN_NAME:
    case DRK_EN_NAME:
    case GNB_EN_NAME:
    case MNK_EN_NAME:
    case DRG_EN_NAME:
    case SAM_EN_NAME:
    case RPR_EN_NAME:
      return STR_STAT_EN_NAME;
    case NIN_EN_NAME:
    case VPR_EN_NAME:
    case BRD_EN_NAME:
    case MCH_EN_NAME:
    case DNC_EN_NAME:
      return DEX_STAT_EN_NAME;
    case WHM_EN_NAME:
    case SCH_EN_NAME:
    case AST_EN_NAME:
    case SGE_EN_NAME:
      return MIND_STAT_EN_NAME;
    default:
      return INT_STAT_EN_NAME;
  }
}

export function getMainStatNameByJob(job: string, LANGUAGE_TEXTS: TextDictionary): string {
  switch (job) {
    case PLD_EN_NAME:
    case WAR_EN_NAME:
    case DRK_EN_NAME:
    case GNB_EN_NAME:
    case MNK_EN_NAME:
    case DRG_EN_NAME:
    case SAM_EN_NAME:
    case RPR_EN_NAME:
      return LANGUAGE_TEXTS.STR_STAT_NAME;
    case NIN_EN_NAME:
    case VPR_EN_NAME:
    case BRD_EN_NAME:
    case MCH_EN_NAME:
    case DNC_EN_NAME:
      return LANGUAGE_TEXTS.DEX_STAT_NAME;
    case WHM_EN_NAME:
    case SCH_EN_NAME:
    case AST_EN_NAME:
    case SGE_EN_NAME:
      return LANGUAGE_TEXTS.MIND_STAT_NAME;
    default:
      return LANGUAGE_TEXTS.INT_STAT_NAME;
  }
}
