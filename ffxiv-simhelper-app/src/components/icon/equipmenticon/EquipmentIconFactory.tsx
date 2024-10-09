import { IMAGES_DIRECTORY } from "../../../const/BaseDirectory";
import { AppLanguageTexts, AST_EN_NAME, BLM_EN_NAME, BRD_EN_NAME, DNC_EN_NAME, DRG_EN_NAME, DRK_EN_NAME, GNB_EN_NAME, MCH_EN_NAME, MNK_EN_NAME, NIN_EN_NAME, PCT_EN_NAME, PLD_EN_NAME, RDM_EN_NAME, RPR_EN_NAME, SAM_EN_NAME, SCH_EN_NAME, SGE_EN_NAME, SMN_EN_NAME, VPR_EN_NAME, WAR_EN_NAME, WHM_EN_NAME } from "../../../const/languageTexts";
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
      case PLD_EN_NAME:
      case WAR_EN_NAME:
      case DRK_EN_NAME:
      case GNB_EN_NAME:
        return FENDING_CATEGORY_NAME;
      case WHM_EN_NAME:
      case AST_EN_NAME:
      case SCH_EN_NAME:
      case SGE_EN_NAME:
        return HEALING_CATEGORY_NAME;
      case DRG_EN_NAME:
      case RPR_EN_NAME:
        return MAIMING_CATEGORY_NAME;
      case MNK_EN_NAME:
      case SAM_EN_NAME:
        return STRIKING_CATEGORY_NAME;
      case NIN_EN_NAME:
      case VPR_EN_NAME:
        return SCOUTING_CATEGORY_NAME;
      case BRD_EN_NAME:
      case MCH_EN_NAME:
      case DNC_EN_NAME:
        return AIMING_CATEGORY_NAME;
      case BLM_EN_NAME:
      case SMN_EN_NAME:
      case RDM_EN_NAME:
      case PCT_EN_NAME:
        return CASTING_CATEGORY_NAME;
    }
  } else {
    switch (jobAbbrev) {
      case PLD_EN_NAME:
      case WAR_EN_NAME:
      case DRK_EN_NAME:
      case GNB_EN_NAME:
        return FENDING_CATEGORY_NAME;
      case WHM_EN_NAME:
      case AST_EN_NAME:
      case SCH_EN_NAME:
      case SGE_EN_NAME:
        return HEALING_CATEGORY_NAME;
      case DRG_EN_NAME:
      case RPR_EN_NAME:
      case MNK_EN_NAME:
      case SAM_EN_NAME:
        return SLAYING_CATEGORY_NAME;
      case NIN_EN_NAME:
      case VPR_EN_NAME:
      case BRD_EN_NAME:
      case MCH_EN_NAME:
      case DNC_EN_NAME:
        return AIMING_CATEGORY_NAME;
      case BLM_EN_NAME:
      case SMN_EN_NAME:
      case RDM_EN_NAME:
      case PCT_EN_NAME:
        return CASTING_CATEGORY_NAME;
    }
  }
  return "Cannot find item category.";
}
