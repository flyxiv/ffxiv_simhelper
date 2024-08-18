import { defaultItemSet } from "../types/ffxivdatabase/ItemSet";
import { GearSetMaterias, Materia } from "../types/ffxivdatabase/Materia";
import { QuickSimInputSaveState } from "../types/QuickSimInput";

export function defaultQuickSimInput(): QuickSimInputSaveState {
  let partyMemberJobs = ["NIN", "WAR", "WHM", "SGE", "DRG", "BRD", "BLM"];
  let defaultMaterias: GearSetMaterias = [
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

  return {
    mainPlayerJob: "PLD",
    race: "Midlander Hyur",
    mainPlayerPartner1Id: null,
    mainPlayerPartner2Id: null,
    itemSet: defaultItemSet(),
    gearSetMaterias: defaultMaterias,
    combatTimeMillisecond: 150000,
    partyMemberJobAbbrevs: partyMemberJobs,
    partyMemberIds: [1, 2, 3, 4, 5, 6, 7],
    foodId: -1,
  };
}
