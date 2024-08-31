import { Box, styled } from "@mui/material";
import "./SimulationResult.css";
import { GearCompareDpsSummary } from "../components/container/DpsSummaryBox";
import { ResultBoardBoxStyle } from "../components/container/Styles";
import { SimulationTitle } from "../components/basic/SimulationTitle";
import { StatComparePlayerInfo } from "../components/container/PlayerInfo";
import { GEAR_COMPARE_RESPONSE_SAVE_NAME } from "../App";
import { GearCompareResponse } from "src/types/GearCompareResponse";

const ResultBoardBox = styled(Box)`
  ${ResultBoardBoxStyle}
`;

export function GearCompareResult() {
  let response = localStorage.getItem(GEAR_COMPARE_RESPONSE_SAVE_NAME);
  console.log(response);

  if (response == null) {
    return (
      <div>
        <p>Simulation Result is not available: simulation is null.</p>
      </div>
    );
  }

  let responseJson = JSON.parse(response) as GearCompareResponse;

  let mainPlayerId = responseJson.simulationGear1.mainPlayerId;
  let mainCharacterJob = responseJson.simulationGear1.mainPlayerJobAbbrev;

  let simulationResult1 = responseJson.simulationGear1;
  let simulationResult2 = responseJson.simulationGear2;

  let simulationData1 =
    simulationResult1.simulationData[mainPlayerId].simulationSummary;
  let simulationData2 =
    simulationResult2.simulationData[mainPlayerId].simulationSummary;
  let combatTimeMilliseconds = simulationResult1.combatTimeMillisecond;

  return (
    <Box className="SimulationResult">
      <ResultBoardBox>
        {SimulationTitle("Simulation1")}
        {GearCompareDpsSummary(simulationData1, simulationData2)}
        {StatComparePlayerInfo(
          mainCharacterJob,
          simulationResult1.mainPlayerPower,
          simulationResult2.mainPlayerPower,
          combatTimeMilliseconds
        )}
      </ResultBoardBox>
      <ResultBoardBox>
        {SimulationTitle("Simulation2")}
        {GearCompareDpsSummary(simulationData2, simulationData1)}
        {StatComparePlayerInfo(
          mainCharacterJob,
          simulationResult2.mainPlayerPower,
          simulationResult1.mainPlayerPower,
          combatTimeMilliseconds
        )}
      </ResultBoardBox>
    </Box>
  );
}
