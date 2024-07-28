import { CharacterStats } from "./CharacterStates";

export interface PartyState {
  combatTime: number;
  combatTimeSetter: Function;

  partyMemberJobs: string[];
}

export interface PartyMemberState {
  playerId: number;
  jobName: string;
  jobNameSetter: Function;
}
export interface PartyInfo {
  playerId: number;
  partner1Id: number | null;
  partner2Id: number | null;
  job: string;
  stats: CharacterStats;
}
