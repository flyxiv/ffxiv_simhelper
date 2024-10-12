import { Box, styled, Typography } from "@mui/material";
import "./SimulationResult.css";
import { useState } from "react";
import { DpsAnalysisResponse } from "../types/DpsAnalysisResponse";
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
import { DPS_ANALYSIS_RESPONSE_SAVE_NAME } from "../App";
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
import { QUICK_SIM_ITERATION_COUNT } from "../components/basic/DpsAnalysisRequestButton";
import { AppLanguageTexts } from "../const/languageTexts";

const ResultBoardTopBox = styled(Box)`
  ${ResultBoardTopBoxStyle}
`;

const ResultBoardBox = styled(Box)`
  ${ResultBoardBoxStyle}
`;

export const TABLE_WIDTH = "80%";

export function SimulationResult() {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  let [currentlyToggledView, setCurrentlyToggledView] =
    useState(LANGUAGE_TEXTS.DAMAGE_PROFILE_BUTTON_TEXT);
  let response = localStorage.getItem(DPS_ANALYSIS_RESPONSE_SAVE_NAME);

  if (response == null) {
    return (
      <div>
        <p>Simulation Result is not available: simulation result is null.</p>
      </div>
    );
  }

  let responseJson = JSON.parse(response) as DpsAnalysisResponse;
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
        {BasicLeftMenu(LANGUAGE_TEXTS.DPS_ANALYSIS_PAGE_NAME, LANGUAGE_TEXTS)}
        <Box>
          {AppHeader()}
          <ResultBoardTopBox>
            {SimulationTitle(LANGUAGE_TEXTS.SIMULATION_RESULT_TEXT)}
            {DpsSummary(mainPlayerSimulationData, "99.9% RDPS")}
            <Typography sx={{ color: 'white' }}>
              {LANGUAGE_TEXTS.EDPS_EXPLANATION_TEXT}
            </Typography>
            {PlayerInfo(responseJson.mainPlayerPower, mainPlayerJob, responseJson.combatTimeMillisecond, partyMemberJobAbbrevs, QUICK_SIM_ITERATION_COUNT, 1, LANGUAGE_TEXTS)}
          </ResultBoardTopBox>
          <Box display="flex" justifyContent={"center"}>
            {ResultPageButtonGroup(
              currentlyToggledView,
              setCurrentlyToggledView,
              teammatesBuffContributionToMyBuffs,
              mainPlayerContributionToOthers,
              LANGUAGE_TEXTS
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
  responseJson: DpsAnalysisResponse,
  teammatesContributionToMyBuffs: null | PartyContributionData,
  mainPlayerContributionToOthers: null | PartyContributionData
) {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  if (currentlyToggledView === LANGUAGE_TEXTS.BEST_TEAMMATE_BUTTON_TEXT) {
    return (
      <ResultBoardBox>
        {SimulationTitle(LANGUAGE_TEXTS.BEST_TEAMMATE_BUTTON_TEXT)}
        {BestTeammateGraph(teammatesContributionToMyBuffs, responseJson.mainPlayerJobAbbrev, LANGUAGE_TEXTS.MEMBER_TEXT, LANGUAGE_TEXTS.TOTAL_TEXT, LANGUAGE_TEXTS.MNK_ROTATION_WARNING_TEXT)}
      </ResultBoardBox>
    );
  } else if (currentlyToggledView === LANGUAGE_TEXTS.DAMAGE_PROFILE_BUTTON_TEXT) {
    return (
      <ResultBoardBox>
        {SimulationTitle(LANGUAGE_TEXTS.DAMAGE_PROFILE_BUTTON_TEXT)}
        {DamageProfileGraph(responseJson, LANGUAGE_TEXTS)}
      </ResultBoardBox>
    );
  } else if (currentlyToggledView == LANGUAGE_TEXTS.MY_CONTRIBUTION_BUTTON_TEXT) {
    return (
      <ResultBoardBox>
        {SimulationTitle(LANGUAGE_TEXTS.MY_CONTRIBUTION_BUTTON_TEXT)}
        {MainPlayerContributionGraph(mainPlayerContributionToOthers, LANGUAGE_TEXTS.MEMBER_TEXT, LANGUAGE_TEXTS.TOTAL_TEXT)}
      </ResultBoardBox>
    );
  } else if (currentlyToggledView === LANGUAGE_TEXTS.ROTATION_SAMPLE_BUTTON_TEXT) {
    return (
      <ResultBoardBox>
        {SimulationTitle(LANGUAGE_TEXTS.ROTATION_SAMPLE_BUTTON_TEXT)}
        {SkillLogResult(responseJson, LANGUAGE_TEXTS)}
      </ResultBoardBox>
    );
  } else {
    return <div></div>;
  }
}
