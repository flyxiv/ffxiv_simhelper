import { Box, styled, Typography } from "@mui/material";
import "./SimulationResult.css";
import { GearCompareDpsSummary } from "../components/container/DpsSummaryBox";
import { ResultBoardTopBoxStyle } from "../components/container/Styles";
import { SimulationTitle } from "../components/basic/SimulationTitle";
import { StatComparePlayerInfo } from "../components/container/PlayerInfo";
import { BODY_WIDTH, GEAR_COMPARE_RESPONSE_SAVE_NAME } from "../App";
import { GearCompareResponse } from "../types/GearCompareResponse";
import { AppConfigurations } from "../Themes";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import { AppHeader } from "../components/image/AppHeader";
import { Footer } from "../components/basic/Footer";
import { GEAR_COMPARE_ITERATION_COUNT } from "../components/basic/GearCompareRequestButton";
import { AppLanguageTexts } from "../const/languageTexts";

const ResultBoardTopBox = styled(Box)`
  ${ResultBoardTopBoxStyle}
`;

export function GearCompareResult() {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  let response = localStorage.getItem(GEAR_COMPARE_RESPONSE_SAVE_NAME);

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
      sx={{ backgroundColor: AppConfigurations.backgroundOne }}
      width="100%"
      alignItems={"center"}
      paddingBottom={20}
    >
      <Box display="flex" width="100%" justifyContent={"center"} alignItems={"center"}>
        {BasicLeftMenu(LANGUAGE_TEXTS.GEAR_COMPARE_PAGE_NAME, LANGUAGE_TEXTS)}
        <Box width={BODY_WIDTH}>
          {AppHeader()}
          <Box className="SimulationResult" sx={{
            backgroundColor: AppConfigurations.backgroundOne
          }}>
            <ResultBoardTopBox marginBottom="50px">
              {SimulationTitle(LANGUAGE_TEXTS.GEARSET1_TEXT)}
              {GearCompareDpsSummary(simulationData1, simulationData2)}
              <Typography sx={{ color: 'white' }} align="center">
                {LANGUAGE_TEXTS.EDPS_EXPLANATION_TEXT}
              </Typography>
              {StatComparePlayerInfo(
                mainCharacterJob,
                simulationResult1.mainPlayerPower,
                simulationResult2.mainPlayerPower,
                combatTimeMilliseconds,
                partyMemberJobAbbrevs,
                GEAR_COMPARE_ITERATION_COUNT,
                0.1,
                LANGUAGE_TEXTS
              )}
            </ResultBoardTopBox>
            <ResultBoardTopBox>
              {SimulationTitle(LANGUAGE_TEXTS.GEARSET2_TEXT)}
              {GearCompareDpsSummary(simulationData2, simulationData1)}
              <Typography sx={{ color: 'white' }} align="center">
                {LANGUAGE_TEXTS.EDPS_EXPLANATION_TEXT}
              </Typography>
              {StatComparePlayerInfo(
                mainCharacterJob,
                simulationResult2.mainPlayerPower,
                simulationResult1.mainPlayerPower,
                combatTimeMilliseconds,
                partyMemberJobAbbrevs,
                GEAR_COMPARE_ITERATION_COUNT,
                0.1,
                LANGUAGE_TEXTS
              )}
            </ResultBoardTopBox>
          </Box>
          {Footer()}
        </Box>

      </Box>

    </Box >
  );
}
