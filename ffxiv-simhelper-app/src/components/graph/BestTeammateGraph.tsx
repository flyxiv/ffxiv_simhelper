import { DpsAnalysisResponse } from "../../types/DpsAnalysisResponse";
import { GraphTitleRow, JobBarChartTeammate } from "./JobBarChart";
import { Box, styled } from "@mui/material";
import { GraphBoxStyle } from "./Style";
import { PartyContributionData, TeammateChartData } from "./GraphData";
import { TABLE_WIDTH } from "../../page/SimulationResult";
import { WarningText } from "../basic/WarningText";
import { MNK_EN_NAME } from "../../const/languageTexts";

const GraphBox = styled(Box)`
  ${GraphBoxStyle}
`;

export const makeBestTeammateData = (
  response: DpsAnalysisResponse,
  teammatesContributions: null | PartyContributionData,
  setTeammatesContributionToMyBuffs: Function
) => {
  if (teammatesContributions !== null) {
    return;
  }

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
      let rdps = rdpsEntry.contributedDamage / response.combatTimeMillisecond * 1000;
      if (rdpsEntry.partyMemberId === mainPlayerId) {
        let currentRdpsOfStatus = rdpsByStatus.get(rdpsEntry.statusId) || 0;
        let currentTotalRdpsOfBuff =
          totalRdpsByStatus.get(rdpsEntry.statusId) || 0;
        rdpsByStatus.set(
          rdpsEntry.statusId,
          currentRdpsOfStatus + rdps
        );
        totalRdpsByStatus.set(
          rdpsEntry.statusId,
          currentTotalRdpsOfBuff + rdps
        );
        teammateChartData.totalRdps +=
          simulationDatas[i].partyContributionTable[j].contributedDamage / response.combatTimeMillisecond * 1000;
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
  console.log(contributionData);
  console.log(totalRdpsByStatus);


  let teammatesContributionToMyBuffs = {
    totalRdpsByStatus: totalRdpsByStatus,
    contributionData: contributionData,
  };

  setTeammatesContributionToMyBuffs(teammatesContributionToMyBuffs);
  return;
};

export const BestTeammateGraph = (
  teammatesContributionToMyBuffs: null | PartyContributionData,
  jobAbbrev: string,
  memberText: string,
  totalText: string,
  mnkRotationWarningText: string
) => {
  if (teammatesContributionToMyBuffs === null) {
    return;
  }

  let teammateContributionData =
    teammatesContributionToMyBuffs.contributionData;
  let totalRdpsByStatus = teammatesContributionToMyBuffs.totalRdpsByStatus;

  let maxContribution = teammateContributionData[0].totalRdps;

  for (let i = 0; i < teammateContributionData.length; i++) {
    let data = teammateContributionData[i];
    data.rdpsEntry.sort(
      (a, b) =>
        (totalRdpsByStatus.get(b.statusId) || 0) -
        (totalRdpsByStatus.get(a.statusId) || 0)
    );
  }

  return (
    <Box width={TABLE_WIDTH}>
      {GraphTitleRow(memberText, totalText)}
      {
        jobAbbrev === MNK_EN_NAME ? WarningText(mnkRotationWarningText) : <Box />
      }
      <GraphBox>
        {teammateContributionData.map((data) => {
          return JobBarChartTeammate(data, maxContribution);
        })}
      </GraphBox>
    </Box >
  );
};
