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
import { ABILITY_TEXT, COMBAT_TIME_TEXT, IMPORTANT_STATUS_TEXT } from "../../const/languageTexts";
import { AppConfigurations } from "../../Themes";

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
            <Typography variant="body1" fontSize={AppConfigurations.body2FontSize}>
              {COMBAT_TIME_TEXT}
            </Typography>
          </SkillLogCombatTimeBox>

          <SkillIconBox>
            <Typography variant="body1" fontSize={AppConfigurations.body2FontSize}>
              {ABILITY_TEXT}
            </Typography>
          </SkillIconBox>

          <StatusBox>
            <Typography variant="body1" fontSize={AppConfigurations.body2FontSize}>
              {IMPORTANT_STATUS_TEXT}
            </Typography>
          </StatusBox>

        </SkillEntityBox>
      </ListItem>
      {SkillLogTable(mainPlayerRotationLog)}
    </SkillLogTableList >
  );
};
