import {
  PartyContributionData,
  statusKeyStringToKey,
  statusKeyToString,
  TeammateChartData,
} from "./GraphData";
import { Box, styled } from "@mui/material";
import { GraphBoxStyle } from "./Style";
import { GraphTitleRow, JobBarChartTeammate } from "./JobBarChart";
import { TABLE_WIDTH } from "../../page/SimulationResult";
import { DpsAnalysisResponse } from "../../types/DpsAnalysisResponse";

const GraphBox = styled(Box)`
  ${GraphBoxStyle}
`;

// main player id has to be 0
export const makeMainPlayerContributionData = (
  response: DpsAnalysisResponse,
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

  let totalContributionByStatus = new Map<number, number>();
  let totalContributionByKey = new Map<string, number>();

  let partyContributionsOfMainPlayer =
    simulationDatas[mainPlayerId].partyContributionTable;

  // statuskey: "statusId-playerId" to string
  let rdpsEntries: Map<string, number> = new Map();

  for (const partyContribution of partyContributionsOfMainPlayer) {
    let partyMemberId = partyContribution.partyMemberId;
    let statusId = partyContribution.statusId;
    let statusKey = { statusId: statusId, playerId: partyMemberId };
    let statusKeyString = statusKeyToString(statusKey);

    let currentDamageOfStatus = rdpsEntries.get(statusKeyString) || 0;
    rdpsEntries.set(
      statusKeyString,
      currentDamageOfStatus + partyContribution.contributedDamage
    );
  }

  rdpsEntries.forEach((rdps, statusKeyString) => {
    let statusKey = statusKeyStringToKey(statusKeyString);
    let contributionOfStatus = totalContributionByStatus.get(statusKey.statusId) || 0;
    let contributionOfKey = totalContributionByKey.get(statusKeyString) || 0;

    totalContributionByStatus.set(statusKey.statusId, contributionOfStatus + rdps);
    totalContributionByKey.set(statusKeyString, contributionOfKey + rdps);
  });

  totalContributionByKey.forEach((totalContribution, statusKeyString) => {
    let statusKey = statusKeyStringToKey(statusKeyString);

    let rdps = totalContribution / response.combatTimeMillisecond * 1000;

    let playerId = statusKey.playerId;
    let contributionData = mainPlayerContributionData[playerId - 1];
    contributionData.jobName = simulationDatas[playerId].jobAbbrev;
    contributionData.totalRdps += rdps;
    contributionData.rdpsEntry.push({
      statusId: statusKey.statusId,
      rdps
    });
  });

  mainPlayerContributionData = mainPlayerContributionData.filter(
    (data) => data.totalRdps > 0
  );
  mainPlayerContributionData.sort((a, b) => b.totalRdps - a.totalRdps);

  const totalRdpsByStatus = new Map<number, number>();
  for (const [statusId, contribution] of totalContributionByStatus) {
    let rdps = Math.floor(contribution / response.combatTimeMillisecond * 1000);
    totalRdpsByStatus.set(statusId, rdps);
  }

  setMainPlayerContributionToOthers({
    totalRdpsByStatus,
    contributionData: mainPlayerContributionData,
  });
};

export const MainPlayerContributionGraph = (
  mainPlayerContributionData: null | PartyContributionData,
  memberText: string,
  totalText: string
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
    <Box width={TABLE_WIDTH}>
      {GraphTitleRow(memberText, totalText)}
      <GraphBox>
        {contributionData.map((data) => {
          return JobBarChartTeammate(data, maxContribution);
        })}
      </GraphBox>
    </Box>
  );
};
