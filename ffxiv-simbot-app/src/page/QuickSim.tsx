import { useState } from "react";
import { Box, styled } from "@mui/material";
import { QUICKSIM_RESULT_URL, QuickSimInputSaveName } from "../App"
import { EquipmentSelectionMenu } from "../components/input/basicform/EquipmentInputForm";
import { StatPowerSummary } from "../components/container/StatSummary";
import { HorizontalPartyInput } from "../components/input/partyinput/HorizontalPartyInput";
import { EquipmentInput } from "../types/EquipmentInput";
import { defaultSingleEquipmentInput } from "../const/DefaultSingleEquipmentInput";
import { MENU_WIDTH_VW, LeftMenuWithLoadout } from "../components/container/LeftMenu";
import { ColorConfigurations } from "../Themes";
import { Footer } from "../components/basic/Footer";
import { AppHeader } from "../components/image/AppHeader";
import { SelectionTitle } from "../components/basic/SelectionTitle";
import { QuickSimBottomMenu } from "../components/container/BottomMenu";
import { CustomizeBoardStyle, EquipmentBoardStyle, InputContainerStyle } from "./Styles";
import { calculatePlayerPowerFromInputs } from "../types/ffxivdatabase/ItemSet";

let INPUT_CONTAINER_WIDTH = "40vw";
const QUICKSIM_LOADOUT_COUNT = 3;

let QuickSimInputContainer = styled(Box)`
  ${InputContainerStyle(INPUT_CONTAINER_WIDTH)} 
`;

let CustomizeBoard = styled(Box)`
  ${CustomizeBoardStyle}
`

let EquipmentBoard = styled(Box)`
  ${EquipmentBoardStyle}
`

export function isNotValid(input: EquipmentInput) {
  if (input.equipmentDatas === null) {
    return true;
  }

  return false;
}


export function QuickSim() {
  let mostRecentInputState = localStorage.getItem(QuickSimInputSaveName);
  let mostRecentInput = null;

  if (mostRecentInputState === null) {
    mostRecentInput = defaultSingleEquipmentInput();
  } else {
    mostRecentInput = JSON.parse(
      mostRecentInputState
    ) as EquipmentInput;
  }

  if (isNotValid(mostRecentInput)) {
    mostRecentInput = defaultSingleEquipmentInput();
  }
  let power = calculatePlayerPowerFromInputs(mostRecentInput.equipmentDatas[0]);
  mostRecentInput.equipmentDatas[0].power = power;

  const [totalState, setTotalState] = useState(
    mostRecentInput
  );

  let bodyWidth = 100 - MENU_WIDTH_VW;

  return (
    <>
      <Box display="flex" sx={{ backgroundColor: ColorConfigurations.backgroundOne }} width="100vw">
        {LeftMenuWithLoadout(QUICKSIM_LOADOUT_COUNT, QUICKSIM_RESULT_URL, totalState, setTotalState)}
        <Box width={`${bodyWidth}vw`}>
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
                {HorizontalPartyInput(
                  totalState,
                  setTotalState
                )}
              </CustomizeBoard>
            </QuickSimInputContainer>

            <QuickSimInputContainer marginTop={10}>
              {SelectionTitle("3. Specific Player Power")}
              <Box display="flex" justifyContent="center" paddingBottom={"20vh"}>
                {StatPowerSummary(totalState.equipmentDatas[0])}
              </Box>
            </QuickSimInputContainer>

            {QuickSimBottomMenu(totalState)}
          </Box>
          {Footer()}
        </Box>
      </Box >
    </>
  );
}
