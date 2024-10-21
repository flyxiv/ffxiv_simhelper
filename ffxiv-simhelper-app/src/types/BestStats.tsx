import { PlayerPower } from "./ffxivdatabase/PlayerPower";

// Frontend app sends best stats request for each stat so that it can give updates to the progress bar in 
// reasonable time periods. Then appends the result of each request into this response table
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
