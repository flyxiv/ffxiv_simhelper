import { useState } from "react";
import { QuickSimRequestButton } from "../components/basic/QuickSimRequestButton";
import { Box, Typography } from "@mui/material";
import "./QuickSim.css";
import { defaultQuickSimRequest } from "src/const/DefaultQuickSimRequest";
import { QuickSimRequest } from "src/types/QuickSimRequest";
import { PartyInfo } from "src/types/PartyStates";
import { CharacterStates } from "src/types/CharacterStates";
import { VerticalPartyInput } from "../components/input/partyinput/VerticalPartyInput";
import { QuickSimRequestSaveName } from "src/App";
import { EquipmentSelectionMenu } from "src/components/input/basicform/EquipmentInputForm";
import { defaultItemSet } from "src/types/ffxivdatabase/ItemSet";
import { StatSummary } from "src/components/container/StatSummary";
import { RACES } from "src/const/StartStats";
import { HorizontalPartyInput } from "src/components/input/partyinput/HorizontalPartyInput";
import { Materia } from "src/types/ffxivdatabase/Materia";

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
  let [race, setRace] = useState(RACES[0]);

  const mainPlayerState: CharacterStates = {
    jobAbbrev: mainPlayerJob,
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
  let [itemSet, setItemSet] = useState(defaultItemSet());
  let [foodId, setFoodId] = useState(-1);
  let defaultMaterias = new Map<string, Materia[]>([
    ["weapon", []],
    ["head", []],
    ["body", []],
    ["hands", []],
    ["legs", []],
    ["feet", []],
    ["neck", []],
    ["ear", []],
    ["wrist", []],
    ["finger1", []],
    ["finger2", []],
  ]);

  let [gearSetMaterias, setGearSetMaterias] = useState(defaultMaterias);

  let borderRadius = 3;

  return (
    <>
      <Box alignContent={"center"}>
        <Box className="QuickSimInputContainer">
          <Box className="SelectionTitle" borderRadius={borderRadius}>
            <Typography variant="h5">1. Input Your Info</Typography>
          </Box>
          <Box className="EquipmentBoard">
            {EquipmentSelectionMenu(
              mainPlayerState,
              itemSet,
              setItemSet,
              race,
              setRace,
              foodId,
              setFoodId,
              gearSetMaterias,
              setGearSetMaterias
            )}
          </Box>
        </Box>
        <Box className="QuickSimInputContainer">
          {StatSummary(
            mainPlayerState.jobAbbrev,
            race,
            itemSet,
            foodId,
            gearSetMaterias
          )}
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
            mainPlayerState
          )}
        </Box>
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
