import { QuickSimRequest } from "src/types/QuickSimRequest";
import {
  DEFAULT_WEAPON_DAMAGE,
  DEFAULT_CRITICAL_STRIKE,
  DEFAULT_DIRECT_HIT,
  DEFAULT_DETERMINATION,
  DEFAULT_SPEED,
  DEFAULT_TENACITY,
  DEFAULT_MAIN_STAT_NON_TANK,
} from "../const/StatValue";

export function defaultQuickSimRequest(): QuickSimRequest {
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
  let i = 0;

  let party = [];

  for (i = 0; i < defaultJobsList.length; i++) {
    party.push({
      playerId: i,
      job: defaultJobsList[i],
      role: "",
      partner1Id: null,
      partner2Id: null,
      stats: {
        weaponDamage: DEFAULT_WEAPON_DAMAGE,
        mainStat: DEFAULT_MAIN_STAT_NON_TANK,
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
    combatTimeMillisecond: 120000,
    party: party,
  };
}
