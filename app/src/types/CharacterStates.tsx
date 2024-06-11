import { CharacterStats } from "./QuickSimRequest";

export interface CharacterStates {
  jobName: string;
  jobNameSetter: Function;
  value: CharacterStats;
  setter: CharacterSetter;
}

export interface CharacterSetter {
  weaponAttack: Function;
  mainStat: Function;
  criticalStrike: Function;
  directHit: Function;
  determination: Function;
  speed: Function;
  tenacity: Function;
}
