import { PlayerPower } from "./ffxivdatabase/PlayerPower";

export interface BestPartnerResponseTable {
  combatTimeMillisecond: number;
  mainPlayerPower: PlayerPower;
  mainPlayerJobAbbrev: string;
  partnerSimulationData: Array<BestPartnerResponse>;
}

export interface BestPartnerResponse {
  partnerJobAbbrev: string;
  contributedDps: Array<number>;
}

export interface BestPartnerSingleBurst {
  partnerJobAbbrev: string;
  contributedDps: number;
  minute: number;
}
