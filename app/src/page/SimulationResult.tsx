import { Box, Typography, styled } from "@mui/material";
import "./SimulationResult.css";
import { QuickSimResponse } from "../types/QuickSimResponse";
import { BestTeammateGraph } from "../components/graph/BestTeammateGraph";
import { DamageProfileGraph } from "../components/graph/DamageProfileGraph";
import { SkillLogResult } from "../components/container/SkillLog";
import { DpsSummary } from "src/components/container/DpsSummaryBox";
import {
  SummaryBoardBoxStyle,
  TitleBoxStyle,
} from "src/components/container/Styles";
import { PlayerInfo } from "src/components/container/PlayerInfo";

const SummaryBoardBox = styled(Box)`
  ${SummaryBoardBoxStyle}
`;

const TitleBox = styled(Box)`
  ${TitleBoxStyle}
`;

export function SimulationResult() {
  let response = localStorage.getItem("simulationResult");

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
  let mainPlayerJob = mainPlayerSimulationData.job.valueOf();

  return (
    <Box className="SimulationResult">
      <SummaryBoardBox>
        <TitleBox borderRadius={4}>
          <Typography variant="h5" component="div">
            Summary
          </Typography>
        </TitleBox>

        {DpsSummary(mainPlayerSimulationData)}
        {PlayerInfo(mainPlayerJob)}
      </SummaryBoardBox>
    </Box>
  );
}
/*
      <Box className="BestTeammateGraph">
        <Typography variant="h5" component="div">
          Best Teammate
        </Typography>

        {BestTeammateGraph(responseJson)}
      </Box>
      <Box className="DamageProfileGraph">
        <Typography variant="h5" component="div">
          Damage Profile
        </Typography>
        {DamageProfileGraph(responseJson)}
      </Box>
      <Box className="RotationLog">
        <Typography variant="h5" component="div">
          Rotation Log
        </Typography>
        {SkillLogResult(responseJson)}
      </Box>
*/
