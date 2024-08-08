import { read } from "fs";

let totalEquipments: Array<Equipment> = require("src/resources/equipment_data.json");
const leftSlots = ["head", "body", "hands", "legs", "feet"];
const rightSlots = ["wrist", "ears", "neck", "finger"];

const AIMING_CATEGORY_NAME = "Aiming";
const CASTING_CATEGORY_NAME = "Casting";
const FENDING_CATEGORY_NAME = "Fending";
const HEALING_CATEGORY_NAME = "Healing";
const MAIMING_CATEGORY_NAME = "Maiming";
const SCOUTING_CATEGORY_NAME = "Scouting";
const STRIKING_CATEGORY_NAME = "Striking";
const SLAYING_CATEGORY_NAME = "Slaying";

const CURRENT_MIN_ITEM_LEVEL = 710;

export const EQUIPMENT_DATABASE = readEquipmentData(CURRENT_MIN_ITEM_LEVEL);

export interface Equipment {
  id: number;
  name: string;
  jobName: Array<string>;
  itemLevel: number;
  slotName: string;
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
  weaponDamage: number;
}

export const EMPTY_EQUIPMENT_ID = -1;

export interface EquipmentKey {
  slotName: string;
  jobAbbrev: string;
}

export function toEquipmentKeyString(key: EquipmentKey): string {
  return `${key.slotName}-${key.jobAbbrev}`;
}

export function readEquipmentData(minItemLevel: number) {
  let equipmentDataFiltered = totalEquipments.filter(
    (equipment: Equipment) => equipment.itemLevel >= minItemLevel
  );

  let equipmentDatabase: Map<string, Equipment[]> = new Map();

  for (let i = 0; i < equipmentDataFiltered.length; i++) {
    let equipment = equipmentDataFiltered[i];
    let key = { slotName: equipment.slotName, jobAbbrev: "" };
    for (let j = 0; j < equipment.jobName.length; j++) {
      key.jobAbbrev = equipment.jobName[j];
      const equipments = equipmentDatabase.get(toEquipmentKeyString(key));
      if (equipments === undefined) {
        equipmentDatabase.set(toEquipmentKeyString(key), [equipment]);
      } else {
        equipments.push(equipment);
        equipmentDatabase.set(toEquipmentKeyString(key), equipments);
      }
    }
  }

  return equipmentDatabase;
}

export function accessEquipmentData(
  equipmentKey: EquipmentKey,
  database: Map<string, Equipment>
) {
  return database.get(toEquipmentKeyString(equipmentKey));
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

export function getEquipmentIconDirectory(
  equipmentSlot: string,
  jobAbbrev: string,
  equipmentName: string
) {
  let base_directory = process.env.PUBLIC_URL + "/images/equipment";
  let equipmentIconName = equipmentName.toLowerCase().replace(/ /g, "_");

  if (equipmentSlot === "mainhand" || equipmentSlot === "offhand") {
    return `${base_directory}/${equipmentSlot}/${jobAbbrev}/${equipmentIconName}.png`;
  }
  let category = getEquipmentCategory(equipmentSlot, jobAbbrev);

  return `${base_directory}/${category}/${equipmentSlot}/${equipmentIconName}.png`;
}

function getEquipmentCategory(equipmentSlot: string, jobAbbrev: string) {
  if (leftSlots.includes(equipmentSlot)) {
    switch (jobAbbrev) {
      case "PLD":
      case "WAR":
      case "DRK":
      case "GNB":
        return FENDING_CATEGORY_NAME;
      case "WHM":
      case "AST":
      case "SCH":
      case "SGE":
        return HEALING_CATEGORY_NAME;
      case "DRG":
      case "RPR":
        return MAIMING_CATEGORY_NAME;
      case "MNK":
      case "SAM":
        return STRIKING_CATEGORY_NAME;
      case "NIN":
      case "VPR":
        return SCOUTING_CATEGORY_NAME;
      case "BRD":
      case "MCH":
      case "DNC":
        return AIMING_CATEGORY_NAME;
      case "BLM":
      case "SMN":
      case "RDM":
      case "PCT":
        return CASTING_CATEGORY_NAME;
    }
  } else {
    switch (jobAbbrev) {
      case "PLD":
      case "WAR":
      case "DRK":
      case "GNB":
        return FENDING_CATEGORY_NAME;
      case "WHM":
      case "AST":
      case "SCH":
      case "SGE":
        return HEALING_CATEGORY_NAME;
      case "DRG":
      case "RPR":
      case "MNK":
      case "SAM":
        return SLAYING_CATEGORY_NAME;
      case "NIN":
      case "VPR":
        return SCOUTING_CATEGORY_NAME;
      case "BRD":
      case "MCH":
      case "DNC":
        return AIMING_CATEGORY_NAME;
      case "BLM":
      case "SMN":
      case "RDM":
      case "PCT":
        return CASTING_CATEGORY_NAME;
    }
  }
  return "Cannot find item category.";
}
