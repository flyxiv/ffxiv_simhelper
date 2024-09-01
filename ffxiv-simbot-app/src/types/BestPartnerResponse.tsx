import { PlayerPower } from "./ffxivdatabase/PlayerPower";

export interface BestPartnerResponseTable {
  combatTimeMillisecond: number;
  mainPlayerPower: PlayerPower;
  mainPlayerJobAbbrev: string;
  partnerSimulationData: Array<BestPartnerResponse>;
}

export interface BestPartnerResponse {
  partnerJobAbbrev: string;
  contributedDps: number;
}
