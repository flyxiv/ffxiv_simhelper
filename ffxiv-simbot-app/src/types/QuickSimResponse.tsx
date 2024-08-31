import { SimulationData } from "./CombatSimulationResult";
import { PlayerPower } from "./ffxivdatabase/PlayerPower";

export interface QuickSimResponse {
  mainPlayerId: number;
  combatTimeMillisecond: number;
  simulationData: Array<SimulationData>;
  mainPlayerPower: PlayerPower;
  mainPlayerJobAbbrev: string;
}
