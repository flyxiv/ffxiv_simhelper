import { QuickSimResponse } from "src/types/QuickSimResponse";
import { SkillLogTable } from "./SkillLogTable";
import { Box } from "@mui/material";
import "./SkillLogTable.css";

export const SkillLogResult = (response: QuickSimResponse) => {
  const mainPlayerId = response.mainPlayerId;
  const simulationDatas = response.simulationData;

  let mainPlayerRotationLog = null;

  let i = 0;
  for (i = 0; i < simulationDatas.length; i++) {
    if (simulationDatas[i].playerId === mainPlayerId) {
      mainPlayerRotationLog = simulationDatas[i].rotationLog;
      break;
    }
  }

  if (mainPlayerRotationLog === null) {
    return (
      <div>
        <p>
          Simulation Result is not available: main player rotation data is null.
        </p>
      </div>
    );
  }

  return (
    <Box className="SkillLogTableContainer">
      <SkillLogTable skilllog={mainPlayerRotationLog} />
    </Box>
  );
};
