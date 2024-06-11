export interface QuickSimPartyState {
  combatTime: number;
  combatTimeSetter: Function;

  partyMemberJobs: string[];
}

export interface QuickSimPartyMemberState {
  playerId: number;
  jobName: string;
  jobNameSetter: Function;
}
