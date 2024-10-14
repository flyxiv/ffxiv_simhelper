import { useState } from "react";
import { Box, styled } from "@mui/material";
import {
  BODY_WIDTH,
  DPS_ANALYSIS_URL,
  SINGLE_INPUT_SAVE_NAME,
} from "../App";
import { EquipmentSelectionMenu } from "../components/input/basicform/EquipmentInputForm";
import { StatPowerSummary } from "../components/container/StatSummary";
import { HorizontalPartyInput } from "../components/input/partyinput/HorizontalPartyInput";
import { EquipmentInput } from "../types/EquipmentInput";
import { defaultSingleEquipmentInput } from "../const/DefaultSingleEquipmentInput";
import { LeftMenuWithLoadout } from "../components/container/LeftMenu";
import { AppConfigurations } from "../Themes";
import { Footer } from "../components/basic/Footer";
import { AppHeader } from "../components/image/AppHeader";
import { SelectionTitle } from "../components/basic/SelectionTitle";
import { BasicBottomMenu } from "../components/container/BottomMenu";
import {
  CustomizeBoardStyle,
  EquipmentBoardStyle,
  InputContainerStyle,
} from "./Styles";
import { BestStatsRequestButton } from "../components/basic/BestStatsRequestButton";
import { isNotValid, DPS_ANALYSIS_LOADOUT_COUNT } from "./DpsAnalysis";
import { AppLanguageTexts } from "../const/languageTexts";

let INPUT_CONTAINER_WIDTH = "70%";

let StatWeightsInputContainer = styled(Box)`
  ${InputContainerStyle},
  width: ${INPUT_CONTAINER_WIDTH}
`;

let CustomizeBoard = styled(Box)`
  ${CustomizeBoardStyle}
`;

let EquipmentBoard = styled(Box)`
  ${EquipmentBoardStyle}
`;

export function BestStats() {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  let mostRecentInputState = localStorage.getItem(
    SINGLE_INPUT_SAVE_NAME
  );
  let mostRecentInput = null;

  if (mostRecentInputState === null) {
    mostRecentInput = defaultSingleEquipmentInput();
  } else {
    mostRecentInput = JSON.parse(mostRecentInputState) as EquipmentInput;
  }

  if (isNotValid(mostRecentInput)) {
    mostRecentInput = defaultSingleEquipmentInput();
  }

  const [totalState, setTotalState] = useState(mostRecentInput);

  return (
    <>
      <Box
        display="flex"
        sx={{ backgroundColor: AppConfigurations.backgroundOne }}
        width="100vw"
        overflow={"auto"}
      >
        {LeftMenuWithLoadout(
          DPS_ANALYSIS_LOADOUT_COUNT,
          DPS_ANALYSIS_URL,
          LANGUAGE_TEXTS.BEST_STAT_PAGE_NAME,
          totalState,
          setTotalState,
          LANGUAGE_TEXTS
        )}
        <Box width={BODY_WIDTH}>
          {AppHeader()}
          <Box alignContent={"center"}>
            <StatWeightsInputContainer justifyContent={"center"}>
              {SelectionTitle(LANGUAGE_TEXTS.DPS_ANALYSIS_INPUT_INFO_TEXT)}
              <EquipmentBoard>
                {EquipmentSelectionMenu(0, totalState, setTotalState, LANGUAGE_TEXTS)}
              </EquipmentBoard>
            </StatWeightsInputContainer>

            <StatWeightsInputContainer paddingTop={20}>
              {SelectionTitle(LANGUAGE_TEXTS.DPS_ANALYSIS_PARTY_INPUT_INFO_TEXT)}
              <CustomizeBoard>
                {HorizontalPartyInput(totalState, setTotalState, LANGUAGE_TEXTS)}
              </CustomizeBoard>
            </StatWeightsInputContainer>

            <StatWeightsInputContainer marginTop={10}>
              {SelectionTitle(`3. ${LANGUAGE_TEXTS.PLAYER_POWER_TEXT}`)}
              <Box
                display="flex"
                justifyContent="center"
                paddingBottom={"30vh"}
              >
                {StatPowerSummary(totalState.equipmentDatas[0], LANGUAGE_TEXTS)}
              </Box>
            </StatWeightsInputContainer>

            {BasicBottomMenu(totalState, BestStatsRequestButton, LANGUAGE_TEXTS, true)}
          </Box>
          {Footer()}
        </Box>
      </Box >
    </>
  );
}
