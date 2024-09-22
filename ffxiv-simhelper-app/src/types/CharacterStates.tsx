import { isCaster } from "./ffxivdatabase/ItemSet";
import { PlayerPower } from "./ffxivdatabase/PlayerPower";

export interface CharacterStates {
  jobAbbrev: string;
  jobNameSetter: Function;
  partner1Id: number | null;
  setPartner1Id: Function;
  partner2Id: number | null;
  setPartner2Id: Function;
  stats: CharacterStats;
  setStats: Function;
}

export interface EquipmentSimCharacterStates {
  jobAbbrev: string;
  jobNameSetter: Function;
  partner1Id: number | null;
  setPartner1Id: Function;
  partner2Id: number | null;
  setPartner2Id: Function;
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

export function powerToCharacterStats(
  power: PlayerPower,
  jobAbbrev: string
): CharacterStats {
  return {
    weaponDamage: power.weaponDamage,
    mainStat: power.mainStat,
    criticalStrike: power.criticalStrike,
    directHit: power.directHit,
    determination: power.determination,
    speed: isCaster(jobAbbrev) ? power.spellSpeed : power.skillSpeed,
    tenacity: power.tenacity,
  };
}
