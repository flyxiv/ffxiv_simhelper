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
import { PARTY_MAX_ILVL } from "../components/input/PartyMemberIlvlSelection";
import { mapJobAbbrevToJobBisEquipments } from "./StatValue";
import { BLM_EN_NAME, BRD_EN_NAME, DRG_EN_NAME, MIDLANDER_HYUR_EN_NAME, NIN_EN_NAME, PLD_EN_NAME, SCH_EN_NAME, SGE_EN_NAME, WAR_EN_NAME, WHM_EN_NAME } from "./languageTexts";

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
    WAR_EN_NAME,
    WHM_EN_NAME,
    SGE_EN_NAME,
    DRG_EN_NAME,
    NIN_EN_NAME,
    BRD_EN_NAME,
    BLM_EN_NAME,
  ];
  let defaultMaterias = DEFAULT_GEARSET_MATERIAS;
  let [itemSet, weaponSlotmateriaCount] = defaultItemSet(PLD_EN_NAME);
  defaultMaterias[WEAPON_SLOT_ID] = Array(weaponSlotmateriaCount).fill(
    EMPTY_MATERIA
  );

  let mainPlayerJobAbbrev = PLD_EN_NAME;

  let bisGear = mapJobAbbrevToJobBisEquipments(mainPlayerJobAbbrev);

  let singleEquipmentInput: SingleEquipmentInputSaveState = {
    mainPlayerJobAbbrev: mainPlayerJobAbbrev,
    race: MIDLANDER_HYUR_EN_NAME,
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
  let [itemSet, weaponSlotmateriaCount] = defaultItemSet(SCH_EN_NAME);
  defaultMaterias[WEAPON_SLOT_ID] = Array(weaponSlotmateriaCount).fill(
    EMPTY_MATERIA
  );

  let singleEquipmentInput: SingleEquipmentInputSaveState = {
    mainPlayerJobAbbrev: SCH_EN_NAME,
    race: MIDLANDER_HYUR_EN_NAME,
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
