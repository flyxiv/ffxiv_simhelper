import { PlayerPower } from "./ffxivdatabase/PlayerPower";

// Frontend app sends best partner request for each job so that it can give updates to the progress bar in 
// reasonable time periods. Then appends the result of each request into this response table
export interface BestPartnerResponseTable {
  combatTimeMillisecond: number;
  mainPlayerPower: PlayerPower;
  mainPlayerJobAbbrev: string;
  partnerSimulationData: Array<BestPartnerResponse>;
}

export interface BestPartnerResponse {
  partnerJobAbbrev: string;
  contributedDamage: Array<number>;
}

export interface BestPartnerSingleBurst {
  partnerJobAbbrev: string;
  contributedDamage: number;
  minute: number;
}
