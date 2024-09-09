import { PartyInfo } from "./PartyStates";

export interface StatWeightsRequest {
  mainPlayerId: number;
  combatTimeMillisecond: number;
  party: Array<PartyInfo>;
  statName: string;
  augmentAmount: number;
  partyIlvlAdjustment: number;
}
