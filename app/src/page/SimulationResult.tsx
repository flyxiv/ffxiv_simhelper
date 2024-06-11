import { Summary } from "src/components/container/Summary";
import { JobIconFactory } from "../components/jobicon/JobIconFactory";
import { Box, Typography } from "@mui/material";
import "./SimulationResult.css";
import { QuickSimResponse } from "../types/QuickSimResponse";
import { BestTeammateGraph } from "../components/graph/BestTeammateGraph";
import { DamageProfileGraph } from "../components/graph/DamageProfileGraph";
import { SkillLogResult } from "../components/container/SkillLog";

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
  let mainPlayerJob = mainPlayerSimulationData.job;

  return (
    <Box className="SimulationResult">
      <Box className="Summary">
        <Typography variant="h5" component="div">
          Summary
        </Typography>

        <Box className="PlayerInfo">
          {JobIconFactory(mainPlayerJob)}
          <Typography variant="h5" component="div">
            {mainPlayerJob}
          </Typography>
        </Box>

        {Summary(mainPlayerSimulationData)}

        <p></p>
      </Box>
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
    </Box>
  );
}
