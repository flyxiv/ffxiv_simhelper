import { DpsAnalysisResponse } from "../../types/DpsAnalysisResponse";
import { GraphTitleRow, JobBarChartTeammate } from "./JobBarChart";
import { Box, styled } from "@mui/material";
import { GraphBoxStyle } from "./Style";
import { PartyContributionData, TeammateChartData } from "./GraphData";
import { TABLE_WIDTH } from "../../page/SimulationResult";
import { WarningText } from "../basic/WarningText";
import { LANGUAGE_TEXTS } from "../../const/languageTexts";

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

  let teammatesContributionToMyBuffs = {
    totalRdpsByStatus: totalRdpsByStatus,
    contributionData: contributionData,
  };

  setTeammatesContributionToMyBuffs(teammatesContributionToMyBuffs);
  return;
};

export const BestTeammateGraph = (
  teammatesContributionToMyBuffs: null | PartyContributionData,
  jobAbbrev: string
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
      {GraphTitleRow()}
      {
        jobAbbrev === LANGUAGE_TEXTS.MNK_EN_NAME ? WarningText(LANGUAGE_TEXTS.MNK_ROTATION_WARNING_TEXT) : <Box />
      }
      <GraphBox>
        {teammateContributionData.map((data) => {
          return JobBarChartTeammate(data, maxContribution);
        })}
      </GraphBox>
    </Box >
  );
};
