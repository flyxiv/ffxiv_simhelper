import { CharacterStats } from "./CharacterStates";
import { PartyInfo } from "./PartyStates";

export interface StatCompareRequest {
  mainPlayerId: number;
  mainPlayerJob: string;
  mainPlayerPartner1Id: number | null;
  mainPlayerPartner2Id: number | null;
  combatTimeMillisecond: number;
  mainPlayerStat1: CharacterStats;
  mainPlayerStat2: CharacterStats;
  party: Array<PartyInfo>;
}
