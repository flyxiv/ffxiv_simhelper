import { ItemSet } from "./ffxivdatabase/ItemSet";
import { GearSetMaterias } from "./ffxivdatabase/Materia";

export interface QuickSimInputSaveState {
  mainPlayerJob: string;
  race: string;
  foodId: number;
  mainPlayerPartner1Id: number | null;
  mainPlayerPartner2Id: number | null;
  itemSet: ItemSet;
  gearSetMaterias: GearSetMaterias;
  combatTimeMillisecond: number;
  partyMemberJobAbbrevs: string[];
  partyMemberIds: number[];
}
