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
  job: string;
  stats: CharacterStats;
}
