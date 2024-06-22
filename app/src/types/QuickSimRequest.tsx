import { PartyInfo } from "./PartyStates";

export interface QuickSimRequest {
  mainPlayerId: number;
  combatTimeMillisecond: number;
  party: Array<PartyInfo>;
}
