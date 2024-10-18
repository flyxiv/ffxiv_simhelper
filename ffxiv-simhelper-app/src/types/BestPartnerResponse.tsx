import { PlayerPower } from "./ffxivdatabase/PlayerPower";

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
