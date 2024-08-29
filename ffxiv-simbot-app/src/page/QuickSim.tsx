import { useState } from "react";
import { QuickSimRequestButton } from "../components/basic/QuickSimRequestButton";
import { Box, Typography } from "@mui/material";
import "./QuickSim.css";
import { QuickSimInputSaveName } from "../App"
import { EquipmentSelectionMenu } from "../components/input/basicform/EquipmentInputForm";
import { StatPowerSummary, StatSummary } from "../components/container/StatSummary";
import { HorizontalPartyInput } from "../components/input/partyinput/HorizontalPartyInput";
import { SingleEquipmentInputSaveState } from "../types/SingleEquipmentInputSaveState";
import { defaultSingleEquipmentInput } from "../const/DefaultSingleEquipmentInput";
import { MENU_WIDTH_VW, QuicksimLeftMenu } from "../components/container/LeftMenu";
import { ColorConfigurations } from "../Themes";
import { Footer } from "../components/basic/Footer";
import { AppHeader } from "../components/image/AppHeader";
import { SelectionTitle } from "../components/basic/SelectionTitle";
import { QuickSimBottomMenu } from "../components/container/BottomMenu";

export function isNotValid(input: SingleEquipmentInputSaveState) {
  if (input.mainPlayerJobAbbrev === null || input.mainPlayerJobAbbrev === undefined) {
    return true;
  }

  if (
    input.combatTimeMillisecond === null ||
    input.combatTimeMillisecond === undefined
  ) {
    return true;
  }

  if (
    input.combatTimeMillisecond === null ||
    input.combatTimeMillisecond === undefined
  ) {
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
    ) as SingleEquipmentInputSaveState;
  }

  if (isNotValid(mostRecentInput)) {
    mostRecentInput = defaultSingleEquipmentInput();
  }

  const [totalState, setTotalState] = useState(
    mostRecentInput
  );

  let bodyWidth = 100 - MENU_WIDTH_VW;

  return (
    <>
      <Box display="flex" sx={{ backgroundColor: ColorConfigurations.backgroundOne }} width="100vw">
        {QuicksimLeftMenu(totalState, setTotalState)}
        <Box width={`${bodyWidth}vw`}>
          {AppHeader()}
          <Box alignContent={"center"}>
            <Box className="QuickSimInputContainer" justifyContent={"center"}>
              {SelectionTitle("1. Input Your Info")}
              <Box className="EquipmentBoard">
                {EquipmentSelectionMenu(0, totalState, setTotalState)}
              </Box>
            </Box>

            <Box>
              {StatSummary(totalState)}
            </Box>

            <Box className="QuickSimInputContainer">
              {SelectionTitle("2. Additional Settings")}
              <Box className="CustomizeBoard">
                {HorizontalPartyInput(
                  totalState,
                  setTotalState
                )}
              </Box>
            </Box>

            <Box className="QuickSimInputContainer">
              {SelectionTitle("3. Specific Player Power")}
              <Box display="flex" justifyContent="center">
                {StatPowerSummary(totalState)}
              </Box>
            </Box>

            <Box display="flex" justifyContent="center">
              {QuickSimRequestButton(
                totalState
              )}
            </Box>
            {QuickSimBottomMenu()}
          </Box>
          {Footer()}
        </Box>
      </Box >
    </>
  );
}
