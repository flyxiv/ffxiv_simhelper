import { QuickSimResponse } from "../../types/QuickSimResponse";
import {
  SkillLogTableStyle,
  SkillLogCombatTimeBoxStyle,
  SkillIconBoxStyle,
  StatusIconBoxStyle,
  statusBoxWidth,
  SkillEntityBoxStyle,
} from "./Styles";
import { SkillLogTable } from "./SkillLogTable";
import { styled, List, Box, Typography, ListItem } from "@mui/material";

const SkillLogCombatTimeBox = styled(Box)`
  ${SkillLogCombatTimeBoxStyle}
`;

const SkillIconBox = styled(Box)`
  ${SkillIconBoxStyle}
`;

const SkillLogTableList = styled(List)`
  ${SkillLogTableStyle}
`;

const StatusBox = styled(Box)`
    ${StatusIconBoxStyle(statusBoxWidth)}
`;

const SkillEntityBox = styled(Box)`
  ${SkillEntityBoxStyle}
`;


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
  let fontSize = 10;

  return (
    <SkillLogTableList>
      <ListItem>
        <SkillEntityBox>
          <SkillLogCombatTimeBox>
            <Typography variant="body1" fontSize={fontSize}>
              Combat Time
            </Typography>
          </SkillLogCombatTimeBox>

          <SkillIconBox>
            <Typography variant="body1" fontSize={fontSize}>
              Ability
            </Typography>
          </SkillIconBox>

          <StatusBox>
            <Typography variant="body1" fontSize={fontSize}>
              Important Status
            </Typography>
          </StatusBox>

        </SkillEntityBox>
      </ListItem>
      {SkillLogTable(mainPlayerRotationLog)}
    </SkillLogTableList >
  );
};
