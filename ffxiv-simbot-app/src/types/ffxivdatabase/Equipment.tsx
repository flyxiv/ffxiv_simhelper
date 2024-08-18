let totalEquipments: Array<Equipment> = require("../../assets/data/equipment_data.json")

export const WEAPONSLOTS = ["weapon", "offHand"];
export const LEFTSLOTS = ["head", "body", "hands", "legs", "feet"];
export const RIGHTSLOTS = ["wrists", "ears", "neck", "finger1", "finger2"];
export const TOTAL_SLOTS = WEAPONSLOTS.concat(LEFTSLOTS).concat(RIGHTSLOTS);

const CURRENT_MIN_ITEM_LEVEL = 710;

export const {
  idDatabase: EQUIPMENT_DATABASE_BY_ID,
  keyDatabase: EQUIPMENT_DATABASE_BY_KEYS,
} = readEquipmentData(CURRENT_MIN_ITEM_LEVEL);

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
  if (slotName === "finger1" || slotName === "finger2") {
    keySlotName = "finger";
  }
  return `${keySlotName}-${jobAbbrev}`;
}

export function readEquipmentData(minItemLevel: number) {
  let equipmentDataFiltered = totalEquipments.filter(
    (equipment: Equipment) => equipment.itemLevel >= minItemLevel
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
  if (equipment.weaponDamage > 0) {
    stats.push({ statName: "WD", value: equipment.weaponDamage });
  }
  if (equipment.STR > 0) {
    stats.push({
      statName: "STR",
      value: equipment.STR,
    });
  }
  if (equipment.DEX > 0) {
    stats.push({
      statName: "DEX",
      value: equipment.DEX,
    });
  }
  if (equipment.INT > 0) {
    stats.push({
      statName: "INT",
      value: equipment.INT,
    });
  }
  if (equipment.MND > 0) {
    stats.push({
      statName: "MND",
      value: equipment.MND,
    });
  }

  if (equipment.criticalStrike > 0) {
    stats.push({ statName: "CRT", value: equipment.criticalStrike });
  }
  if (equipment.directHit > 0) {
    stats.push({ statName: "DH", value: equipment.directHit });
  }
  if (equipment.determination > 0) {
    stats.push({ statName: "DET", value: equipment.determination });
  }
  if (equipment.skillSpeed > 0) {
    stats.push({ statName: "SKS", value: equipment.skillSpeed });
  }
  if (equipment.spellSpeed > 0) {
    stats.push({ statName: "SPS", value: equipment.spellSpeed });
  }
  if (equipment.tenacity > 0) {
    stats.push({ statName: "TEN", value: equipment.tenacity });
  }
  if (equipment.piety > 0) {
    stats.push({ statName: "PIE", value: equipment.piety });
  }

  let descriptionString = "";
  stats.forEach((stat, _) => {
    descriptionString += `${stat.statName} +${stat.value} `;
  });

  return descriptionString.trim();
}

export function getEquipmentSlotsOfJob(jobAbbrev: string) {
  if (jobAbbrev === "PLD") {
    return [
      "weapon",
      "offHand",
      "head",
      "ears",
      "body",
      "neck",
      "hands",
      "wrists",
      "legs",
      "finger1",
      "feet",
      "finger2",
    ];
  }

  return [
    "weapon",
    "ears",
    "head",
    "neck",
    "body",
    "wrists",
    "hands",
    "finger1",
    "legs",
    "finger2",
    "feet",
  ];
}
