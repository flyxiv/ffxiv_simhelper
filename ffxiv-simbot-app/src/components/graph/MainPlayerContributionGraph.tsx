import { QuickSimResponse } from "src/types/QuickSimResponse";
import {
  PartyContributionData,
  StatusKey,
  statusKeyStringToKey,
  statusKeyToString,
  TeammateChartData,
} from "./GraphData";
import { Box, styled } from "@mui/material";
import { GraphBoxStyle } from "./Style";
import { GraphTitleRow, JobBarChartTeammate } from "./JobBarChart";

const GraphBox = styled(Box)`
  ${GraphBoxStyle}
`;

// main player id has to be 0
export const makeMainPlayerContributionData = (
  response: QuickSimResponse,
  mainPlayerContributionToOthers: null | PartyContributionData,
  setMainPlayerContributionToOthers: Function
) => {
  if (mainPlayerContributionToOthers !== null) {
    return;
  }

  const mainPlayerId = response.mainPlayerId;
  const simulationDatas = response.simulationData;
  let mainPlayerContributionData: Array<TeammateChartData> = [];

  for (
    let partyMemberId = 1;
    partyMemberId < simulationDatas.length;
    partyMemberId++
  ) {
    mainPlayerContributionData.push({
      jobName: simulationDatas[partyMemberId].jobAbbrev,
      totalRdps: 0,
      rdpsEntry: [],
    });
  }

  let totalRdpsByStatus = new Map<number, number>();
  let totalRdpsByKey = new Map<string, number>();

  let partyContributionsOfMainPlayer =
    simulationDatas[mainPlayerId].partyContributionTable;

  // statuskey: "statusId-playerId" to string
  let rdpsEntries: Map<string, number> = new Map();

  for (const partyContribution of partyContributionsOfMainPlayer) {
    let partyMemberId = partyContribution.partyMemberId;
    let statusId = partyContribution.statusId;
    let statusKey = { statusId: statusId, playerId: partyMemberId };
    let statusKeyString = statusKeyToString(statusKey);

    let currentRdpsOfStatus = rdpsEntries.get(statusKeyString) || 0;
    rdpsEntries.set(
      statusKeyString,
      currentRdpsOfStatus + partyContribution.contributedRdps
    );
  }

  rdpsEntries.forEach((rdps, statusKeyString) => {
    let statusKey = statusKeyStringToKey(statusKeyString);
    let rdpsOfStatus = totalRdpsByStatus.get(statusKey.statusId) || 0;
    let rdpsOfKey = totalRdpsByKey.get(statusKeyString) || 0;

    totalRdpsByStatus.set(statusKey.statusId, rdpsOfStatus + rdps);
    totalRdpsByKey.set(statusKeyString, rdpsOfKey + rdps);
  });

  totalRdpsByKey.forEach((rdps, statusKeyString) => {
    let statusKey = statusKeyStringToKey(statusKeyString);

    let playerId = statusKey.playerId;
    let contributionData = mainPlayerContributionData[playerId - 1];
    contributionData.jobName = simulationDatas[playerId].jobAbbrev;
    contributionData.totalRdps = rdps;
    contributionData.rdpsEntry.push({
      statusId: statusKey.statusId,
      rdps: rdps,
    });
  });

  mainPlayerContributionData = mainPlayerContributionData.filter(
    (data) => data.totalRdps > 0
  );
  mainPlayerContributionData.sort((a, b) => b.totalRdps - a.totalRdps);

  setMainPlayerContributionToOthers({
    totalRdpsByStatus: totalRdpsByStatus,
    contributionData: mainPlayerContributionData,
  });
};

export const MainPlayerContributionGraph = (
  mainPlayerContributionData: null | PartyContributionData
) => {
  if (mainPlayerContributionData === null) {
    return;
  }

  let contributionData = mainPlayerContributionData.contributionData;
  let totalRdpsByStatus = mainPlayerContributionData.totalRdpsByStatus;

  let maxContribution = contributionData[0].totalRdps;

  for (const data of contributionData) {
    data.rdpsEntry.sort(
      (a, b) =>
        (totalRdpsByStatus.get(b.statusId) || 0) -
        (totalRdpsByStatus.get(a.statusId) || 0)
    );
  }

  return (
    <>
      {GraphTitleRow()}
      <GraphBox>
        {contributionData.map((data) => {
          return JobBarChartTeammate(data, maxContribution);
        })}
      </GraphBox>
    </>
  );
};
