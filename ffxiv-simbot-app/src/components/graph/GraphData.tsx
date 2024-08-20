export interface DamageChartData {
  icon: String;
  rdps: number;
  pdps: number;
  castCount: number;
  name: String;
}

export interface RdpsEntry {
  statusId: number;
  rdps: number;
}
export interface TeammateChartData {
  rdpsEntry: RdpsEntry[];
  totalRdps: number;
  jobName: String;
}

export interface PartyContributionData {
  totalRdpsByStatus: Map<number, number>;
  contributionData: Array<TeammateChartData>;
}

export interface StatusKey {
  statusId: number;
  playerId: number;
}

export function statusKeyToString(key: StatusKey): string {
  return key.statusId + "-" + key.playerId;
}

export function statusKeyStringToKey(keyString: string): StatusKey {
  let split = keyString.split("-");
  return { statusId: parseInt(split[0]), playerId: parseInt(split[1]) };
}
