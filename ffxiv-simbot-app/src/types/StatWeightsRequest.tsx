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
<<<<<<< HEAD
>>>>>>> 15b0829d (added job modified gcds)
=======
>>>>>>> a1871f3e (configured for electron version)
