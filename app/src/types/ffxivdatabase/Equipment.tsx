let totalEquipments: Array<Equipment> = require("src/resources/equipment_data.json");

export const WEAPONSLOTS = ["mainhand", "offhand"];
export const LEFTSLOTS = ["head", "body", "hands", "legs", "feet"];
export const RIGHTSLOTS = ["wrist", "ears", "neck", "finger"];
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
  mainStat: number;
  criticalStrike: number;
  directHit: number;
  determination: number;
  skillSpeed: number;
  spellSpeed: number;
  tenacity: number;
  piety: number;

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
  return `${slotName}-${jobAbbrev}`;
}

export function readEquipmentData(minItemLevel: number) {
  let equipmentDataFiltered = totalEquipments.filter(
    (equipment: Equipment) => equipment.itemLevel >= minItemLevel
  );

  let equipmentDatabaseById: Map<number, Equipment> = new Map();
  let equipmentDatabaseByKey: Map<string, Equipment[]> = new Map();

  for (let i = 0; i < equipmentDataFiltered.length; i++) {
    let equipment = equipmentDataFiltered[i];
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
  if (equipment.mainStat > 0) {
    stats.push({ statName: "Main", value: equipment.mainStat });
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

export function getLeftEquipmentSlotsOfJob(jobAbbrev: string) {
  let weaponSlots = WEAPONSLOTS;
  if (jobAbbrev !== "PLD") {
    weaponSlots = [weaponSlots[0]];
  }

  return weaponSlots.concat(LEFTSLOTS);
}
