import { defaultPlayerPower } from "../types/ffxivdatabase/PlayerPower";
import { defaultItemSet } from "../types/ffxivdatabase/ItemSet";
import { GearSetMaterias } from "../types/ffxivdatabase/Materia";
import { SingleEquipmentInputSaveState } from "../types/SingleEquipmentInputSaveState";

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

export function defaultSingleEquipmentInput(): SingleEquipmentInputSaveState {
  let partyMemberJobs = ["NIN", "WAR", "WHM", "SGE", "DRG", "BRD", "BLM"];
  let defaultMaterias = DEFAULT_GEARSET_MATERIAS;

  return {
    mainPlayerJobAbbrev: "PLD",
    race: "Midlander Hyur",
    mainPlayerPartner1Id: null,
    mainPlayerPartner2Id: null,
    itemSet: defaultItemSet(),
    gearSetMaterias: defaultMaterias,
    combatTimeMillisecond: 150000,
    partyMemberJobAbbrevs: partyMemberJobs,
    partyMemberIds: [1, 2, 3, 4, 5, 6, 7],
    foodId: -1,
    power: defaultPlayerPower()
  };
}
