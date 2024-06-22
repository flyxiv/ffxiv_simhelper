import { Box, styled } from "@mui/material";
import "./SimulationResult.css";
import { StatCompareDpsSummary } from "src/components/container/DpsSummaryBox";
import { ResultBoardBoxStyle } from "src/components/container/Styles";
import { SimulationTitle } from "src/components/basic/SimulationTitle";
import { StatComparePlayerInfo } from "src/components/container/PlayerInfo";
import {
  StatCompareRequestSaveName,
  StatCompareResponseSaveName,
} from "src/App";
import { StatCompareRequest } from "src/types/StatCompareRequest";
import { StatCompareResponse } from "src/types/StatCompareResponse";

const ResultBoardBox = styled(Box)`
  ${ResultBoardBoxStyle}
`;

export function StatCompareResult() {
  let request = localStorage.getItem(StatCompareRequestSaveName);
  let response = localStorage.getItem(StatCompareResponseSaveName);

  if (response == null || request == null) {
    return (
      <div>
        <p>Simulation Result is not available: simulation is null.</p>
      </div>
    );
  }

  let requestJson = JSON.parse(request) as StatCompareRequest;
  let responseJson = JSON.parse(response) as StatCompareResponse;

  let mainCharacterJob = requestJson.mainPlayerJob;
  let simulationData1 = responseJson.simulationGear1;
  let simulationData2 = responseJson.simulationGear2;

  return (
    <Box className="SimulationResult">
      <ResultBoardBox>
        {SimulationTitle("Simulation1")}
        {StatCompareDpsSummary(simulationData1, simulationData2)}
        {StatComparePlayerInfo(
          mainCharacterJob,
          requestJson.mainPlayerStat1,
          requestJson.mainPlayerStat2
        )}
      </ResultBoardBox>
      <ResultBoardBox>
        {SimulationTitle("Simulation2")}
        {StatCompareDpsSummary(simulationData2, simulationData1)}
        {StatComparePlayerInfo(
          mainCharacterJob,
          requestJson.mainPlayerStat2,
          requestJson.mainPlayerStat1
        )}
      </ResultBoardBox>
    </Box>
  );
}
