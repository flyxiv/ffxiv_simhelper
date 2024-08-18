import { useState } from "react";
import { QuickSimRequestButton } from "../components/basic/QuickSimRequestButton";
import { Box, Typography } from "@mui/material";
import "./QuickSim.css";
import { EquipmentSimCharacterStates } from "src/types/CharacterStates";
import { QuickSimInputSaveName } from "src/App";
import { calculatePlayerPowerFromInputs } from "src/types/ffxivdatabase/ItemSet";
import { EquipmentSelectionMenu } from "src/components/input/basicform/EquipmentInputForm";
import { StatSummary } from "src/components/container/StatSummary";
import { HorizontalPartyInput } from "src/components/input/partyinput/HorizontalPartyInput";
import { QuickSimInputSaveState } from "src/types/QuickSimInput";
import { defaultQuickSimInput } from "src/const/DefaultQuickSimInput";

export function isNotValid(input: QuickSimInputSaveState) {
  if (input.mainPlayerJob === null || input.mainPlayerJob === undefined) {
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

  if (mostRecentInputState == null) {
    mostRecentInput = defaultQuickSimInput();
  } else {
    mostRecentInput = JSON.parse(
      mostRecentInputState
    ) as QuickSimInputSaveState;
  }

  if (isNotValid(mostRecentInput)) {
    mostRecentInput = defaultQuickSimInput();
  }

  const [mainPlayerJobAbbrev, setMainPlayerJobAbbrev] = useState(
    mostRecentInput.mainPlayerJob
  );
  let [mainPlayerPartner1Id, setMainPlayerPartner1Id] = useState(
    mostRecentInput.mainPlayerPartner1Id
  );
  let [mainPlayerPartner2Id, setMainPlayerPartner2Id] = useState(
    mostRecentInput.mainPlayerPartner2Id
  );
  const mainPlayerState: EquipmentSimCharacterStates = {
    jobAbbrev: mainPlayerJobAbbrev,
    jobNameSetter: setMainPlayerJobAbbrev,
    partner1Id: mainPlayerPartner1Id,
    setPartner1Id: setMainPlayerPartner1Id,
    partner2Id: mainPlayerPartner2Id,
    setPartner2Id: setMainPlayerPartner2Id,
  };

  let combatTimeSeconds = mostRecentInput.combatTimeMillisecond / 1000;
  let [combatTimeStateSeconds, setCombatTimeSeconds] =
    useState(combatTimeSeconds);

  let partyJobAbbrevs = mostRecentInput.partyMemberJobAbbrevs;
  let ids = [1, 2, 3, 4, 5, 6, 7];

  let [availablePartyIds, setAvailablePartyIds] = useState(ids);

  let [partyJobs, setPartyJobs] = useState(partyJobAbbrevs);
  let initialPower = calculatePlayerPowerFromInputs(
    mostRecentInput.itemSet,
    mainPlayerJobAbbrev,
    mostRecentInput.race,
    mostRecentInput.foodId,
    mostRecentInput.gearSetMaterias
  );

  let [data, setData] = useState({
    power: initialPower,
    itemSet: mostRecentInput.itemSet,
    foodId: mostRecentInput.foodId,
    gearSetMaterias: mostRecentInput.gearSetMaterias,
    jobAbbrev: mainPlayerJobAbbrev,
    race: mostRecentInput.race,
  });

  let borderRadius = 3;

  return (
    <>
      <Box alignContent={"center"}>
        <Box className="QuickSimInputContainer">
          <Box className="SelectionTitle" borderRadius={borderRadius}>
            <Typography variant="h5">1. Input Your Info</Typography>
          </Box>
          <Box className="EquipmentBoard">
            {EquipmentSelectionMenu(0, mainPlayerState, data, setData)}
          </Box>
        </Box>
        <Box className="QuickSimInputContainer">
          {StatSummary(mainPlayerState.jobAbbrev, data.power)}
        </Box>
        <Box className="StatComparePartyInputContainer">
          <Box className="SelectionTitle" borderRadius={borderRadius}>
            <Typography variant="h5">2. Additional Settings</Typography>
          </Box>
          <Box className="CustomizeBoard">
            {HorizontalPartyInput(
              ids,
              partyJobs,
              setPartyJobs,
              availablePartyIds,
              setAvailablePartyIds,
              combatTimeStateSeconds,
              setCombatTimeSeconds
            )}
          </Box>
        </Box>
        <Box>
          {QuickSimRequestButton(
            partyJobs,
            combatTimeStateSeconds,
            mainPlayerState,
            data
          )}
        </Box>
      </Box>
    </>
  );
}
