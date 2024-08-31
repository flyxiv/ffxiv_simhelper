import { Box, styled } from "@mui/material";
import "./SimulationResult.css";
import { useState } from "react";
import { QuickSimResponse } from "../types/QuickSimResponse";
import {
  BestTeammateGraph,
  makeBestTeammateData,
} from "../components/graph/BestTeammateGraph";
import { DpsSummary } from "../components/container/DpsSummaryBox";
import { ResultBoardBoxStyle } from "../components/container/Styles";
import { PlayerInfo } from "../components/container/PlayerInfo";
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
import { ColorConfigurations } from "../Themes";
import { BasicLeftMenu } from "../components/container/LeftMenu";
import { AppHeader } from "../components/image/AppHeader";

const ResultBoardBox = styled(Box)`
  ${ResultBoardBoxStyle}
`;

export const DAMAGE_PROFILE_TEXT = "Damage Profile";
export const BEST_TEAMMATE_TEXT = "Best Teammate";
export const MY_CONTRIBUTIONS_TEXT = "My Contributions";
export const ROTATION_LOG_TEXT = "Rotation Log";
export const TABLE_WIDTH = "80%";

export function SimulationResult() {
  let [currentlyToggledView, setCurrentlyToggledView] =
    useState(DAMAGE_PROFILE_TEXT);
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
            {DpsSummary(mainPlayerSimulationData)}
            {PlayerInfo(mainPlayerJob, responseJson.combatTimeMillisecond)}
          </ResultBoardBox>
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
  if (currentlyToggledView === BEST_TEAMMATE_TEXT) {
    return (
      <ResultBoardBox>
        {SimulationTitle(BEST_TEAMMATE_TEXT)}
        {BestTeammateGraph(teammatesContributionToMyBuffs)}
      </ResultBoardBox>
    );
  } else if (currentlyToggledView === DAMAGE_PROFILE_TEXT) {
    return (
      <ResultBoardBox>
        {SimulationTitle(DAMAGE_PROFILE_TEXT)}
        {DamageProfileGraph(responseJson)}
      </ResultBoardBox>
    );
  } else if (currentlyToggledView == MY_CONTRIBUTIONS_TEXT) {
    return (
      <ResultBoardBox paddingBottom={5}>
        {SimulationTitle(MY_CONTRIBUTIONS_TEXT)}
        {MainPlayerContributionGraph(mainPlayerContributionToOthers)}
      </ResultBoardBox>
    );
  } else if (currentlyToggledView === ROTATION_LOG_TEXT) {
    return (
      <ResultBoardBox>
        {SimulationTitle(ROTATION_LOG_TEXT)}
        {SkillLogResult(responseJson)}
      </ResultBoardBox>
    );
  } else {
    return <div></div>;
  }
}
