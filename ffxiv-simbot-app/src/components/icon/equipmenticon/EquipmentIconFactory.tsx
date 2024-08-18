import { IMAGES_DIRECTORY } from "../../../const/BaseDirectory";
import { LEFTSLOTS, WEAPONSLOTS } from "../../../types/ffxivdatabase/Equipment";

const AIMING_CATEGORY_NAME = "Aiming";
const CASTING_CATEGORY_NAME = "Casting";
const FENDING_CATEGORY_NAME = "Fending";
const HEALING_CATEGORY_NAME = "Healing";
const MAIMING_CATEGORY_NAME = "Maiming";
const SCOUTING_CATEGORY_NAME = "Scouting";
const STRIKING_CATEGORY_NAME = "Striking";
const SLAYING_CATEGORY_NAME = "Slaying";

export function getEquipmentIconDirectory(
  slotName: string,
  jobAbbrev: string,
  equipmentName: string
) {
  let base_directory = `${IMAGES_DIRECTORY}/equipment`;
  let equipmentIconName = equipmentName.toLowerCase().replace(/ /g, "_");

  if (WEAPONSLOTS.includes(slotName)) {
    return `${base_directory}/${slotName}/${jobAbbrev}/${equipmentIconName}.png`;
  }
  let category = getEquipmentCategory(slotName, jobAbbrev);

  return `${base_directory}/${slotName}/${category}/${equipmentIconName}.png`;
}

function getEquipmentCategory(equipmentSlot: string, jobAbbrev: string) {
  if (LEFTSLOTS.includes(equipmentSlot)) {
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
