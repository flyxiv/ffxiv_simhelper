export interface CharacterStates {
  jobName: string;
  jobNameSetter: Function;
  stats: CharacterStats;
  setStats: Function;
}

export interface CharacterSetter {
  weaponDamage: Function;
  mainStat: Function;
  criticalStrike: Function;
  directHit: Function;
  determination: Function;
  speed: Function;
  tenacity: Function;
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
