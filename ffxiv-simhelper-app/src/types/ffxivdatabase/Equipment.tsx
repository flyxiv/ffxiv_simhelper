import {
  AppLanguageTexts,
  BODY_SLOT_EN_TEXT,
  EARS_SLOT_EN_TEXT,
  FEET_SLOT_EN_TEXT,
  FINGER1_SLOT_EN_TEXT,
  FINGER2_SLOT_EN_TEXT,
  FINGER_SLOT_EN_TEXT,
  HANDS_SLOT_EN_TEXT,
  HEAD_SLOT_EN_TEXT,
  LEGS_SLOT_EN_TEXT,
  NECK_SLOT_EN_TEXT,
  OFFHAND_SLOT_EN_TEXT,
  PLD_EN_NAME,
  WEAPON_SLOT_EN_TEXT,
  WRIST_SLOT_EN_TEXT,
} from "../../const/languageTexts";
import totalEquipmentsJson from "../../assets/data/equipment_data.json";
import { LanguageMode } from "../../LanguageContext";

export const WEAPONSLOTS = [WEAPON_SLOT_EN_TEXT, OFFHAND_SLOT_EN_TEXT];
export const LEFTSLOTS = [
  HEAD_SLOT_EN_TEXT,
  BODY_SLOT_EN_TEXT,
  HANDS_SLOT_EN_TEXT,
  LEGS_SLOT_EN_TEXT,
  FEET_SLOT_EN_TEXT,
];
export const RIGHTSLOTS = [
  WRIST_SLOT_EN_TEXT,
  EARS_SLOT_EN_TEXT,
  NECK_SLOT_EN_TEXT,
  FINGER1_SLOT_EN_TEXT,
  FINGER2_SLOT_EN_TEXT,
];
export const TOTAL_SLOTS = WEAPONSLOTS.concat(LEFTSLOTS).concat(RIGHTSLOTS);

let totalEquipments: Array<Equipment> = [];

totalEquipmentsJson.forEach((element) => {
  totalEquipments.push({
    id: element.id,
    name: element.name,
    jobName: element.jobName,
    itemLevel: element.itemLevel,
    slotName: element.slotName,
    weaponDamage: element.weaponDamage,
    STR: element.STR,
    DEX: element.DEX,
    INT: element.INT,
    MND: element.MND,
    criticalStrike: element.criticalStrike,
    directHit: element.directHit,
    determination: element.determination,
    skillSpeed: element.skillSpeed,
    spellSpeed: element.spellSpeed,
    tenacity: element.tenacity,
    piety: element.piety,
    maxSubstat: 0,
    materiaSlotCount: element.materiaSlotCount,
    pentameldable: element.pentameldable,
  });
});

const CURRENT_MIN_ITEM_LEVEL = 710;
const CURRENT_MAX_ITEM_LEVEL = 735;

export const {
  idDatabase: EQUIPMENT_DATABASE_BY_ID,
  keyDatabase: EQUIPMENT_DATABASE_BY_KEYS,
} = readEquipmentData(CURRENT_MIN_ITEM_LEVEL, CURRENT_MAX_ITEM_LEVEL);


export interface Equipment {
  id: number;
  name: string;
  jobName: Array<string>;
  itemLevel: number;
  slotName: string;

  weaponDamage: number;
  STR: number;
  DEX: number;
  INT: number;
  MND: number;
  criticalStrike: number;
  directHit: number;
  determination: number;
  skillSpeed: number;
  spellSpeed: number;
  tenacity: number;
  piety: number;
  maxSubstat: number;

  materiaSlotCount: number;
  pentameldable: boolean;
}

export const EMPTY_EQUIPMENT_ID = -1;

export interface EquipmentKey {
  slotName: string;
  jobAbbrev: string;
}

export function toEquipmentKeyString(
  jobAbbrev: string,
  slotName: string
): string {
  let keySlotName = slotName;
  if (slotName === FINGER1_SLOT_EN_TEXT || slotName === FINGER2_SLOT_EN_TEXT) {
    keySlotName = FINGER_SLOT_EN_TEXT;
  }
  return `${keySlotName}-${jobAbbrev}`;
}

export function readEquipmentData(minItemLevel: number, maxItemLevel: number) {
  let equipmentDataFiltered = totalEquipments.filter(
    (equipment) =>
      equipment.itemLevel >= minItemLevel && equipment.itemLevel <= maxItemLevel
  );

  let equipmentDatabaseById: Map<number, Equipment> = new Map();
  let equipmentDatabaseByKey: Map<string, Equipment[]> = new Map();

  for (let i = 0; i < equipmentDataFiltered.length; i++) {
    let equipment = equipmentDataFiltered[i];
    equipment.maxSubstat = Math.max(
      equipment.criticalStrike,
      equipment.directHit,
      equipment.determination,
      equipment.skillSpeed,
      equipment.spellSpeed,
      equipment.tenacity,
      equipment.piety
    );
    equipmentDatabaseById.set(equipment.id, equipment);
    let key = { slotName: equipment.slotName, jobAbbrev: "" };
    for (let j = 0; j < equipment.jobName.length; j++) {
      key.jobAbbrev = equipment.jobName[j];
      const equipments = equipmentDatabaseByKey.get(
        toEquipmentKeyString(key.jobAbbrev, key.slotName)
      );
      if (equipments === undefined) {
        equipmentDatabaseByKey.set(
          toEquipmentKeyString(key.jobAbbrev, key.slotName),
          [equipment]
        );
      } else {
        equipments.push(equipment);
        equipmentDatabaseByKey.set(
          toEquipmentKeyString(key.jobAbbrev, key.slotName),
          equipments
        );
      }
    }
  }

  equipmentDatabaseByKey.forEach((equipments, _) => {
    equipments.sort((equipment1, equipment2) => {
      return equipment2.itemLevel - equipment1.itemLevel;
    });
  });

  return {
    idDatabase: equipmentDatabaseById,
    keyDatabase: equipmentDatabaseByKey,
  };
}

export function equipmentStatDescriptionString(equipment: Equipment) {
  const stats = [];
  let LANGUAGE_TEXTS = AppLanguageTexts();

  if (equipment.weaponDamage > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.WD_STAT_NAME, value: equipment.weaponDamage });
  }
  if (equipment.STR > 0) {
    stats.push({
      statName: LANGUAGE_TEXTS.STR_STAT_NAME,
      value: equipment.STR,
    });
  }
  if (equipment.DEX > 0) {
    stats.push({
      statName: LANGUAGE_TEXTS.DEX_STAT_NAME,
      value: equipment.DEX,
    });
  }
  if (equipment.INT > 0) {
    stats.push({
      statName: LANGUAGE_TEXTS.INT_STAT_NAME,
      value: equipment.INT,
    });
  }
  if (equipment.MND > 0) {
    stats.push({
      statName: LANGUAGE_TEXTS.MIND_STAT_NAME,
      value: equipment.MND,
    });
  }

  if (equipment.criticalStrike > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.CRIT_STAT_NAME, value: equipment.criticalStrike });
  }
  if (equipment.directHit > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.DH_STAT_NAME, value: equipment.directHit });
  }
  if (equipment.determination > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.DET_STAT_NAME, value: equipment.determination });
  }
  if (equipment.skillSpeed > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.SKS_STAT_NAME, value: equipment.skillSpeed });
  }
  if (equipment.spellSpeed > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.SPS_STAT_NAME, value: equipment.spellSpeed });
  }
  if (equipment.tenacity > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.TEN_STAT_NAME, value: equipment.tenacity });
  }
  if (equipment.piety > 0) {
    stats.push({ statName: LANGUAGE_TEXTS.PIE_STAT_NAME, value: equipment.piety });
  }

  let descriptionString = "";
  stats.forEach((stat, _) => {
    descriptionString += `${stat.statName} +${stat.value} `;
  });

  return descriptionString.trim();
}

export function getEquipmentSlotsOfJob(jobAbbrev: string) {
  if (jobAbbrev === PLD_EN_NAME) {
    return [
      WEAPON_SLOT_EN_TEXT,
      OFFHAND_SLOT_EN_TEXT,
      HEAD_SLOT_EN_TEXT,
      BODY_SLOT_EN_TEXT,
      HANDS_SLOT_EN_TEXT,
      LEGS_SLOT_EN_TEXT,
      FEET_SLOT_EN_TEXT,
      EARS_SLOT_EN_TEXT,
      NECK_SLOT_EN_TEXT,
      WRIST_SLOT_EN_TEXT,
      FINGER1_SLOT_EN_TEXT,
      FINGER2_SLOT_EN_TEXT,
    ];
  }

  return [
    WEAPON_SLOT_EN_TEXT,
    HEAD_SLOT_EN_TEXT,
    BODY_SLOT_EN_TEXT,
    HANDS_SLOT_EN_TEXT,
    LEGS_SLOT_EN_TEXT,
    FEET_SLOT_EN_TEXT,
    EARS_SLOT_EN_TEXT,
    NECK_SLOT_EN_TEXT,
    WRIST_SLOT_EN_TEXT,
    FINGER1_SLOT_EN_TEXT,
    FINGER2_SLOT_EN_TEXT,
  ];
}

export function getFirstSecondSubStat(equipment: Equipment) {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  let firstSubStat = LANGUAGE_TEXTS.CRIT_STAT_NAME;
  let secondSubStat = LANGUAGE_TEXTS.DH_STAT_NAME;
  if (equipment.criticalStrike > 0) {
    if (equipment.criticalStrike === equipment.maxSubstat) {
      firstSubStat = LANGUAGE_TEXTS.CRIT_STAT_NAME;
    } else {
      secondSubStat = LANGUAGE_TEXTS.CRIT_STAT_NAME;
    }
  }

  if (equipment.directHit > 0) {
    if (equipment.directHit === equipment.maxSubstat) {
      firstSubStat = LANGUAGE_TEXTS.DH_STAT_NAME;
    } else {
      secondSubStat = LANGUAGE_TEXTS.DH_STAT_NAME;
    }
  }

  if (equipment.determination > 0) {
    if (equipment.determination === equipment.maxSubstat) {
      firstSubStat = LANGUAGE_TEXTS.DET_STAT_NAME;
    } else {
      secondSubStat = LANGUAGE_TEXTS.DET_STAT_NAME;
    }
  }

  if (equipment.skillSpeed > 0) {
    if (equipment.skillSpeed === equipment.maxSubstat) {
      firstSubStat = LANGUAGE_TEXTS.SKS_STAT_NAME;
    } else {
      secondSubStat = LANGUAGE_TEXTS.SKS_STAT_NAME;
    }
  }

  if (equipment.spellSpeed > 0) {
    if (equipment.spellSpeed === equipment.maxSubstat) {
      firstSubStat = LANGUAGE_TEXTS.SPS_STAT_NAME;
    } else {
      secondSubStat = LANGUAGE_TEXTS.SPS_STAT_NAME;
    }
  }

  if (equipment.tenacity > 0) {
    if (equipment.tenacity === equipment.maxSubstat) {
      firstSubStat = LANGUAGE_TEXTS.TEN_STAT_NAME;
    } else {
      secondSubStat = LANGUAGE_TEXTS.TEN_STAT_NAME;
    }
  }

  if (equipment.piety > 0) {
    if (equipment.piety === equipment.maxSubstat) {
      firstSubStat = LANGUAGE_TEXTS.PIE_STAT_NAME;
    } else {
      secondSubStat = LANGUAGE_TEXTS.PIE_STAT_NAME;
    }
  }

  return [firstSubStat, secondSubStat];
}
