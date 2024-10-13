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
import { DpsAnalysisRequestButton } from "../components/basic/DpsAnalysisRequestButton";
import { AppLanguageTexts } from "../const/languageTexts";

export const DPS_ANALYSIS_LOADOUT_COUNT = 6;

let DpsAnalysisInputContainer = styled(Box)`
  ${InputContainerStyle},
`;

let CustomizeBoard = styled(Box)`
  ${CustomizeBoardStyle}
`;

let EquipmentBoard = styled(Box)`
  ${EquipmentBoardStyle}
`;

export function isNotValid(input: EquipmentInput) {
  if (input.equipmentDatas === null || input.equipmentDatas === undefined) {
    return true;
  }

  if (input.equipmentDatas.length !== 1) {
    return true;
  }

  for (let i = 0; i < input.equipmentDatas.length; i++) {
    if (input.equipmentDatas[i].partyMemberIlvl === undefined) {
      return true;
    }

    if (input.equipmentDatas[i].mainPlayerPartner1Id === undefined) {
      return true;
    }

    if (input.equipmentDatas[i].mainPlayerPartner2Id === undefined) {
      return true;
    }


    if (input.equipmentDatas[i].usePot === undefined) {
      return true;
    }

    if (input.equipmentDatas[i].partyMemberJobAbbrevs.length === 0) {
      return true;
    }

    if (input.equipmentDatas[i].compositionBuffPercent === undefined) {
      return true;
    }
  }

  return false;
}

export function DpsAnalysis() {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  let mostRecentInputState = localStorage.getItem(SINGLE_INPUT_SAVE_NAME);
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
        overflow={"auto"}
        width={"100%"}
      >
        {LeftMenuWithLoadout(
          DPS_ANALYSIS_LOADOUT_COUNT,
          DPS_ANALYSIS_URL,
          LANGUAGE_TEXTS.DPS_ANALYSIS_PAGE_NAME,
          totalState,
          setTotalState,
          LANGUAGE_TEXTS
        )}
        <Box width={BODY_WIDTH} display="flex" alignItems={"center"} flexDirection={"column"}>
          {AppHeader()}
          <Box alignContent={"center"} width="100%">
            <DpsAnalysisInputContainer justifyContent={"center"}>
              {SelectionTitle(LANGUAGE_TEXTS.DPS_ANALYSIS_INPUT_INFO_TEXT)}
              <EquipmentBoard width="100%">
                {EquipmentSelectionMenu(0, totalState, setTotalState, LANGUAGE_TEXTS)}
              </EquipmentBoard>
            </DpsAnalysisInputContainer>

            <DpsAnalysisInputContainer>
              {SelectionTitle(LANGUAGE_TEXTS.DPS_ANALYSIS_PARTY_INPUT_INFO_TEXT)}
              <CustomizeBoard>
                {HorizontalPartyInput(totalState, setTotalState, LANGUAGE_TEXTS)}
              </CustomizeBoard>
            </DpsAnalysisInputContainer>

            <DpsAnalysisInputContainer marginTop={10}>
              {SelectionTitle(`3. ${LANGUAGE_TEXTS.PLAYER_POWER_TEXT}`)}
              <Box
                display="flex"
                justifyContent="center"
                paddingBottom={"20vh"}
              >
                {StatPowerSummary(totalState.equipmentDatas[0], LANGUAGE_TEXTS)}
              </Box>
            </DpsAnalysisInputContainer>

            {BasicBottomMenu(totalState, DpsAnalysisRequestButton, LANGUAGE_TEXTS)}
          </Box>
          {Footer()}
        </Box>
      </Box>
    </>
  );
}
