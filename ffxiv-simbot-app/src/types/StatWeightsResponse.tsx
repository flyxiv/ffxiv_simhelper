import { PlayerPower } from "./ffxivdatabase/PlayerPower";

export interface StatWeightsResponseTable {
  combatTimeMillisecond: number;
  mainPlayerPower: PlayerPower;
  mainPlayerJobAbbrev: string;
  statAugmentedSimulationData: Array<StatWeightsResponse>;
}

export interface StatWeightsResponse {
  statName: string;
  augmentAmount: number;
  dps: number;
}
