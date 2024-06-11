export interface CharacterStats {
  weaponDamage: number;
  mainStat: number;
  criticalStrike: number;
  directHit: number;
  determination: number;
  speed: number;
  tenacity: number;
}

export interface QuickSimRequest {
  mainPlayerId: number;
  combatTimeMillisecond: number;
  party: Array<PartyInfo>;
}

export interface PartyInfo {
  playerId: number;
  job: string;
  stats: CharacterStats;
}
