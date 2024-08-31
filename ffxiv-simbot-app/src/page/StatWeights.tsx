import { useState } from "react";
import { Box, styled } from "@mui/material";
import { SINGLE_INPUT_SAVE_NAME, STAT_WEIGHTS_URL } from "../App";
import { EquipmentSelectionMenu } from "../components/input/basicform/EquipmentInputForm";
import { StatPowerSummary } from "../components/container/StatSummary";
import { HorizontalPartyInput } from "../components/input/partyinput/HorizontalPartyInput";
import { EquipmentInput } from "../types/EquipmentInput";
import { defaultSingleEquipmentInput } from "../const/DefaultSingleEquipmentInput";
import {
  MENU_WIDTH_VW,
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
import { StatWeightsRequestButton } from "../components/basic/StatWeightsRequestButton";

let INPUT_CONTAINER_WIDTH = "40vw";
const STATWEIGHTS_LOADOUT_COUNT = 3;

let StatWeightsInputContainer = styled(Box)`
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

export function StatWeights() {
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

  let bodyWidth = 100 - MENU_WIDTH_VW;

  return (
    <>
      <Box
        display="flex"
        sx={{ backgroundColor: ColorConfigurations.backgroundOne }}
        width="100vw"
      >
        {LeftMenuWithLoadout(
          STATWEIGHTS_LOADOUT_COUNT,
          STAT_WEIGHTS_URL,
          totalState,
          setTotalState
        )}
        <Box width={`${bodyWidth}vw`}>
          {AppHeader()}
          <Box alignContent={"center"}>
            <StatWeightsInputContainer justifyContent={"center"}>
              {SelectionTitle("1. Input Your Info")}
              <EquipmentBoard>
                {EquipmentSelectionMenu(0, totalState, setTotalState)}
              </EquipmentBoard>
            </StatWeightsInputContainer>

            <StatWeightsInputContainer paddingTop={20}>
              {SelectionTitle("2. Additional Settings")}
              <CustomizeBoard>
                {HorizontalPartyInput(totalState, setTotalState)}
              </CustomizeBoard>
            </StatWeightsInputContainer>

            <StatWeightsInputContainer marginTop={10}>
              {SelectionTitle("3. Specific Player Power")}
              <Box
                display="flex"
                justifyContent="center"
                paddingBottom={"20vh"}
              >
                {StatPowerSummary(totalState.equipmentDatas[0])}
              </Box>
            </StatWeightsInputContainer>

            {BasicBottomMenu(totalState, StatWeightsRequestButton)}
          </Box>
          {Footer()}
        </Box>
      </Box>
    </>
  );
}
