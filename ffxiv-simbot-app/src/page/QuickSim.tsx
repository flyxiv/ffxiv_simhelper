import { useState } from "react";
import { QuickSimRequestButton } from "../components/basic/QuickSimRequestButton";
import { Box, Typography } from "@mui/material";
import "./QuickSim.css";
import { QuickSimInputSaveName } from "../App"
import { updatePlayerPower } from "../types/ffxivdatabase/ItemSet";
import { EquipmentSelectionMenu } from "../components/input/basicform/EquipmentInputForm";
import { StatSummary } from "../components/container/StatSummary";
import { HorizontalPartyInput } from "../components/input/partyinput/HorizontalPartyInput";
import { SingleEquipmentInputSaveState } from "../types/SingleEquipmentInputSaveState";
import { defaultSingleEquipmentInput } from "../const/DefaultSingleEquipmentInput";
import { QuicksimLeftMenu } from "../components/container/LeftMenu";
import { ColorConfigurations } from "../Themes";
import { Footer } from "../components/basic/Footer";
import { AppHeader } from "../components/image/AppHeader";

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

  let borderRadius = 3;

  return (
    <>
      <Box display="flex" sx={{ backgroundColor: ColorConfigurations.backgroundOne }}>
        {QuicksimLeftMenu(totalState, setTotalState)}
        <Box>
          {AppHeader()}
          <Box alignContent={"center"}>
            <Box className="QuickSimInputContainer">
              <Box className="SelectionTitle" borderRadius={borderRadius}>
                <Typography variant="h5">1. Input Your Info</Typography>
              </Box>
              <Box className="EquipmentBoard">
                {EquipmentSelectionMenu(0, totalState, setTotalState)}
              </Box>
            </Box>
            <Box className="QuickSimInputContainer">
              {StatSummary(totalState)}
            </Box>
            <Box className="StatComparePartyInputContainer">
              <Box className="SelectionTitle" borderRadius={borderRadius}>
                <Typography variant="h5">2. Additional Settings</Typography>
              </Box>
              <Box className="CustomizeBoard">
                {HorizontalPartyInput(
                  totalState,
                  setTotalState
                )}
              </Box>
            </Box>
            <Box display="flex" justifyContent="center">
              {QuickSimRequestButton(
                totalState
              )}
            </Box>
          </Box>
          {Footer()}
        </Box>
      </Box>
    </>
  );
}
