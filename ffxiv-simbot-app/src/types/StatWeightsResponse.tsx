import { PlayerPower } from "./ffxivdatabase/PlayerPower";

export interface StatWeightsResponseTable {
  combatTimeMillisecond: number;
  mainPlayerPower: PlayerPower;
  mainPlayerJobAbbrev: string;
  statAugmentedSimulationData: Array<StatWeightsResponse>;
  partyMemberJobAbbrevs: Array<string>;
}

export interface StatWeightsResponse {
  statName: string;
  augmentAmount: number;
  dps: number;
}
