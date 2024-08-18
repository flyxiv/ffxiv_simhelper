import { QuickSimResponse } from "../../types/QuickSimResponse";
import {
  GraphTitleRow,
  JobBarChartTeammate,
  TeammateChartData,
} from "./JobBarChart";
import { Box, styled } from "@mui/material";
import { GraphBoxStyle } from "./Style";

const GraphBox = styled(Box)`
  ${GraphBoxStyle}
`;

export const BestTeammateGraph = (response: QuickSimResponse) => {
  const mainPlayerId = response.mainPlayerId;
  const simulationDatas = response.simulationData;

  let contributionData: Array<TeammateChartData> = [];
  let totalRdpsByStatus = new Map<number, number>();
  let i = 0;

  for (i = 0; i < simulationDatas.length; i++) {
    if (simulationDatas[i].playerId === mainPlayerId) {
      continue;
    }

    let teammateChartData: TeammateChartData = {
      jobName: simulationDatas[i].jobAbbrev,
      totalRdps: 0,
      rdpsEntry: [],
    };

    let rdpsByStatus = new Map<number, number>();

    for (let j = 0; j < simulationDatas[i].partyContributionTable.length; j++) {
      let rdpsEntry = simulationDatas[i].partyContributionTable[j];
      if (rdpsEntry.partyMemberId === mainPlayerId) {
        let currentRdpsOfStatus = rdpsByStatus.get(rdpsEntry.statusId) || 0;
        let currentTotalRdpsOfBuff =
          totalRdpsByStatus.get(rdpsEntry.statusId) || 0;
        rdpsByStatus.set(
          rdpsEntry.statusId,
          currentRdpsOfStatus + rdpsEntry.contributedRdps
        );
        totalRdpsByStatus.set(
          rdpsEntry.statusId,
          currentTotalRdpsOfBuff + rdpsEntry.contributedRdps
        );
        teammateChartData.totalRdps +=
          simulationDatas[i].partyContributionTable[j].contributedRdps;
      }
    }

    rdpsByStatus.forEach((rdps, statusId) => {
      teammateChartData.rdpsEntry.push({
        statusId: statusId,
        rdps: rdps,
      });
    });
    contributionData.push(teammateChartData);
  }

  contributionData.sort((a, b) => b.totalRdps - a.totalRdps);

  for (let i = 0; i < contributionData.length; i++) {
    let data = contributionData[i];
    data.rdpsEntry.sort(
      (a, b) =>
        (totalRdpsByStatus.get(b.statusId) || 0) -
        (totalRdpsByStatus.get(a.statusId) || 0)
    );
  }

  let maxContribution = contributionData[0].totalRdps;

  return (
    <>
      {GraphTitleRow}
      <GraphBox>
        {contributionData.map((data) => {
          return JobBarChartTeammate(data, maxContribution);
        })}
      </GraphBox>
    </>
  );
};
