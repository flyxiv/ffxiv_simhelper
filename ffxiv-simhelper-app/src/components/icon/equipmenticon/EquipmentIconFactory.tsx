import { IMAGES_DIRECTORY } from "../../../const/BaseDirectory";
import { LANGUAGE_TEXTS } from "../../../const/languageTexts";
import { LEFTSLOTS, WEAPONSLOTS } from "../../../types/ffxivdatabase/Equipment";

const AIMING_CATEGORY_NAME = "Aiming";
const CASTING_CATEGORY_NAME = "Casting";
const FENDING_CATEGORY_NAME = "Fending";
const HEALING_CATEGORY_NAME = "Healing";
const MAIMING_CATEGORY_NAME = "Maiming";
const SCOUTING_CATEGORY_NAME = "Scouting";
const STRIKING_CATEGORY_NAME = "Striking";
const SLAYING_CATEGORY_NAME = "Slaying";

const ALL_CATEGORIES = [AIMING_CATEGORY_NAME.toLowerCase(), CASTING_CATEGORY_NAME.toLowerCase(), FENDING_CATEGORY_NAME.toLowerCase(), HEALING_CATEGORY_NAME.toLowerCase(), MAIMING_CATEGORY_NAME.toLowerCase(), SCOUTING_CATEGORY_NAME.toLowerCase(), STRIKING_CATEGORY_NAME.toLowerCase(), SLAYING_CATEGORY_NAME.toLowerCase()];

export function getEquipmentIconDirectory(
  slotName: string,
  jobAbbrev: string,
  equipmentName: string
) {
  let base_directory = `${IMAGES_DIRECTORY}/equipment`;
  let equipmentIconName = equipmentName.toLowerCase().replace(/ /g, "_");

  let isJobSpecificEquipment = ALL_CATEGORIES.filter((category) => equipmentIconName.includes(category)).length === 0;

  if (WEAPONSLOTS.includes(slotName)) {
    return `${base_directory}/${slotName}/${jobAbbrev}/${equipmentIconName}.png`;
  }

  if (isJobSpecificEquipment) {
    return `${base_directory}/${slotName}/JobSpecific/${jobAbbrev}/${equipmentIconName}.png`;
  }
  let category = getEquipmentCategory(slotName, jobAbbrev);

  return `${base_directory}/${slotName}/${category}/${equipmentIconName}.png`;
}

function getEquipmentCategory(equipmentSlot: string, jobAbbrev: string) {
  if (LEFTSLOTS.includes(equipmentSlot)) {
    switch (jobAbbrev) {
      case LANGUAGE_TEXTS.PLD_EN_NAME:
      case LANGUAGE_TEXTS.WAR_EN_NAME:
      case LANGUAGE_TEXTS.DRK_EN_NAME:
      case LANGUAGE_TEXTS.GNB_EN_NAME:
        return FENDING_CATEGORY_NAME;
      case LANGUAGE_TEXTS.WHM_EN_NAME:
      case LANGUAGE_TEXTS.AST_EN_NAME:
      case LANGUAGE_TEXTS.SCH_EN_NAME:
      case LANGUAGE_TEXTS.SGE_EN_NAME:
        return HEALING_CATEGORY_NAME;
      case LANGUAGE_TEXTS.DRG_EN_NAME:
      case LANGUAGE_TEXTS.RPR_EN_NAME:
        return MAIMING_CATEGORY_NAME;
      case LANGUAGE_TEXTS.MNK_EN_NAME:
      case LANGUAGE_TEXTS.SAM_EN_NAME:
        return STRIKING_CATEGORY_NAME;
      case LANGUAGE_TEXTS.NIN_EN_NAME:
      case LANGUAGE_TEXTS.VPR_EN_NAME:
        return SCOUTING_CATEGORY_NAME;
      case LANGUAGE_TEXTS.BRD_EN_NAME:
      case LANGUAGE_TEXTS.MCH_EN_NAME:
      case LANGUAGE_TEXTS.DNC_EN_NAME:
        return AIMING_CATEGORY_NAME;
      case LANGUAGE_TEXTS.BLM_EN_NAME:
      case LANGUAGE_TEXTS.SMN_EN_NAME:
      case LANGUAGE_TEXTS.RDM_EN_NAME:
      case LANGUAGE_TEXTS.PCT_EN_NAME:
        return CASTING_CATEGORY_NAME;
    }
  } else {
    switch (jobAbbrev) {
      case LANGUAGE_TEXTS.PLD_EN_NAME:
      case LANGUAGE_TEXTS.WAR_EN_NAME:
      case LANGUAGE_TEXTS.DRK_EN_NAME:
      case LANGUAGE_TEXTS.GNB_EN_NAME:
        return FENDING_CATEGORY_NAME;
      case LANGUAGE_TEXTS.WHM_EN_NAME:
      case LANGUAGE_TEXTS.AST_EN_NAME:
      case LANGUAGE_TEXTS.SCH_EN_NAME:
      case LANGUAGE_TEXTS.SGE_EN_NAME:
        return HEALING_CATEGORY_NAME;
      case LANGUAGE_TEXTS.DRG_EN_NAME:
      case LANGUAGE_TEXTS.RPR_EN_NAME:
      case LANGUAGE_TEXTS.MNK_EN_NAME:
      case LANGUAGE_TEXTS.SAM_EN_NAME:
        return SLAYING_CATEGORY_NAME;
      case LANGUAGE_TEXTS.NIN_EN_NAME:
      case LANGUAGE_TEXTS.VPR_EN_NAME:
      case LANGUAGE_TEXTS.BRD_EN_NAME:
      case LANGUAGE_TEXTS.MCH_EN_NAME:
      case LANGUAGE_TEXTS.DNC_EN_NAME:
        return AIMING_CATEGORY_NAME;
      case LANGUAGE_TEXTS.BLM_EN_NAME:
      case LANGUAGE_TEXTS.SMN_EN_NAME:
      case LANGUAGE_TEXTS.RDM_EN_NAME:
      case LANGUAGE_TEXTS.PCT_EN_NAME:
        return CASTING_CATEGORY_NAME;
    }
  }
  return "Cannot find item category.";
}
