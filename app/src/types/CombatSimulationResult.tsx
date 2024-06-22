export interface SimulationSummary {
  rdps: number;
  adps: number;
  pdps: number;
  edps: number;
}
export interface PartyContribution {
  skillId: number;
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
  job: String;
  role: String;
  simulationSummary: SimulationSummary;
  partyContributionTable: Array<PartyContribution>;
  damageProfileTable: Array<DamageProfile>;
  rotationLog: Array<SkillLog>;
}
