import { defaultPlayerPower } from "../types/ffxivdatabase/PlayerPower";
import {
  calculatePlayerPowerFromInputs,
  defaultItemSet,
  WEAPON_SLOT_ID,
} from "../types/ffxivdatabase/ItemSet";
import { EMPTY_MATERIA, GearSetMaterias } from "../types/ffxivdatabase/Materia";
import {
  EquipmentInput,
  SingleEquipmentInputSaveState,
  USE_POT_VAL,
} from "../types/EquipmentInput";
import { LANGUAGE_TEXTS } from "./languageTexts";
import { PARTY_MAX_ILVL } from "../components/input/PartyMemberIlvlSelection";
import { mapJobAbbrevToJobBisEquipments } from "./StatValue";

export const DEFAULT_GEARSET_MATERIAS: GearSetMaterias = [
  [],
  [],
  [],
  [],
  [],
  [],
  [],
  [],
  [],
  [],
  [],
  [],
];

export function defaultSingleEquipmentInput(): EquipmentInput {
  let partyMemberJobs = [
    LANGUAGE_TEXTS.WAR_EN_NAME,
    LANGUAGE_TEXTS.WHM_EN_NAME,
    LANGUAGE_TEXTS.SGE_EN_NAME,
    LANGUAGE_TEXTS.DRG_EN_NAME,
    LANGUAGE_TEXTS.NIN_EN_NAME,
    LANGUAGE_TEXTS.BRD_EN_NAME,
    LANGUAGE_TEXTS.BLM_EN_NAME,
  ];
  let defaultMaterias = DEFAULT_GEARSET_MATERIAS;
  let [itemSet, weaponSlotmateriaCount] = defaultItemSet(LANGUAGE_TEXTS.PLD_EN_NAME);
  defaultMaterias[WEAPON_SLOT_ID] = Array(weaponSlotmateriaCount).fill(
    EMPTY_MATERIA
  );

  let mainPlayerJobAbbrev = LANGUAGE_TEXTS.PLD_EN_NAME;

  let bisGear = mapJobAbbrevToJobBisEquipments(mainPlayerJobAbbrev);

  let singleEquipmentInput: SingleEquipmentInputSaveState = {
    mainPlayerJobAbbrev: LANGUAGE_TEXTS.PLD_EN_NAME,
    race: LANGUAGE_TEXTS.MIDLANDER_HYUR_EN_NAME,
    mainPlayerPartner1Id: null,
    mainPlayerPartner2Id: null,
    itemSet: itemSet,
    gearSetMaterias: defaultMaterias,
    combatTimeMillisecond: 300000,
    partyMemberJobAbbrevs: partyMemberJobs,
    partyMemberIds: [1, 2, 3, 4, 5, 6, 7],
    partyMemberIlvl: PARTY_MAX_ILVL,
    foodId: -1,
    usePot: USE_POT_VAL,
    power: defaultPlayerPower(),
    compositionBuffPercent: 0,
  };

  if (bisGear !== undefined) {
    singleEquipmentInput.itemSet = bisGear.itemSet;
    singleEquipmentInput.gearSetMaterias = bisGear.gearSetMaterias;
    singleEquipmentInput.foodId = bisGear.foodId;
  }

  singleEquipmentInput.power =
    calculatePlayerPowerFromInputs(singleEquipmentInput);

  return {
    equipmentDatas: [singleEquipmentInput],
  };
}

export function defaultBestPartnerEquipmentInput(): EquipmentInput {
  let defaultMaterias = DEFAULT_GEARSET_MATERIAS;
  let [itemSet, weaponSlotmateriaCount] = defaultItemSet(LANGUAGE_TEXTS.SCH_EN_NAME);
  defaultMaterias[WEAPON_SLOT_ID] = Array(weaponSlotmateriaCount).fill(
    EMPTY_MATERIA
  );

  let singleEquipmentInput: SingleEquipmentInputSaveState = {
    mainPlayerJobAbbrev: LANGUAGE_TEXTS.SCH_EN_NAME,
    race: LANGUAGE_TEXTS.MIDLANDER_HYUR_EN_NAME,
    mainPlayerPartner1Id: null,
    mainPlayerPartner2Id: null,
    itemSet: itemSet,
    gearSetMaterias: defaultMaterias,
    combatTimeMillisecond: 300000,
    partyMemberJobAbbrevs: [],
    partyMemberIds: [],
    partyMemberIlvl: PARTY_MAX_ILVL,
    foodId: -1,
    usePot: USE_POT_VAL,
    power: defaultPlayerPower(),
    compositionBuffPercent: 0,
  };

  singleEquipmentInput.power =
    calculatePlayerPowerFromInputs(singleEquipmentInput);

  return {
    equipmentDatas: [singleEquipmentInput],
  };
}
