export interface CharacterStates {
  jobName: string;
  jobNameSetter: Function;
  partner1Id: number | null;
  setPartner1Id: Function;
  partner2Id: number | null;
  setPartner2Id: Function;
  stats: CharacterStats;
  setStats: Function;
}

export interface CharacterStats {
  weaponDamage: number;
  mainStat: number;
  criticalStrike: number;
  directHit: number;
  determination: number;
  speed: number;
  tenacity: number;
}
