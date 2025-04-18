import { DpsAnalysisResponse } from "../../types/DpsAnalysisResponse";
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
import { AppConfigurations } from "../../Themes";
import { WarningText } from "../basic/WarningText";
import { MNK_EN_NAME, SAM_EN_NAME, TextDictionary } from "../../const/languageTexts";

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

export const SkillLogResult = (response: DpsAnalysisResponse, LANGUAGE_TEXTS: TextDictionary) => {
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
    <>
      <Typography
        sx={{ fontSize: AppConfigurations.body2FontSize, color: "white" }}
      >
        {LANGUAGE_TEXTS.ROTATION_SAMPLE_WARNING_TEXT}
      </Typography>
      {
        response.mainPlayerJobAbbrev === SAM_EN_NAME ?
          WarningText(LANGUAGE_TEXTS.SAMURAI_ROTATION_WARNING_TEXT)
          : response.mainPlayerJobAbbrev === MNK_EN_NAME ? WarningText(LANGUAGE_TEXTS.MNK_ROTATION_WARNING_TEXT) : <Box></Box>
      }
      <SkillLogTableList sx={{ width: { xs: "100%", sm: "100%" }, overflow: "auto" }}>
        <ListItem>
          <SkillEntityBox>
            <SkillLogCombatTimeBox>
              <Typography
                variant="body1"
                fontSize={AppConfigurations.body2FontSize}
              >
                {LANGUAGE_TEXTS.COMBAT_TIME_TEXT}
              </Typography>
            </SkillLogCombatTimeBox>

            <SkillIconBox>
              <Typography
                variant="body1"
                fontSize={AppConfigurations.body2FontSize}
              >
                {LANGUAGE_TEXTS.ABILITY_TEXT}
              </Typography>
            </SkillIconBox>

            <StatusBox>
              <Typography
                variant="body1"
                fontSize={AppConfigurations.body2FontSize}
              >
                {LANGUAGE_TEXTS.IMPORTANT_STATUS_TEXT}
              </Typography>
            </StatusBox>
          </SkillEntityBox>
        </ListItem>
        {SkillLogTable(mainPlayerRotationLog)}
      </SkillLogTableList >
    </>
  );
};
