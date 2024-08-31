import { PartyInfo } from "./PartyStates";

export interface SimulationApiRequest {
  mainPlayerId: number;
  combatTimeMillisecond: number;
  party: Array<PartyInfo>;
}
