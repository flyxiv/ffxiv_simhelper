import { SimulationData } from "./CombatSimulationResult";

export interface QuickSimResponse {
  mainPlayerId: number;
  combatTimeMillisecond: number;
  simulationData: Array<SimulationData>;
}
