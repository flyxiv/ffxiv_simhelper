import { QuickSimResponse } from "src/types/QuickSimResponse";
import { JobBarChartTeammate, TeammateChartData } from "./JobBarChart";
import { Box } from "@mui/material";

export const BestTeammateGraph = (response: QuickSimResponse) => {
  const mainPlayerId = response.mainPlayerId;
  const simulationDatas = response.simulationData;

  let contributionData: Array<TeammateChartData> = [];
  let i = 0;

  for (i = 0; i < simulationDatas.length; i++) {
    if (simulationDatas[i].playerId === mainPlayerId) {
      continue;
    }

    let totalRdpsContributionToMainPlayer = 0;

    for (let j = 0; j < simulationDatas[i].partyContributionTable.length; j++) {
      if (
        simulationDatas[i].partyContributionTable[j].partyMemberId ===
        mainPlayerId
      ) {
        totalRdpsContributionToMainPlayer +=
          simulationDatas[i].partyContributionTable[j].contributedRdps;
      }
    }

    contributionData.push({
      jobName: simulationDatas[i].job,
      rdps: totalRdpsContributionToMainPlayer,
    });
  }

  contributionData.sort((a, b) => b.rdps - a.rdps);

  return (
    <Box width={500} height={80 + contributionData.length * 40}>
      <JobBarChartTeammate data={contributionData} />
    </Box>
  );
};
