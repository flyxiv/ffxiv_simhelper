import { useState } from "react";
import { Box, Button, styled } from "@mui/material";
import {
  GEAR_COMPARE_URL,
  GEAR_COMPARE_REQUEST_SAVE_NAME,
  BODY_WIDTH,
} from "../App";
import { EquipmentSelectionMenu } from "../components/input/basicform/EquipmentInputForm";
import { HorizontalPartyInput } from "../components/input/partyinput/HorizontalPartyInput";
import { LeftMenuWithLoadout } from "../components/container/LeftMenu";
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
import ArrowDownwardIcom from "@mui/icons-material/ArrowDownward";
import ArrowUpwardIcon from "@mui/icons-material/ArrowUpward";
import { AppLanguageTexts } from "../const/languageTexts";
import { isMobile } from "../util";

export const GEAR_COMPARE_INPUT_CONTAINER_WIDTH = "98%";
const GEAR_COMPARE_LOADOUNT_COUNT = 6;

const PARTY_INPUT_WIDTH = "90%";

let GearCompareEquipmentInputContainer = styled(Box)`
  ${InputContainerStyle},
  width: ${PARTY_INPUT_WIDTH}
`;

let GearComparePartyInputContainer = styled(Box)`
  ${InputContainerStyle},
  width: ${PARTY_INPUT_WIDTH}
`;

let CustomizeBoard = styled(Box)`
  ${CustomizeBoardStyle}
`;

let EquipmentBoard = styled(Box)`
  ${EquipmentBoardStyle}
`;

export function isNotValidGearCompare(input: EquipmentInput) {
  if (
    input.equipmentDatas === null ||
    input.equipmentDatas === undefined ||
    input.equipmentDatas.length !== 2
  ) {
    return true;
  }

  for (let i = 0; i < input.equipmentDatas.length; i++) {
    if (input.equipmentDatas[i].partyMemberIlvl === undefined) {
      return true;
    }

    if (input.equipmentDatas[i].usePot === undefined) {
      return true;
    }
    if (input.equipmentDatas[i].compositionBuffPercent === undefined) {
      return true;
    }
  }

  return false;
}

export function GearCompare() {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  let mostRecentInputState = localStorage.getItem(
    GEAR_COMPARE_REQUEST_SAVE_NAME
  );
  let mostRecentInput = null;

  if (mostRecentInputState === null) {
    mostRecentInput = defaultDoubleEquipmentInput();
  } else {
    mostRecentInput = JSON.parse(mostRecentInputState) as EquipmentInput;
  }

  if (isNotValidGearCompare(mostRecentInput)) {
    mostRecentInput = defaultDoubleEquipmentInput();
  }

  const [totalState, setTotalState] = useState(mostRecentInput);

  return (
    <>
      <Box
        display="flex"
        sx={{ backgroundColor: AppConfigurations.backgroundOne }}
        width="100%"
      >
        {LeftMenuWithLoadout(
          GEAR_COMPARE_LOADOUNT_COUNT,
          GEAR_COMPARE_URL,
          LANGUAGE_TEXTS.GEAR_COMPARE_PAGE_NAME,
          totalState,
          setTotalState,
          LANGUAGE_TEXTS
        )}
        <Box width={BODY_WIDTH}>
          {AppHeader()}
          <Box display="flex" flexDirection="column" justifyContent={"center"}>
            <GearCompareEquipmentInputContainer>
              <Box display="flex" justifyContent={"center"} width={"100%"}>
                {SelectionTitle(LANGUAGE_TEXTS.GEAR_COMPARE_INPUT_INFO_TEXT)}
              </Box>
              <Box
                display="flex"
                justifyContent="space-evenly"
                width={GEAR_COMPARE_INPUT_CONTAINER_WIDTH}
                padding="1%"
                flexDirection={isMobile() ? "column" : "row"}
              >
                <EquipmentBoard>
                  {EquipmentSelectionMenu(
                    0,
                    totalState,
                    setTotalState,
                    LANGUAGE_TEXTS,
                    true,
                    true
                  )}
                </EquipmentBoard>

                {isMobile() ? LoadUpBottonButton(totalState, setTotalState) : LoadLeftRightButton(totalState, setTotalState)}


                <EquipmentBoard>
                  {EquipmentSelectionMenu(
                    1,
                    totalState,
                    setTotalState,
                    LANGUAGE_TEXTS,
                    true,
                    true
                  )}
                </EquipmentBoard>
              </Box>
            </GearCompareEquipmentInputContainer>

            <GearComparePartyInputContainer paddingTop={10} paddingBottom={40}>
              {SelectionTitle(LANGUAGE_TEXTS.DPS_ANALYSIS_PARTY_INPUT_INFO_TEXT)}
              <CustomizeBoard>
                {HorizontalPartyInput(totalState, setTotalState, LANGUAGE_TEXTS)}
              </CustomizeBoard>
            </GearComparePartyInputContainer>

            {GearCompareBottomMenu(totalState, LANGUAGE_TEXTS)}
          </Box>
          {Footer()}
        </Box>
      </Box>
    </>
  );
}

function LoadLeftRightButton(totalState: EquipmentInput, setTotalState: Function) {
  return <Box
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
}

function LoadUpBottonButton(totalState: EquipmentInput, setTotalState: Function) {
  return <Box
    display={"flex"}
    flexDirection={"row"}
    justifyContent={"center"}
    marginY={1}
  >
    <Box marginRight={2}>
      {LoadTopEquipmentToBottomButton(totalState, setTotalState)}
    </Box>
    <Box marginLeft={2}>
      {LoadBottomEquipmentToTopButton(totalState, setTotalState)}
    </Box>
  </Box>
}


function LoadLeftEquipmentToRightButton(
  totalState: EquipmentInput,
  setTotalState: Function
) {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  return (
    <Button
      variant="contained"
      endIcon={<ArrowForwardIcon />}
      onClick={() => {
        let newTotalState = { ...totalState };
        newTotalState.equipmentDatas[1] = JSON.parse(
          JSON.stringify(totalState.equipmentDatas[0])
        );
        setTotalState(newTotalState);
      }}
      sx={{ fontSize: AppConfigurations.body2FontSize }}
    >
      {LANGUAGE_TEXTS.COPY_BUTTON_TEXT}
    </Button>
  );
}


function LoadTopEquipmentToBottomButton(
  totalState: EquipmentInput,
  setTotalState: Function
) {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  return (
    <Button
      variant="contained"
      endIcon={<ArrowDownwardIcom />}
      onClick={() => {
        let newTotalState = { ...totalState };
        newTotalState.equipmentDatas[1] = JSON.parse(
          JSON.stringify(totalState.equipmentDatas[0])
        );
        setTotalState(newTotalState);
      }}
      sx={{ fontSize: AppConfigurations.body2FontSize }}
    >
      {LANGUAGE_TEXTS.COPY_BUTTON_TEXT}
    </Button>
  );
}

function LoadRightEquipmentToLeftButton(
  totalState: EquipmentInput,
  setTotalState: Function
) {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  return (
    <Button
      variant="contained"
      startIcon={<ArrowBackIcon />}
      onClick={() => {
        let newTotalState = { ...totalState };
        newTotalState.equipmentDatas[0] = JSON.parse(
          JSON.stringify(totalState.equipmentDatas[1])
        );
        setTotalState(newTotalState);
      }}
      sx={{ fontSize: AppConfigurations.body2FontSize }}
    >
      {LANGUAGE_TEXTS.COPY_BUTTON_TEXT}
    </Button>
  );
}


function LoadBottomEquipmentToTopButton(
  totalState: EquipmentInput,
  setTotalState: Function
) {
  let LANGUAGE_TEXTS = AppLanguageTexts();

  return (
    <Button
      variant="contained"
      endIcon={<ArrowUpwardIcon />}
      onClick={() => {
        let newTotalState = { ...totalState };
        newTotalState.equipmentDatas[1] = JSON.parse(
          JSON.stringify(totalState.equipmentDatas[0])
        );
        setTotalState(newTotalState);
      }}
      sx={{ fontSize: AppConfigurations.body2FontSize }}
    >
      {LANGUAGE_TEXTS.COPY_BUTTON_TEXT}
    </Button>
  );
}
