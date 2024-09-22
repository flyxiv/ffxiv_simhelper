import { Box, styled } from "@mui/material";
import "./SimulationResult.css";
import { useState } from "react";
import { QuickSimResponse } from "../types/QuickSimResponse";
import {
  BestTeammateGraph,
  makeBestTeammateData,
} from "../components/graph/BestTeammateGraph";
import { DpsSummary } from "../components/container/DpsSummaryBox";
import { ResultBoardBoxStyle, ResultBoardTopBoxStyle } from "../components/container/Styles";

import { SimulationTitle } from "../components/basic/SimulationTitle";
import { DamageProfileGraph } from "../components/graph/DamageProfileGraph";
import { SkillLogResult } from "../components/container/SkillLog";
import { ResultPageButtonGroup } from "../components/container/ResultPageButtonGroup";
import { QUICK_SIM_RESPONSE_SAVE_NAME } from "../App";
import { PartyContributionData } from "../components/graph/GraphData";
import {
  MainPlayerContributionGraph,
  makeMainPlayerContributionData,
} from "../components/graph/MainPlayerContributionGraph";
import { AppConfigurations } from "../Themes";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import { AppHeader } from "../components/image/AppHeader";
import { Footer } from "../components/basic/Footer";
import { PlayerInfo } from "../components/container/PlayerInfo";
import { BEST_TEAMMATE_BUTTON_TEXT, DAMAGE_PROFILE_BUTTON_TEXT, MY_CONTRIBUTION_BUTTON_TEXT, ROTATION_SAMPLE_BUTTON_TEXT, SIMULATION_RESULT_TEXT } from "../const/languageTexts";

const ResultBoardTopBox = styled(Box)`
  ${ResultBoardTopBoxStyle}
`;

const ResultBoardBox = styled(Box)`
  ${ResultBoardBoxStyle}
`;

export const TABLE_WIDTH = "80%";

export function SimulationResult() {
  let [currentlyToggledView, setCurrentlyToggledView] =
    useState(DAMAGE_PROFILE_BUTTON_TEXT);
  let response = localStorage.getItem(QUICK_SIM_RESPONSE_SAVE_NAME);

  if (response == null) {
    return (
      <div>
        <p>Simulation Result is not available: simulation result is null.</p>
      </div>
    );
  }

  let responseJson = JSON.parse(response) as QuickSimResponse;
  let mainPlayerId = responseJson.mainPlayerId;
  let mainPlayerSimulationData = responseJson.simulationData[mainPlayerId];
  let mainPlayerJob = mainPlayerSimulationData.jobAbbrev.valueOf();
  let [teammatesBuffContributionToMyBuffs, setTeammatesContributionToMyBuffs] =
    useState(null);
  let [mainPlayerContributionToOthers, setMainPlayerContributionToOthers] =
    useState(null);

  makeBestTeammateData(
    responseJson,
    teammatesBuffContributionToMyBuffs,
    setTeammatesContributionToMyBuffs
  );
  makeMainPlayerContributionData(
    responseJson,
    mainPlayerContributionToOthers,
    setMainPlayerContributionToOthers
  );

  let partyMemberJobAbbrevs: Array<string> = [];
  responseJson.simulationData.forEach((simulationData, partyMemberId) => {
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
      width="100vw"
      alignItems={"center"}
      paddingBottom={20}
    >
      <Box display="flex">
        {BasicLeftMenu()}
        <Box>
          {AppHeader()}
          <ResultBoardTopBox>
            {SimulationTitle(SIMULATION_RESULT_TEXT)}
            {DpsSummary(mainPlayerSimulationData)}
            {PlayerInfo(responseJson.mainPlayerPower, mainPlayerJob, responseJson.combatTimeMillisecond, partyMemberJobAbbrevs)}
          </ResultBoardTopBox>
          <Box display="flex" justifyContent={"center"}>
            {ResultPageButtonGroup(
              currentlyToggledView,
              setCurrentlyToggledView,
              teammatesBuffContributionToMyBuffs,
              mainPlayerContributionToOthers
            )}
          </Box>
          {renderTableBasedOnSelectedButton(
            currentlyToggledView,
            responseJson,
            teammatesBuffContributionToMyBuffs,
            mainPlayerContributionToOthers
          )}
          {Footer()}
        </Box>
      </Box>
    </Box>
  );
}

function renderTableBasedOnSelectedButton(
  currentlyToggledView: string,
  responseJson: QuickSimResponse,
  teammatesContributionToMyBuffs: null | PartyContributionData,
  mainPlayerContributionToOthers: null | PartyContributionData
) {
  if (currentlyToggledView === BEST_TEAMMATE_BUTTON_TEXT) {
    return (
      <ResultBoardBox>
        {SimulationTitle(BEST_TEAMMATE_BUTTON_TEXT)}
        {BestTeammateGraph(teammatesContributionToMyBuffs)}
      </ResultBoardBox>
    );
  } else if (currentlyToggledView === DAMAGE_PROFILE_BUTTON_TEXT) {
    return (
      <ResultBoardBox>
        {SimulationTitle(DAMAGE_PROFILE_BUTTON_TEXT)}
        {DamageProfileGraph(responseJson)}
      </ResultBoardBox>
    );
  } else if (currentlyToggledView == MY_CONTRIBUTION_BUTTON_TEXT) {
    return (
      <ResultBoardBox>
        {SimulationTitle(MY_CONTRIBUTION_BUTTON_TEXT)}
        {MainPlayerContributionGraph(mainPlayerContributionToOthers)}
      </ResultBoardBox>
    );
  } else if (currentlyToggledView === ROTATION_SAMPLE_BUTTON_TEXT) {
    return (
      <ResultBoardBox>
        {SimulationTitle(ROTATION_SAMPLE_BUTTON_TEXT)}
        {SkillLogResult(responseJson)}
      </ResultBoardBox>
    );
  } else {
    return <div></div>;
  }
}
