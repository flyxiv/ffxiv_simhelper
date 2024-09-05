import { useState } from "react";
import { Box, styled } from "@mui/material";
import { BODY_WIDTH, QUICKSIM_RESULT_URL, SINGLE_INPUT_SAVE_NAME } from "../App";
import { EquipmentSelectionMenu } from "../components/input/basicform/EquipmentInputForm";
import { StatPowerSummary } from "../components/container/StatSummary";
import { HorizontalPartyInput } from "../components/input/partyinput/HorizontalPartyInput";
import { EquipmentInput } from "../types/EquipmentInput";
import { defaultSingleEquipmentInput } from "../const/DefaultSingleEquipmentInput";
import {
  LeftMenuWithLoadout,
} from "../components/container/LeftMenu";
import { ColorConfigurations } from "../Themes";
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

let INPUT_CONTAINER_WIDTH = "50vw";
const QUICKSIM_LOADOUT_COUNT = 3;

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
  if (input.equipmentDatas === null) {
    return true;
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
        sx={{ backgroundColor: ColorConfigurations.backgroundOne }}
        width="100vw"
      >
        {LeftMenuWithLoadout(
          QUICKSIM_LOADOUT_COUNT,
          QUICKSIM_RESULT_URL,
          totalState,
          setTotalState
        )}
        <Box width={BODY_WIDTH}>
          {AppHeader()}
          <Box alignContent={"center"}>
            <QuickSimInputContainer justifyContent={"center"}>
              {SelectionTitle("1. Input Your Info")}
              <EquipmentBoard>
                {EquipmentSelectionMenu(0, totalState, setTotalState)}
              </EquipmentBoard>
            </QuickSimInputContainer>

            <QuickSimInputContainer paddingTop={20}>
              {SelectionTitle("2. Additional Settings")}
              <CustomizeBoard>
                {HorizontalPartyInput(totalState, setTotalState)}
              </CustomizeBoard>
            </QuickSimInputContainer>

            <QuickSimInputContainer marginTop={10}>
              {SelectionTitle("3. Specific Player Power")}
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
