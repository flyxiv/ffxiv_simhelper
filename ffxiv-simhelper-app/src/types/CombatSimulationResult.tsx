export interface SimulationSummary {
  rdps: Array<number>;
  pdps: Array<number>;
  edps: Array<number>;
  maxRdps: Array<number>;
}
export interface PartyContribution {
  skillId: number;
  partyMemberId: number;
  statusId: number;
  contributedRdps: number;
}

export interface PartyBurstContribution {
  skillId: number;
  minute: number;
  partyMemberId: number;
  statusId: number;
  contributedRdps: number;
}

export interface DamageProfile {
  id: number;
  entity: String;
  rdpsContribution: number;
  pdpsContribution: number;
  castCount: number;
}

export interface SkillLog {
  time: number;
  skillId: number;
  target: number | null;
  buffs: Array<number>;
  debuffs: Array<number>;
}
export interface SimulationData {
  playerId: number;
  jobAbbrev: string;
  role: string;
  simulationSummary: SimulationSummary;
  partyContributionTable: Array<PartyContribution>;
  partyBurstContributionTable: Array<PartyBurstContribution>;
  damageProfileTable: Array<DamageProfile>;
  rotationLog: Array<SkillLog>;
}
