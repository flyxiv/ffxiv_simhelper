import { useState } from "react";
import { Box, styled } from "@mui/material";
import {
  BODY_WIDTH,
  STAT_WEIGHTS_REQUEST_SAVE_NAME,
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
} from "../const/languageTexts";

let INPUT_CONTAINER_WIDTH = "70%";
const STATWEIGHTS_LOADOUT_COUNT = 6;

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
  if (input.equipmentDatas === null || input.equipmentDatas === undefined) {
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
  }

  return false;
}

export function StatWeights() {
  let mostRecentInputState = localStorage.getItem(
    STAT_WEIGHTS_REQUEST_SAVE_NAME
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
          STATWEIGHTS_LOADOUT_COUNT,
          STAT_WEIGHTS_URL,
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
