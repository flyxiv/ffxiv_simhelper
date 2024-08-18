import { QuickSimResponse } from "src/types/QuickSimResponse";
import {
  SkillIconBoxTitleStyle as SkillIconTitleBoxStyle,
  SkillLogCombatTimeTitleBoxStyle,
  SkillLogTableStyle,
  StatusIconTitleBoxStyle,
  SkillLogRowStyle,
} from "./Styles";
import { SkillLogTable } from "./SkillLogTable";
import { styled, List, Box, Typography } from "@mui/material";

const SkillLogTableList = styled(List)`
  ${SkillLogTableStyle}
`;

const SkillLogRowBox = styled(Box)`
  ${SkillLogRowStyle}
`;

const SkillLogCombatTimeTitleBox = styled(Box)`
  ${SkillLogCombatTimeTitleBoxStyle}
`;

const SkillIconTitleBox = styled(Box)`
  ${SkillIconTitleBoxStyle}
`;

const StatusIconTitleBox = styled(Box)`
  ${StatusIconTitleBoxStyle("25vw")}
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
      <SkillLogRowBox>
        <SkillLogCombatTimeTitleBox>
          <Typography variant="body1" fontSize={fontSize}>
            Combat Time
          </Typography>
        </SkillLogCombatTimeTitleBox>
        <SkillIconTitleBox>
          <Typography variant="body1" fontSize={fontSize}>
            Ability
          </Typography>
        </SkillIconTitleBox>
        <StatusIconTitleBox>
          <Typography variant="body1" fontSize={fontSize}>
            Important Status
          </Typography>
        </StatusIconTitleBox>
      </SkillLogRowBox>
      {SkillLogTable(mainPlayerRotationLog)}
    </SkillLogTableList>
  );
};
