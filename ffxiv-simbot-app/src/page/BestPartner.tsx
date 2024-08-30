import { useState } from "react";
import { Box, styled } from "@mui/material";
import { BEST_PARTNER_INPUT_SAVE_NAME, BEST_PARTNER_URL } from "../App"
import { EquipmentSelectionMenu } from "../components/input/basicform/EquipmentInputForm";
import { StatPowerSummary } from "../components/container/StatSummary";
import { EquipmentInput } from "../types/EquipmentInput";
import { defaultBestPartnerEquipmentInput } from "../const/DefaultSingleEquipmentInput";
import { MENU_WIDTH_VW, LeftMenuWithLoadout } from "../components/container/LeftMenu";
import { ColorConfigurations } from "../Themes";
import { Footer } from "../components/basic/Footer";
import { AppHeader } from "../components/image/AppHeader";
import { SelectionTitle } from "../components/basic/SelectionTitle";
import { BasicBottomMenu } from "../components/container/BottomMenu";
import { EquipmentBoardStyle, InputContainerStyle } from "./Styles";

let INPUT_CONTAINER_WIDTH = "40vw";
const BEST_PARTNER_LOADOUT_COUNT = 3;

let StatWeightsInputContainer = styled(Box)`
  ${InputContainerStyle(INPUT_CONTAINER_WIDTH)} 
`;

let EquipmentBoard = styled(Box)`
  ${EquipmentBoardStyle}
`

export function isNotValid(input: EquipmentInput) {
  if (input.equipmentDatas === null) {
    return true;
  }

  return false;
}


export function BestPartner() {
  let mostRecentInputState = localStorage.getItem(BEST_PARTNER_INPUT_SAVE_NAME);
  let mostRecentInput = null;

  if (mostRecentInputState === null) {
    mostRecentInput = defaultBestPartnerEquipmentInput();
  } else {
    mostRecentInput = JSON.parse(
      mostRecentInputState
    ) as EquipmentInput;
  }

  if (isNotValid(mostRecentInput)) {
    mostRecentInput = defaultBestPartnerEquipmentInput();
  }

  const [totalState, setTotalState] = useState(
    mostRecentInput
  );

  let bodyWidth = 100 - MENU_WIDTH_VW;

  return (
    <>
      <Box display="flex" sx={{ backgroundColor: ColorConfigurations.backgroundOne }} width="100vw">
        {LeftMenuWithLoadout(BEST_PARTNER_LOADOUT_COUNT, BEST_PARTNER_URL, totalState, setTotalState)}
        <Box width={`${bodyWidth}vw`}>
          {AppHeader()}
          <Box alignContent={"center"}>
            <StatWeightsInputContainer justifyContent={"center"}>
              {SelectionTitle("1. Input Your Info")}
              <EquipmentBoard>
                {EquipmentSelectionMenu(0, totalState, setTotalState, true, true)}
              </EquipmentBoard>
            </StatWeightsInputContainer>

            <StatWeightsInputContainer marginTop={10}>
              {SelectionTitle("2. Specific Player Power")}
              <Box display="flex" justifyContent="center" paddingBottom={"20vh"}>
                {StatPowerSummary(totalState.equipmentDatas[0])}
              </Box>
            </StatWeightsInputContainer>

            {BasicBottomMenu(totalState)}
          </Box>
          {Footer()}
        </Box>
      </Box >
    </>
  );
}
