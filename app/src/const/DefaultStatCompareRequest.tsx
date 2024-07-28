import { StatCompareRequest } from "src/types/StatCompareRequest";
import {
  DEFAULT_WEAPON_DAMAGE,
  DEFAULT_MAIN_STAT,
  DEFAULT_CRITICAL_STRIKE,
  DEFAULT_DIRECT_HIT,
  DEFAULT_DETERMINATION,
  DEFAULT_SPEED,
  DEFAULT_TENACITY,
  NIN_BIS_STATS,
} from "./StatValue";

export function defaultStatCompareRequest(): StatCompareRequest {
  let defaultJobsList = [
    "NIN",
    "PLD",
    "WAR",
    "WHM",
    "SGE",
    "DRG",
    "BRD",
    "BLM",
  ];
  let i = 1;

  let party = [];

  for (i = 1; i < defaultJobsList.length; i++) {
    party.push({
      playerId: i,
      job: defaultJobsList[i],
      role: "",
      partner1Id: null,
      partner2Id: null,
      stats: {
        weaponDamage: DEFAULT_WEAPON_DAMAGE,
        mainStat: DEFAULT_MAIN_STAT,
        criticalStrike: DEFAULT_CRITICAL_STRIKE,
        directHit: DEFAULT_DIRECT_HIT,
        determination: DEFAULT_DETERMINATION,
        speed: DEFAULT_SPEED,
        tenacity: DEFAULT_TENACITY,
      },
    });
  }
  return {
    mainPlayerId: 0,
    mainPlayerJob: defaultJobsList[0],
    mainPlayerPartner1Id: null,
    mainPlayerPartner2Id: null,
    combatTimeMillisecond: 120000,
    mainPlayerStat1: NIN_BIS_STATS,
    mainPlayerStat2: NIN_BIS_STATS,
    party: party,
  };
}
