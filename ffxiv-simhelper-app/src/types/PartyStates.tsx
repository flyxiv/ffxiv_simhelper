import { PlayerPower } from "./ffxivdatabase/PlayerPower";

export const EMPTY_PARTY_MEMBER = "Empty";
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
  jobAbbrev: string;
  power: PlayerPower;
}
