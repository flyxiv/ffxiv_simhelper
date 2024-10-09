import { LanguageMode } from "../LanguageContext";
import { loadMinMaxEquipmentItemLevels } from "../types/ffxivdatabase/Equipment";
import { calculatePowerByStat, isCaster } from "../types/ffxivdatabase/ItemSet";
import { defaultPlayerPower } from "../types/ffxivdatabase/PlayerPower";
import {
  AppLanguageTexts,
} from "./languageTexts";

export const DEFAULT_WEAPON_DAMAGE = 100;
export const DEFAULT_MAIN_STAT_NON_TANK = 440;
export const DEFAULT_MAIN_STAT_TANK = 440;
export const DEFAULT_CRITICAL_STRIKE = 420;
export const DEFAULT_DIRECT_HIT = 420;
export const DEFAULT_DETERMINATION = 440;
export const DEFAULT_SPEED = 420;
export const DEFAULT_TENACITY = 420;

const loadPartyMaxItemLevel = () => {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  let CURRENT_MAX_PARTY_ILVL = LANGUAGE_TEXTS.language === LanguageMode.ENGLISH_MODE ? 730 : 730;

  return { CURRENT_MAX_PARTY_ILVL };
}

export function calculateIlvlAdjustment(partyiLvl: number) {
  let { CURRENT_MAX_PARTY_ILVL } = loadPartyMaxItemLevel();
  return 1 - (CURRENT_MAX_PARTY_ILVL - partyiLvl) / 100;
}

export const mapJobAbbrevToJobBisEquipments = (jobAbbrev: string) => {
  const LANGUAGE_TEXTS = AppLanguageTexts();
  const BIS_SETTINGS = loadBis();

  switch (jobAbbrev) {
    case LANGUAGE_TEXTS.PLD_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.PLD_BIS_SETTINGS);
    case LANGUAGE_TEXTS.WAR_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.WAR_BIS_SETTINGS);
    case LANGUAGE_TEXTS.DRK_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.DRK_BIS_SETTINGS);
    case LANGUAGE_TEXTS.GNB_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.GNB_BIS_SETTINGS);
    case LANGUAGE_TEXTS.WHM_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.WHM_BIS_SETTINGS);
    case LANGUAGE_TEXTS.SCH_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.SCH_BIS_SETTINGS);
    case LANGUAGE_TEXTS.AST_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.AST_BIS_SETTINGS);
    case LANGUAGE_TEXTS.SGE_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.SGE_BIS_SETTINGS);
    case LANGUAGE_TEXTS.DRG_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.DRG_BIS_SETTINGS);
    case LANGUAGE_TEXTS.MNK_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.MNK_BIS_SETTINGS);
    case LANGUAGE_TEXTS.NIN_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.NIN_BIS_SETTINGS);
    case LANGUAGE_TEXTS.SAM_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.SAM_BIS_SETTINGS);
    case LANGUAGE_TEXTS.RPR_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.RPR_BIS_SETTINGS);
    case LANGUAGE_TEXTS.VPR_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.VPR_BIS_SETTINGS);
    case LANGUAGE_TEXTS.BRD_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.BRD_BIS_SETTINGS);
    case LANGUAGE_TEXTS.DNC_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.DNC_BIS_SETTINGS);
    case LANGUAGE_TEXTS.MCH_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.MCH_BIS_SETTINGS);
    case LANGUAGE_TEXTS.BLM_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.BLM_BIS_SETTINGS);
    case LANGUAGE_TEXTS.SMN_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.SMN_BIS_SETTINGS);
    case LANGUAGE_TEXTS.RDM_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.RDM_BIS_SETTINGS);
    case LANGUAGE_TEXTS.PCT_EN_NAME:
      return copyBisSettings(BIS_SETTINGS.PCT_BIS_SETTINGS);
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

export const loadBis = () => {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  const PLD_BIS_SETTINGS = {
    foodId: 652,
    itemSet: [43101, 43123, 43047, 43125, 43126, 43050, 43081, 43163, 43091, 43096, 43173, 43122],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],

      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }

  const WAR_BIS_SETTINGS = {
    foodId: 652,
    itemSet: [43103, 43123, 43047, 43125, 43126, 43050, 43081, 43163, 43091, 43096, 43173, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }


  const DRK_BIS_SETTINGS = {
    foodId: 652,
    itemSet: [43111, 43123, 43047, 43125, 43126, 43050, 43081, 43163, 43091, 43096, 43173, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }

  const GNB_BIS_SETTINGS = {
    foodId: 652,
    itemSet: [43116, 43123, 43047, 43125, 43126, 43050, 43081, 43163, 43091, 43096, 43173, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }


  const WHM_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43106, 43071, 43149, 42919, 43074, 43152, 43161, 43089, 43171, 43099, 43176, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 }
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 18, effectiveValue: 18 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 18, effectiveValue: 18 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      []
    ]
  }

  const SCH_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43109, 43071, 43149, 42919, 43074, 43152, 43161, 43089, 43171, 43099, 43176, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 }
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 18, effectiveValue: 18 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 18, effectiveValue: 18 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      []
    ]
  }


  const AST_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43113, 43071, 43149, 42919, 43074, 43152, 43161, 43089, 43171, 43099, 43176, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 }
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 18, effectiveValue: 18 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 18, effectiveValue: 18 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      []
    ]
  }


  const SGE_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43119, 43071, 43149, 42919, 43074, 43152, 43161, 43089, 43171, 43099, 43176, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 }
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 18, effectiveValue: 18 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 18, effectiveValue: 18 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      []
    ]
  }

  const DRG_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43104, 43051, 43129, 43053, 43054, 43132, 43159, 43087, 43169, 43097, 43174, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }


  const MNK_BIS_SETTINGS = {
    foodId: 654,
    itemSet: [43102, 43056, 43057, 43058, 43136, 43137, 43082, 43087, 43169, 43174, 43097, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.SKS_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.SKS_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.SKS_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }


  const NIN_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43110, 43143, 43144, 43068, 43069, 43147, 43083, 43165, 43093, 43098, 43175, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],

      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }


  const SAM_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43114, 43056, 43057, 43058, 43136, 43137, 43159, 43087, 43169, 43174, 43097, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.SKS_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.SKS_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],

      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }


  const RPR_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43118, 43051, 43129, 43053, 43054, 43132, 43159, 43087, 43169, 43097, 43174, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.SKS_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }



  const VPR_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43120, 43143, 43144, 43068, 43069, 43147, 43083, 43165, 43093, 43098, 43175, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.SKS_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.SKS_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }

  const BRD_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43105, 43138, 43062, 43063, 43141, 43142, 43083, 43165, 43093, 43098, 43175, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],

      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }

  const DNC_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43117, 43138, 43062, 43063, 43141, 43142, 43083, 43165, 43093, 43098, 43175, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],

      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }


  const MCH_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43112, 43138, 43062, 43063, 43141, 43142, 43083, 43165, 43093, 43098, 43175, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],

      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }

  const BLM_BIS_SETTINGS = {
    foodId: 659,
    itemSet: [43107, 43076, 43154, 43155, 43079, 43080, 43162, 43090, 43172, 43100, 43177, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.SPS_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.SPS_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],

      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }

  const SMN_BIS_SETTINGS = {
    foodId: 652,
    itemSet: [43108, 43076, 43154, 43078, 43079, 43157, 43162, 43090, 43172, 43100, 43177, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.SPS_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.SPS_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }


  const RDM_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43115, 43076, 43154, 43078, 43079, 43157, 43162, 43090, 43172, 43100, 43177, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }


  const PCT_BIS_SETTINGS = {
    foodId: 655,
    itemSet: [43121, 43076, 43154, 43078, 43079, 43157, 43162, 43090, 43172, 43100, 43177, -1],
    gearSetMaterias: [
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.DH_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [
        { statName: LANGUAGE_TEXTS.CRIT_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
        { statName: LANGUAGE_TEXTS.DET_STAT_EN_NAME, maxValue: 54, effectiveValue: 54 },
      ],
      [

      ]
    ]
  }


  return {
    PLD_BIS_SETTINGS,
    WAR_BIS_SETTINGS,
    DRK_BIS_SETTINGS,
    GNB_BIS_SETTINGS,
    WHM_BIS_SETTINGS,
    SCH_BIS_SETTINGS,
    AST_BIS_SETTINGS,
    SGE_BIS_SETTINGS,
    MNK_BIS_SETTINGS,
    DRG_BIS_SETTINGS,
    NIN_BIS_SETTINGS,
    SAM_BIS_SETTINGS,
    RPR_BIS_SETTINGS,
    VPR_BIS_SETTINGS,
    BRD_BIS_SETTINGS,
    DNC_BIS_SETTINGS,
    MCH_BIS_SETTINGS,
    SMN_BIS_SETTINGS,
    BLM_BIS_SETTINGS,
    RDM_BIS_SETTINGS,
    PCT_BIS_SETTINGS,
  }
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
