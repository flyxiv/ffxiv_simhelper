import { Box, styled } from "@mui/material";
import { ResultBoardBoxStyle, ResultBoardTopBoxStyle } from "../components/container/Styles";
import { PlayerInfo } from "../components/container/PlayerInfo";
import { SimulationTitle } from "../components/basic/SimulationTitle";
import { STAT_WEIGHTS_RESPONSE_SAVE_NAME } from "../App";
import { AppConfigurations } from "../Themes";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import { AppHeader } from "../components/image/AppHeader";
import { Footer } from "../components/basic/Footer";
import { BestStatsResponseTable } from "../types/BestStats";
import { StatWeightSummary } from "../components/container/StatSummary";
import { EMPTY_PARTY_MEMBER } from "../types/PartyStates";
import { STAT_WEIGHTS_REQUEST_COUNT } from "../components/basic/BestStatsRequestButton";
import { AppLanguageTexts } from "../const/languageTexts";

export interface StatWeightsData {
  statName: string;
  rdpsIncreasePerPoint: number;
}

const ResultBoardBox = styled(Box)`
  ${ResultBoardBoxStyle}
`;

const ResultBoardTopBox = styled(Box)`
  ${ResultBoardTopBoxStyle}
`;

export const TABLE_WIDTH = "80%";

export function StatWeightsResult() {
  let LANGUAGE_TEXTS = AppLanguageTexts();
  let response = localStorage.getItem(STAT_WEIGHTS_RESPONSE_SAVE_NAME);

  if (response == null) {
    return (
      <div>
        <p>Simulation Result is not available: simulation result is null.</p>
      </div>
    );
  }

  let responseJson = JSON.parse(response) as BestStatsResponseTable;
  let mainPlayerJob = responseJson.mainPlayerJobAbbrev;
  let partyMemberJobAbbrevs = responseJson.partyMemberJobAbbrevs.filter((job) => job !== EMPTY_PARTY_MEMBER);
  let statWeightsTable = responseJson.statAugmentedSimulationData;
  let statWeightsBaseline = statWeightsTable.filter(
    (statWeight) => statWeight.statName === ""
  )[0];
  let statWeightsCalculated: Array<StatWeightsData> = statWeightsTable
    .map((statWeight) => {
      return {
        statName: statWeight.statName,
        rdpsIncreasePerPoint:
          Math.max((statWeight.dps - statWeightsBaseline.dps) / statWeight.augmentAmount, 0.01),
      };
    })
    .filter((statWeight) => statWeight.statName !== "")
    .sort((a, b) => b.rdpsIncreasePerPoint - a.rdpsIncreasePerPoint);

  return (
    <Box
      display="flex"
      flexDirection={"column"}
      sx={{ backgroundColor: AppConfigurations.backgroundOne }}
      width="100vw"
      alignItems={"center"}
      paddingBottom={20}
    >
      <Box display="flex">
        {BasicLeftMenu(LANGUAGE_TEXTS.BEST_STAT_PAGE_NAME, LANGUAGE_TEXTS)}
        <Box>
          {AppHeader()}
          <ResultBoardTopBox marginBottom="40px">
            {SimulationTitle(LANGUAGE_TEXTS.SIMULATION_RESULT_TEXT)}
            {PlayerInfo(responseJson.mainPlayerPower, mainPlayerJob, responseJson.combatTimeMillisecond, partyMemberJobAbbrevs, STAT_WEIGHTS_REQUEST_COUNT, 1, LANGUAGE_TEXTS)}
          </ResultBoardTopBox>
          <ResultBoardBox >
            {SimulationTitle(LANGUAGE_TEXTS.BEST_STATS_TEXT)}
            {StatWeightSummary(statWeightsCalculated, LANGUAGE_TEXTS)}
          </ResultBoardBox>
          {Footer()}
        </Box>
      </Box>
    </Box>
  );
}
