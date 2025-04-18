import { ItemSet } from "./ffxivdatabase/ItemSet";
import { GearSetMaterias } from "./ffxivdatabase/Materia";
import { PlayerPower } from "./ffxivdatabase/PlayerPower";

export interface EquipmentInput {
  equipmentDatas: Array<SingleEquipmentInputSaveState>;
}

export const USE_POT_VAL = 1;
export const NO_POT_VAL = 0;

export interface SingleEquipmentInputSaveState {
  mainPlayerJobAbbrev: string;
  race: string;
  foodId: number;
  mainPlayerPartner1Id: number | null;
  mainPlayerPartner2Id: number | null;
  itemSet: ItemSet;
  gearSetMaterias: GearSetMaterias;
  combatTimeMillisecond: number;
  partyMemberJobAbbrevs: string[];
  partyMemberIds: number[];
  partyMemberIlvl: number;
  usePot: number;
  power: PlayerPower;
  compositionBuffPercent: number;
}

export interface SingleEquipmentSetLoadoutData {
  equipmentData: SingleEquipmentInputSaveState;
  loadoutName: string;
}
