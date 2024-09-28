import { CURRENT_MAX_ITEM_LEVEL } from "../types/ffxivdatabase/Equipment";
import { calculatePowerByStat, isCaster } from "../types/ffxivdatabase/ItemSet";
import { defaultPlayerPower } from "../types/ffxivdatabase/PlayerPower";
import {
  AST_EN_NAME,
  BLM_EN_NAME,
  BRD_EN_NAME,
  CRIT_STAT_NAME,
  DET_STAT_NAME,
  DH_STAT_NAME,
  DNC_EN_NAME,
  DRG_EN_NAME,
  DRK_EN_NAME,
  GNB_EN_NAME,
  MCH_EN_NAME,
  MNK_EN_NAME,
  NIN_EN_NAME,
  PCT_EN_NAME,
  PLD_EN_NAME,
  RDM_EN_NAME,
  RPR_EN_NAME,
  SAM_EN_NAME,
  SCH_EN_NAME,
  SGE_EN_NAME,
  SKS_STAT_NAME,
  SMN_EN_NAME,
  SPS_STAT_NAME,
  VPR_EN_NAME,
  WAR_EN_NAME,
  WHM_EN_NAME,
} from "./languageTexts";

export const DEFAULT_WEAPON_DAMAGE = 100;
export const DEFAULT_MAIN_STAT_NON_TANK = 440;
export const DEFAULT_MAIN_STAT_TANK = 440;
export const DEFAULT_CRITICAL_STRIKE = 420;
export const DEFAULT_DIRECT_HIT = 420;
export const DEFAULT_DETERMINATION = 440;
export const DEFAULT_SPEED = 420;
export const DEFAULT_TENACITY = 420;

export function calculateIlvlAdjustment(partyiLvl: number) {
  return 1 - (CURRENT_MAX_ITEM_LEVEL - partyiLvl) / 100;
}

export const mapJobAbbrevToJobBisEquipments = (jobAbbrev: string) => {
  switch (jobAbbrev) {
    case PLD_EN_NAME:
      return copyBisSettings(PLD_BIS_SETTINGS);
    case WAR_EN_NAME:
      return copyBisSettings(WAR_BIS_SETTINGS);
    case DRK_EN_NAME:
      return copyBisSettings(DRK_BIS_SETTINGS);
    case GNB_EN_NAME:
      return copyBisSettings(GNB_BIS_SETTINGS);
    case WHM_EN_NAME:
      return copyBisSettings(WHM_BIS_SETTINGS);
    case SCH_EN_NAME:
      return copyBisSettings(SCH_BIS_SETTINGS);
    case AST_EN_NAME:
      return copyBisSettings(AST_BIS_SETTINGS);
    case SGE_EN_NAME:
      return copyBisSettings(SGE_BIS_SETTINGS);
    case DRG_EN_NAME:
      return copyBisSettings(DRG_BIS_SETTINGS);
    case MNK_EN_NAME:
      return copyBisSettings(MNK_BIS_SETTINGS);
    case NIN_EN_NAME:
      return copyBisSettings(NIN_BIS_SETTINGS);
    case SAM_EN_NAME:
      return copyBisSettings(SAM_BIS_SETTINGS);
    case RPR_EN_NAME:
      return copyBisSettings(RPR_BIS_SETTINGS);
    case VPR_EN_NAME:
      return copyBisSettings(VPR_BIS_SETTINGS);
    case BRD_EN_NAME:
      return copyBisSettings(BRD_BIS_SETTINGS);
    case DNC_EN_NAME:
      return copyBisSettings(DNC_BIS_SETTINGS);
    case MCH_EN_NAME:
      return copyBisSettings(MCH_BIS_SETTINGS);
    case BLM_EN_NAME:
      return copyBisSettings(BLM_BIS_SETTINGS);
    case SMN_EN_NAME:
      return copyBisSettings(SMN_BIS_SETTINGS);
    case RDM_EN_NAME:
      return copyBisSettings(RDM_BIS_SETTINGS);
    case PCT_EN_NAME:
      return copyBisSettings(PCT_BIS_SETTINGS);
    default:
      return undefined;
  }
};

export interface PlayerStats {
  weaponDamage: number;
  mainStat: number;
  criticalStrike: number;
  directHit: number;
  determination: number;
  speed: number;
  tenacity: number;
}

interface BisSettings {
  foodId: number;
  itemSet: number[];
  gearSetMaterias: { statName: string; maxValue: number; effectiveValue: number }[][];
}

function copyBisSettings(bisSettings: BisSettings) {
  return {
    foodId: bisSettings.foodId,
    itemSet: [...bisSettings.itemSet],
    gearSetMaterias: bisSettings.gearSetMaterias.map((materia) => [...materia]),
  };
}

export const PLD_BIS_SETTINGS = {
  foodId: 652,
  itemSet: [43101, 43123, 43047, 43125, 43126, 43050, 43081, 43163, 43091, 43096, 43173, 43122],
  gearSetMaterias: [
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],

    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}

export const WAR_BIS_SETTINGS = {
  foodId: 652,
  itemSet: [43103, 43123, 43047, 43125, 43126, 43050, 43081, 43163, 43091, 43096, 43173, -1],
  gearSetMaterias: [
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}


export const DRK_BIS_SETTINGS = {
  foodId: 652,
  itemSet: [43111, 43123, 43047, 43125, 43126, 43050, 43081, 43163, 43091, 43096, 43173, -1],
  gearSetMaterias: [
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}

export const GNB_BIS_SETTINGS = {
  foodId: 652,
  itemSet: [43116, 43123, 43047, 43125, 43126, 43050, 43081, 43163, 43091, 43096, 43173, -1],
  gearSetMaterias: [
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}


export const WHM_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43106, 43071, 43149, 42919, 43074, 43152, 43161, 43089, 43171, 43099, 43176, -1],
  gearSetMaterias: [
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 }
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 18, effectiveValue: 18 },
      { statName: DH_STAT_NAME, maxValue: 18, effectiveValue: 18 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    []
  ]
}

export const SCH_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43109, 43071, 43149, 42919, 43074, 43152, 43161, 43089, 43171, 43099, 43176, -1],
  gearSetMaterias: [
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 }
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 18, effectiveValue: 18 },
      { statName: DH_STAT_NAME, maxValue: 18, effectiveValue: 18 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    []
  ]
}


export const AST_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43113, 43071, 43149, 42919, 43074, 43152, 43161, 43089, 43171, 43099, 43176, -1],
  gearSetMaterias: [
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 }
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 18, effectiveValue: 18 },
      { statName: DH_STAT_NAME, maxValue: 18, effectiveValue: 18 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    []
  ]
}


export const SGE_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43119, 43071, 43149, 42919, 43074, 43152, 43161, 43089, 43171, 43099, 43176, -1],
  gearSetMaterias: [
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 }
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 18, effectiveValue: 18 },
      { statName: DH_STAT_NAME, maxValue: 18, effectiveValue: 18 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    []
  ]
}

export const DRG_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43104, 43051, 43129, 43053, 43054, 43132, 43159, 43087, 43169, 43097, 43174, -1],
  gearSetMaterias: [
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}


export const MNK_BIS_SETTINGS = {
  foodId: 654,
  itemSet: [43102, 43056, 43057, 43058, 43136, 43137, 43082, 43087, 43169, 43174, 43097, -1],
  gearSetMaterias: [
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: SKS_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: SKS_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],

    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: SKS_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}


export const NIN_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43110, 43143, 43144, 43068, 43069, 43147, 43083, 43165, 43093, 43098, 43175, -1],
  gearSetMaterias: [
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],

    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}


export const SAM_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43114, 43056, 43057, 43058, 43136, 43137, 43159, 43087, 43169, 43174, 43097, -1],
  gearSetMaterias: [
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: SKS_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: SKS_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],

    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}


export const RPR_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43118, 43051, 43129, 43053, 43054, 43132, 43159, 43087, 43169, 43097, 43174, -1],
  gearSetMaterias: [
    [
      { statName: SKS_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}



export const VPR_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43120, 43143, 43144, 43068, 43069, 43147, 43083, 43165, 43093, 43098, 43175, -1],
  gearSetMaterias: [
    [
      { statName: SKS_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: SKS_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}

export const BRD_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43105, 43138, 43062, 43063, 43141, 43142, 43083, 43165, 43093, 43098, 43175, -1],
  gearSetMaterias: [
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],

    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}

export const DNC_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43117, 43138, 43062, 43063, 43141, 43142, 43083, 43165, 43093, 43098, 43175, -1],
  gearSetMaterias: [
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],

    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}


export const MCH_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43112, 43138, 43062, 43063, 43141, 43142, 43083, 43165, 43093, 43098, 43175, -1],
  gearSetMaterias: [
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],

    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}

export const BLM_BIS_SETTINGS = {
  foodId: 659,
  itemSet: [43107, 43076, 43154, 43155, 43079, 43080, 43162, 43090, 43172, 43100, 43177, -1],
  gearSetMaterias: [
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: SPS_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: SPS_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],

    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}

export const SMN_BIS_SETTINGS = {
  foodId: 652,
  itemSet: [43108, 43076, 43154, 43078, 43079, 43157, 43162, 43090, 43172, 43100, 43177, -1],
  gearSetMaterias: [
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: SPS_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: SPS_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}


export const RDM_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43115, 43076, 43154, 43078, 43079, 43157, 43162, 43090, 43172, 43100, 43177, -1],
  gearSetMaterias: [
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}


export const PCT_BIS_SETTINGS = {
  foodId: 655,
  itemSet: [43121, 43076, 43154, 43078, 43079, 43157, 43162, 43090, 43172, 43100, 43177, -1],
  gearSetMaterias: [
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: DH_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [
      { statName: CRIT_STAT_NAME, maxValue: 54, effectiveValue: 54 },
      { statName: DET_STAT_NAME, maxValue: 54, effectiveValue: 54 },
    ],
    [

    ]
  ]
}

export function playerStatToPlayerPower(
  playerStats: PlayerStats,
  jobAbbrev: string
) {
  let power = defaultPlayerPower();
  power.weaponDamage = playerStats.weaponDamage;
  power.mainStat = playerStats.mainStat;
  power.criticalStrike = playerStats.criticalStrike;
  power.directHit = playerStats.directHit;
  power.determination = playerStats.determination;

  if (isCaster(jobAbbrev)) {
    power.spellSpeed = playerStats.speed;
  } else {
    power.skillSpeed = playerStats.speed;
  }
  power.tenacity = playerStats.tenacity;

  calculatePowerByStat(power, jobAbbrev);
  return power;
}
