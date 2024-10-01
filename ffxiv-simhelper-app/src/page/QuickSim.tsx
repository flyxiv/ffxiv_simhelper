import { useState } from "react";
import { Box, styled } from "@mui/material";
import {
  BODY_WIDTH,
  QUICKSIM_RESULT_URL,
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
import { QuickSimRequestButton } from "../components/basic/QuickSimRequestButton";
import {
  PLAYER_POWER_TEXT,
  QUICK_SIM_INPUT_INFO_TEXT,
  QUICK_SIM_PARTY_INPUT_INFO_TEXT,
  QUICKSIM_PAGE_NAME,
} from "../const/languageTexts";

export const INPUT_CONTAINER_WIDTH = "70%";
export const QUICKSIM_LOADOUT_COUNT = 6;

let QuickSimInputContainer = styled(Box)`
  ${InputContainerStyle(INPUT_CONTAINER_WIDTH)}
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

export function QuickSim() {
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
        width="100vw"
      >
        {LeftMenuWithLoadout(
          QUICKSIM_LOADOUT_COUNT,
          QUICKSIM_RESULT_URL,
          QUICKSIM_PAGE_NAME,
          totalState,
          setTotalState
        )}
        <Box width={BODY_WIDTH}>
          {AppHeader()}
          <Box alignContent={"center"}>
            <QuickSimInputContainer justifyContent={"center"}>
              {SelectionTitle(QUICK_SIM_INPUT_INFO_TEXT)}
              <EquipmentBoard>
                {EquipmentSelectionMenu(0, totalState, setTotalState)}
              </EquipmentBoard>
            </QuickSimInputContainer>

            <QuickSimInputContainer paddingTop={20}>
              {SelectionTitle(QUICK_SIM_PARTY_INPUT_INFO_TEXT)}
              <CustomizeBoard>
                {HorizontalPartyInput(totalState, setTotalState)}
              </CustomizeBoard>
            </QuickSimInputContainer>

            <QuickSimInputContainer marginTop={10}>
              {SelectionTitle(`3. ${PLAYER_POWER_TEXT}`)}
              <Box
                display="flex"
                justifyContent="center"
                paddingBottom={"20vh"}
              >
                {StatPowerSummary(totalState.equipmentDatas[0])}
              </Box>
            </QuickSimInputContainer>

            {BasicBottomMenu(totalState, QuickSimRequestButton)}
          </Box>
          {Footer()}
        </Box>
      </Box>
    </>
  );
}
