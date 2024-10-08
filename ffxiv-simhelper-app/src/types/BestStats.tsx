import { PlayerPower } from "./ffxivdatabase/PlayerPower";

export interface BestStatsResponseTable {
  combatTimeMillisecond: number;
  mainPlayerPower: PlayerPower;
  mainPlayerJobAbbrev: string;
  statAugmentedSimulationData: Array<BestStatsResponse>;
  partyMemberJobAbbrevs: Array<string>;
}

export interface BestStatsResponse {
  statName: string;
  augmentAmount: number;
  dps: number;
}
