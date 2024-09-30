import { useState } from "react";
import { Box, styled } from "@mui/material";
import {
  BODY_WIDTH,
  QUICKSIM_URL,
  SINGLE_INPUT_SAVE_NAME,
  STAT_WEIGHTS_URL,
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
import { StatWeightsRequestButton } from "../components/basic/StatWeightsRequestButton";
import {
  PLAYER_POWER_TEXT,
  QUICK_SIM_INPUT_INFO_TEXT,
  QUICK_SIM_PARTY_INPUT_INFO_TEXT,
  QUICKSIM_PAGE_NAME,
} from "../const/languageTexts";
import { isNotValid, QUICKSIM_LOADOUT_COUNT } from "./QuickSim";

let INPUT_CONTAINER_WIDTH = "70%";

let StatWeightsInputContainer = styled(Box)`
  ${InputContainerStyle(INPUT_CONTAINER_WIDTH)}
`;

let CustomizeBoard = styled(Box)`
  ${CustomizeBoardStyle}
`;

let EquipmentBoard = styled(Box)`
  ${EquipmentBoardStyle}
`;

export function StatWeights() {
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
      >
        {LeftMenuWithLoadout(
          QUICKSIM_LOADOUT_COUNT,
          QUICKSIM_URL,
          QUICKSIM_PAGE_NAME,
          totalState,
          setTotalState
        )}
        <Box width={BODY_WIDTH}>
          {AppHeader()}
          <Box alignContent={"center"}>
            <StatWeightsInputContainer justifyContent={"center"}>
              {SelectionTitle(QUICK_SIM_INPUT_INFO_TEXT)}
              <EquipmentBoard>
                {EquipmentSelectionMenu(0, totalState, setTotalState)}
              </EquipmentBoard>
            </StatWeightsInputContainer>

            <StatWeightsInputContainer paddingTop={20}>
              {SelectionTitle(QUICK_SIM_PARTY_INPUT_INFO_TEXT)}
              <CustomizeBoard>
                {HorizontalPartyInput(totalState, setTotalState)}
              </CustomizeBoard>
            </StatWeightsInputContainer>

            <StatWeightsInputContainer marginTop={10}>
              {SelectionTitle(`3. ${PLAYER_POWER_TEXT}`)}
              <Box
                display="flex"
                justifyContent="center"
                paddingBottom={"30vh"}
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
