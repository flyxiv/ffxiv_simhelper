import { Box, styled } from "@mui/material";
import "./SimulationResult.css";
import { useState } from "react";
import { QuickSimResponse } from "../types/QuickSimResponse";
import { BestTeammateGraph } from "../components/graph/BestTeammateGraph";
import { DpsSummary } from "src/components/container/DpsSummaryBox";
import { ResultBoardBoxStyle } from "src/components/container/Styles";
import { PlayerInfo } from "src/components/container/PlayerInfo";
import { SimulationTitle } from "src/components/basic/SimulationTitle";
import { DamageProfileGraph } from "src/components/graph/DamageProfileGraph";
import { SkillLogResult } from "src/components/container/SkillLog";
import { ResultPageButtonGroup } from "src/components/container/ResultPageButtonGroup";
import { QuickSimResponseSaveName } from "src/App";

const ResultBoardBox = styled(Box)`
  ${ResultBoardBoxStyle}
`;

export function SimulationResult() {
  let [currentlyToggledView, setCurrentlyToggledView] =
    useState("Damage Profile");
  let response = localStorage.getItem(QuickSimResponseSaveName);

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

  return (
    <Box className="SimulationResult">
      <ResultBoardBox>
        {SimulationTitle("Simulation Result")}
        {DpsSummary(mainPlayerSimulationData)}
        {PlayerInfo(mainPlayerJob, responseJson.combatTimeMillisecond)}
      </ResultBoardBox>
      {ResultPageButtonGroup(
        currentlyToggledView,
        setCurrentlyToggledView,
        mainPlayerJob
      )}
      {renderTableBasedOnSelectedButton(currentlyToggledView, responseJson)}
    </Box>
  );
}

function renderTableBasedOnSelectedButton(
  currentlyToggledView: string,
  responseJson: QuickSimResponse
) {
  if (currentlyToggledView === "Best Teammate") {
    return (
      <ResultBoardBox>
        {SimulationTitle("Best Teammate")}
        {BestTeammateGraph(responseJson)}
      </ResultBoardBox>
    );
  } else if (currentlyToggledView === "Damage Profile") {
    return (
      <ResultBoardBox>
        {SimulationTitle("Damage Profile")}
        {DamageProfileGraph(responseJson)}
      </ResultBoardBox>
    );
  } else if (currentlyToggledView === "Rotation Log") {
    return (
      <ResultBoardBox>
        {SimulationTitle("Rotation Log")}
        {SkillLogResult(responseJson)}
      </ResultBoardBox>
    );
  } else {
    return <div></div>;
  }
}
