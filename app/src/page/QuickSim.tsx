import { useState } from "react";
import { QuickSimRequestButton } from "../components/basic/QuickSimRequestButton";
import { Box, Typography } from "@mui/material";
import "./QuickSim.css";
import { CharacterDetailedInput } from "../components/input/CharacterDetailedInput";
import { defaultQuickSimRequest } from "src/const/DefaultQuickSimRequest";
import { QuickSimRequest } from "src/types/QuickSimRequest";
import { PartyInfo } from "src/types/PartyStates";
import { CharacterStates } from "src/types/CharacterStates";
import { QuickSimPartyInput } from "../components/input/QuickSimPartyInput";
import { QuickSimRequestSaveName } from "src/App";
import { ItemSet } from "src/types/ffxivdatabase/ItemSet";
import { EMPTY_EQUIPMENT_ID } from "src/types/ffxivdatabase/Equipment";
import { ItemInputForm } from "src/components/input/ItemInputForm";

export function isNotValid(request: QuickSimRequest) {
  if (request.mainPlayerId === null || request.mainPlayerId === undefined) {
    return true;
  }

  if (
    request.combatTimeMillisecond === null ||
    request.combatTimeMillisecond === undefined
  ) {
    return true;
  }

  if (request.party === null || request.party === undefined) {
    return true;
  }

  return false;
}
export function QuickSim() {
  let mostRecentRequestState = localStorage.getItem(QuickSimRequestSaveName);
  let mostRecentRequest = null;

  if (mostRecentRequestState == null) {
    mostRecentRequest = defaultQuickSimRequest();
  } else {
    mostRecentRequest = JSON.parse(mostRecentRequestState) as QuickSimRequest;
  }

  if (isNotValid(mostRecentRequest)) {
    mostRecentRequest = defaultQuickSimRequest();
  }

  let mainPlayerId = mostRecentRequest.mainPlayerId;
  let mainPlayerInfo = mostRecentRequest.party[mainPlayerId];
  const [mainPlayerJob, setMainPlayerJob] = useState(
    mostRecentRequest.party[mostRecentRequest.mainPlayerId].job
  );
  const [mainPlayerStat, setMainPlayerStat] = useState(mainPlayerInfo.stats);
  let [mainPlayerPartner1Id, setMainPlayerPartner1Id] = useState(
    mainPlayerInfo.partner1Id
  );
  let [mainPlayerPartner2Id, setMainPlayerPartner2Id] = useState(
    mainPlayerInfo.partner2Id
  );

  const mainPlayerState: CharacterStates = {
    jobName: mainPlayerJob,
    jobNameSetter: setMainPlayerJob,
    stats: mainPlayerStat,
    setStats: setMainPlayerStat,
    partner1Id: mainPlayerPartner1Id,
    setPartner1Id: setMainPlayerPartner1Id,
    partner2Id: mainPlayerPartner2Id,
    setPartner2Id: setMainPlayerPartner2Id,
  };

  let combatTimeSeconds = mostRecentRequest.combatTimeMillisecond / 1000;
  let [combatTimeStateSeconds, setCombatTimeSeconds] =
    useState(combatTimeSeconds);

  let input = loadPartyJobs(mostRecentRequest.party);
  let ids = input.ids;
  let otherPartyJobs = input.jobs;

  let [availablePartyIds, setAvailablePartyIds] = useState(ids);

  let [partyJobs, setPartyJobs] = useState(otherPartyJobs);
  let itemSet: ItemSet = {
    head: EMPTY_EQUIPMENT_ID,
    body: EMPTY_EQUIPMENT_ID,
    hands: EMPTY_EQUIPMENT_ID,
    legs: EMPTY_EQUIPMENT_ID,
    feet: EMPTY_EQUIPMENT_ID,
    necklace: EMPTY_EQUIPMENT_ID,
    earring: EMPTY_EQUIPMENT_ID,
    bracelet: EMPTY_EQUIPMENT_ID,
    ring1: EMPTY_EQUIPMENT_ID,
    ring2: EMPTY_EQUIPMENT_ID,
    weapon: EMPTY_EQUIPMENT_ID,
    secondaryWeapon: EMPTY_EQUIPMENT_ID,
    food: EMPTY_EQUIPMENT_ID,
  };
  let borderRadius = 3;

  return (
    <>
      <Box alignContent={"center"}>
        <Box className="QuickSimInputContainer">
          <Box className="CharacterDetailCustomizeBoard">
            <Box className="SelectionTitle" borderRadius={borderRadius}>
              <Typography variant="h5">1. Input Your Info</Typography>
            </Box>
            <Box className="CustomizeBoard">
              {CharacterDetailedInput(mainPlayerState, availablePartyIds)}
            </Box>
          </Box>
          <Box className="CharacterDetailCustomizeBoard">
            <Box className="SelectionTitle" borderRadius={borderRadius}>
              <Typography variant="h5">2. Input Combat Info</Typography>
            </Box>
            <Box className="CustomizeBoard">
              {QuickSimPartyInput(
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
        </Box>
        <Box>
          {QuickSimRequestButton(
            partyJobs,
            combatTimeStateSeconds,
            mainPlayerState
          )}
        </Box>
        <Box>{ItemInputForm(mainPlayerState, availablePartyIds, itemSet)}</Box>
      </Box>
    </>
  );
}

function loadPartyJobs(partyInfo: PartyInfo[]) {
  let jobs = [];
  let ids = [];

  let i = 0;
  for (i = 1; i < 8; i++) {
    ids.push(i);
    if (partyInfo.length > i) {
      jobs.push(partyInfo[i].job);
    } else {
      jobs.push("Empty");
    }
  }

  return {
    ids: ids,
    jobs: jobs,
  };
}
