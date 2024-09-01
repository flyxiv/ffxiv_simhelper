import { Box, styled } from "@mui/material";
import { ResultBoardBoxStyle } from "../components/container/Styles";
import { PlayerInfo } from "../components/container/PlayerInfo";
import { SimulationTitle } from "../components/basic/SimulationTitle";
import { STAT_WEIGHTS_RESPONSE_SAVE_NAME } from "../App";
import { ColorConfigurations } from "../Themes";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import { AppHeader } from "../components/image/AppHeader";
import { Footer } from "../components/basic/Footer";
import { StatWeightsResponseTable } from "../types/StatWeightsResponse";
import { StatWeightsGraphContainer } from "../components/graph/StatWeightsGraph";

export interface StatWeightsData {
  statName: string;
  rdpsIncreasePerPoint: number;
}

const ResultBoardBox = styled(Box)`
  ${ResultBoardBoxStyle}
`;

const BEST_PARTNERS_BY_ROLE_TEXT = "Stat Weights Result";
export const TABLE_WIDTH = "80%";

export function StatWeightsResult() {
  let response = localStorage.getItem(STAT_WEIGHTS_RESPONSE_SAVE_NAME);

  if (response == null) {
    return (
      <div>
        <p>Simulation Result is not available: simulation result is null.</p>
      </div>
    );
  }

  let responseJson = JSON.parse(response) as StatWeightsResponseTable;
  let mainPlayerJob = responseJson.mainPlayerJobAbbrev;
  let statWeightsTable = responseJson.statAugmentedSimulationData;
  let statWeightsBaseline = statWeightsTable.filter(
    (statWeight) => statWeight.statName === ""
  )[0];
  let statWeightsCalculated: Array<StatWeightsData> = statWeightsTable
    .map((statWeight) => {
      return {
        statName: statWeight.statName,
        rdpsIncreasePerPoint:
          (statWeight.dps - statWeightsBaseline.dps) / statWeight.augmentAmount,
      };
    })
    .filter((statWeight) => statWeight.statName !== "")
    .sort((a, b) => b.rdpsIncreasePerPoint - a.rdpsIncreasePerPoint);

  console.log(statWeightsCalculated);

  return (
    <Box
      display="flex"
      flexDirection={"column"}
      sx={{ backgroundColor: ColorConfigurations.backgroundOne }}
      width="100vw"
      alignItems={"center"}
      paddingBottom={20}
    >
      <Box display="flex">
        {BasicLeftMenu()}
        <Box>
          {AppHeader()}
          <ResultBoardBox>
            {SimulationTitle("Simulation Result")}
            {PlayerInfo(mainPlayerJob, responseJson.combatTimeMillisecond)}
          </ResultBoardBox>
          <ResultBoardBox>
            {SimulationTitle(BEST_PARTNERS_BY_ROLE_TEXT)}
            {StatWeightsGraphContainer(statWeightsCalculated)}
          </ResultBoardBox>
          {Footer()}
        </Box>
      </Box>
    </Box>
  );
}
