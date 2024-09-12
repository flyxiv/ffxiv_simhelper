<<<<<<< HEAD
import { PartyInfo } from "./PartyStates";

export interface StatWeightsRequest {
  mainPlayerId: number;
  combatTimeMillisecond: number;
  party: Array<PartyInfo>;
  statName: string;
  augmentAmount: number;
  partyIlvlAdjustment: number;
}
=======
import { PartyInfo } from "./PartyStates";

export interface StatWeightsRequest {
  mainPlayerId: number;
  combatTimeMillisecond: number;
  party: Array<PartyInfo>;
  statName: string;
  augmentAmount: number;
  partyIlvlAdjustment: number;
  usePot: boolean;
}
>>>>>>> 15b0829d (added job modified gcds)
