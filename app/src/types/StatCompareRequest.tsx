import { CharacterStats } from "./CharacterStates";
import { PartyInfo } from "./PartyStates";

export interface StatCompareRequest {
  mainPlayerId: number;
  mainPlayerJob: string;
  combatTimeMillisecond: number;
  mainPlayerStat1: CharacterStats;
  mainPlayerStat2: CharacterStats;
  party: Array<PartyInfo>;
}
