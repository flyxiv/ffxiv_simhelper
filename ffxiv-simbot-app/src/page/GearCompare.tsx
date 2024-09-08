import { useState } from "react";
import { Box, Button, styled } from "@mui/material";
import { GEAR_COMPARE_URL, GEAR_COMPARE_REQUEST_SAVE_NAME, BODY_WIDTH } from "../App";
import { EquipmentSelectionMenu } from "../components/input/basicform/EquipmentInputForm";
import { HorizontalPartyInput } from "../components/input/partyinput/HorizontalPartyInput";
import {
  LeftMenuWithLoadout,
} from "../components/container/LeftMenu";
import { AppConfigurations } from "../Themes";
import { Footer } from "../components/basic/Footer";
import { AppHeader } from "../components/image/AppHeader";
import { SelectionTitle } from "../components/basic/SelectionTitle";
import { GearCompareBottomMenu } from "../components/container/BottomMenu";
import {
  CustomizeBoardStyle,
  EquipmentBoardStyle,
  InputContainerStyle,
} from "./Styles";
import { defaultDoubleEquipmentInput } from "../const/DefaultDoubleEquipmentInput";
import { EquipmentInput } from "../types/EquipmentInput";
import ArrowBackIcon from "@mui/icons-material/ArrowBack";
import ArrowForwardIcon from "@mui/icons-material/ArrowForward";
import { COPY_BUTTON_TEXT, GEAR_COMPARE_INPUT_INFO_TEXT, QUICK_SIM_PARTY_INPUT_INFO_TEXT } from "../const/languageTexts";

export const GEAR_COMPARE_INPUT_CONTAINER_WIDTH = "70vw";
const GEAR_COMPARE_LOADOUNT_COUNT = 6;

const PARTY_INPUT_WIDTH = "40vw";

let GearCompareEquipmentInputContainer = styled(Box)`
  ${InputContainerStyle(GEAR_COMPARE_INPUT_CONTAINER_WIDTH)}
`;
let GearComparePartyInputContainer = styled(Box)`
  ${InputContainerStyle(PARTY_INPUT_WIDTH)}
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

export function GearCompare() {
  let mostRecentInputState = localStorage.getItem(
    GEAR_COMPARE_REQUEST_SAVE_NAME
  );
  let mostRecentInput = null;

  if (mostRecentInputState === null) {
    mostRecentInput = defaultDoubleEquipmentInput();
  } else {
    mostRecentInput = JSON.parse(mostRecentInputState) as EquipmentInput;
  }

  if (isNotValid(mostRecentInput)) {
    mostRecentInput = defaultDoubleEquipmentInput();
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
          GEAR_COMPARE_LOADOUNT_COUNT,
          GEAR_COMPARE_URL,
          totalState,
          setTotalState
        )}
        <Box width={BODY_WIDTH}>
          {AppHeader()}
          <Box alignContent={"center"}>
            <GearCompareEquipmentInputContainer>
              {SelectionTitle(GEAR_COMPARE_INPUT_INFO_TEXT)}
              <Box
                display="flex"
                justifyContent="space-evenly"
                width={GEAR_COMPARE_INPUT_CONTAINER_WIDTH}
              >
                <EquipmentBoard>
                  {EquipmentSelectionMenu(0, totalState, setTotalState)}
                </EquipmentBoard>

                <Box
                  display={"flex"}
                  flexDirection={"column"}
                  justifyContent={"center"}
                  marginX={2}
                >
                  <Box marginBottom={5}>
                    {LoadLeftEquipmentToRightButton(totalState, setTotalState)}
                  </Box>
                  <Box marginTop={5}>
                    {LoadRightEquipmentToLeftButton(totalState, setTotalState)}
                  </Box>
                </Box>

                <EquipmentBoard>
                  {EquipmentSelectionMenu(1, totalState, setTotalState)}
                </EquipmentBoard>
              </Box>
            </GearCompareEquipmentInputContainer>

            <GearComparePartyInputContainer paddingTop={20} paddingBottom={40}>
              {SelectionTitle(QUICK_SIM_PARTY_INPUT_INFO_TEXT)}
              <CustomizeBoard>
                {HorizontalPartyInput(totalState, setTotalState)}
              </CustomizeBoard>
            </GearComparePartyInputContainer>

            {GearCompareBottomMenu(totalState)}
          </Box>
          {Footer()}
        </Box>
      </Box>
    </>
  );
}

function LoadLeftEquipmentToRightButton(
  totalState: EquipmentInput,
  setTotalState: Function
) {
  return (
    <Button
      variant="contained"
      endIcon={<ArrowForwardIcon />}
      onClick={() => {
        let newTotalState = { ...totalState };
        newTotalState.equipmentDatas[1] = JSON.parse(JSON.stringify(totalState.equipmentDatas[0]));
        setTotalState(newTotalState);
      }}
    >
      {COPY_BUTTON_TEXT}
    </Button>
  );
}

function LoadRightEquipmentToLeftButton(
  totalState: EquipmentInput,
  setTotalState: Function
) {
  return (
    <Button
      variant="contained"
      startIcon={<ArrowBackIcon />}
      onClick={() => {
        let newTotalState = { ...totalState };
        newTotalState.equipmentDatas[0] = JSON.parse(JSON.stringify(totalState.equipmentDatas[1]));
        setTotalState(newTotalState);
      }}
    >
      {COPY_BUTTON_TEXT}
    </Button>
  );
}
