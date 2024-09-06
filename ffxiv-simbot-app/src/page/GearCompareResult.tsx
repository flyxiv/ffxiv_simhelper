import { Box, styled } from "@mui/material";
import "./SimulationResult.css";
import { GearCompareDpsSummary } from "../components/container/DpsSummaryBox";
import { ResultBoardTopBoxStyle } from "../components/container/Styles";
import { SimulationTitle } from "../components/basic/SimulationTitle";
import { StatComparePlayerInfo } from "../components/container/PlayerInfo";
import { GEAR_COMPARE_RESPONSE_SAVE_NAME } from "../App";
import { GearCompareResponse } from "../types/GearCompareResponse";
import { ColorConfigurations } from "../Themes";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import { AppHeader } from "../components/image/AppHeader";
import { Footer } from "../components/basic/Footer";

const ResultBoardTopBox = styled(Box)`
  ${ResultBoardTopBoxStyle}
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
  let partyMemberJobAbbrevs: Array<string> = [];
  simulationResult1.simulationData.forEach((simulationData, partyMemberId) => {
    if (partyMemberId === mainPlayerId) {
      return;
    }

    partyMemberJobAbbrevs.push(simulationData.jobAbbrev);
  })


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
          <Box className="SimulationResult" sx={{
            backgroundColor: ColorConfigurations.backgroundOne
          }}>
            <ResultBoardTopBox marginBottom="50px">
              {SimulationTitle("GearSet1")}
              {GearCompareDpsSummary(simulationData1, simulationData2)}
              {StatComparePlayerInfo(
                mainCharacterJob,
                simulationResult1.mainPlayerPower,
                simulationResult2.mainPlayerPower,
                combatTimeMilliseconds,
                partyMemberJobAbbrevs
              )}
            </ResultBoardTopBox>
            <ResultBoardTopBox>
              {SimulationTitle("GearSet2")}
              {GearCompareDpsSummary(simulationData2, simulationData1)}
              {StatComparePlayerInfo(
                mainCharacterJob,
                simulationResult2.mainPlayerPower,
                simulationResult1.mainPlayerPower,
                combatTimeMilliseconds,
                partyMemberJobAbbrevs
              )}
            </ResultBoardTopBox>
          </Box>
          {Footer()}
        </Box>

      </Box>

    </Box >
  );
}
