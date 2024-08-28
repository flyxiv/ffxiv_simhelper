import { ItemSet } from "./ffxivdatabase/ItemSet";
import { GearSetMaterias } from "./ffxivdatabase/Materia";
import { PlayerPower } from "./ffxivdatabase/PlayerPower";

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
  power: PlayerPower
}


export interface SingleEquipmentSetLoadoutData {
  equipmentData: SingleEquipmentInputSaveState,
  loadoutName: string,
}

export interface DoubleEquipmentSetLoadoutData {
  equipmentData1: SingleEquipmentInputSaveState,
  equipmentData2: SingleEquipmentInputSaveState,
  loadoutName: string
}
