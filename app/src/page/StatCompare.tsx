import { useState } from "react";
import { Box, Typography } from "@mui/material";
import "./StatCompare.css";
import { CharacterDetailedInput } from "../components/input/CharacterDetailedInput";
import { PartyInfo } from "src/types/PartyStates";
import { StatCompareRequestSaveName } from "src/App";
import { defaultStatCompareRequest } from "src/const/DefaultStatCompareRequest";
import { StatCompareRequest } from "src/types/StatCompareRequest";
import { CharacterStates } from "src/types/CharacterStates";
import { StatCompareRequestButton } from "src/components/basic/StatCompareRequestButton";
import { HorizontalPartyInput } from "src/components/input/partyinput/HorizontalPartyInput";

function isNotValid(request: StatCompareRequest) {
  if (request.mainPlayerId === null || request.mainPlayerId === undefined) {
    return true;
  }

  if (request.mainPlayerJob === null || request.mainPlayerJob === undefined) {
    return true;
  }

  if (
    request.combatTimeMillisecond === null ||
    request.combatTimeMillisecond === undefined
  ) {
    return true;
  }

  if (
    request.mainPlayerStat1 === null ||
    request.mainPlayerStat1 === undefined
  ) {
    return true;
  }

  if (
    request.mainPlayerStat2 === null ||
    request.mainPlayerStat2 === undefined
  ) {
    return true;
  }

  if (request.party === null || request.party === undefined) {
    return true;
  }

  return false;
}

export function StatCompare() {
  let mostRecentRequestState = localStorage.getItem(StatCompareRequestSaveName);
  let mostRecentRequest = null;

  if (mostRecentRequestState == null) {
    mostRecentRequest = defaultStatCompareRequest();
  } else {
    mostRecentRequest = JSON.parse(
      mostRecentRequestState
    ) as StatCompareRequest;
  }

  if (isNotValid(mostRecentRequest)) {
    mostRecentRequest = defaultStatCompareRequest();
  }

  let [mainPlayerJob, setMainPlayerJob] = useState(
    mostRecentRequest.mainPlayerJob
  );
  const [mainPlayerStat1, setMainPlayerStat1] = useState(
    mostRecentRequest.mainPlayerStat1
  );
  let [mainPlayerPartner1Id, setMainPlayerPartner1Id] = useState(null);
  let [mainPlayerPartner2Id, setMainPlayerPartner2Id] = useState(null);

  const mainPlayerState1: CharacterStates = {
    jobAbbrev: mainPlayerJob,
    jobNameSetter: setMainPlayerJob,
    stats: mainPlayerStat1,
    setStats: setMainPlayerStat1,
    partner1Id: mainPlayerPartner1Id,
    setPartner1Id: setMainPlayerPartner1Id,
    partner2Id: mainPlayerPartner2Id,
    setPartner2Id: setMainPlayerPartner2Id,
  };

  const [mainPlayerStat2, setMainPlayerStat2] = useState(
    mostRecentRequest.mainPlayerStat2
  );
  const mainPlayerState2: CharacterStates = {
    jobAbbrev: mainPlayerJob,
    jobNameSetter: setMainPlayerJob,
    stats: mainPlayerStat2,
    setStats: setMainPlayerStat2,
    partner1Id: mainPlayerPartner1Id,
    setPartner1Id: setMainPlayerPartner1Id,
    partner2Id: mainPlayerPartner2Id,
    setPartner2Id: setMainPlayerPartner2Id,
  };

  let combatTimeSeconds = mostRecentRequest.combatTimeMillisecond / 1000;
  let [combatTimeStateSeconds, setCombatTimeSeconds] =
    useState(combatTimeSeconds);

  let input = loadStatComparePartyJobs(mostRecentRequest.party);
  let ids = input.ids.map((id) => id + 1);
  let otherPartyJobs = input.jobs;
  let [availablePartyIds, setAvailablePartyIds] = useState(ids);

  let [partyJobs, setPartyJobs] = useState(otherPartyJobs);
  let borderRadius = 3;

  return (
    <>
      <Box alignContent={"center"}>
        <Box className="StatCompareInputContainer">
          <Box className="CharacterDetailCustomizeBoard">
            <Box className="SelectionTitle" borderRadius={borderRadius}>
              <Typography variant="h5">1. Input Stat1</Typography>
            </Box>
            <Box className="CustomizeBoard">
              {CharacterDetailedInput(mainPlayerState1, availablePartyIds)}
            </Box>
          </Box>
          <Box className="CharacterDetailCustomizeBoard">
            <Box className="SelectionTitle" borderRadius={borderRadius}>
              <Typography variant="h5">2. Input Stat2</Typography>
            </Box>
            <Box className="CustomizeBoard">
              {CharacterDetailedInput(mainPlayerState2, availablePartyIds)}
            </Box>
          </Box>
        </Box>
        <Box className="StatComparePartyInputContainer">
          <Box className="SelectionTitle" borderRadius={borderRadius}>
            <Typography variant="h5">3. Additional Settings</Typography>
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
          {StatCompareRequestButton(
            partyJobs,
            combatTimeStateSeconds,
            mainPlayerState1,
            mainPlayerState2
          )}
        </Box>
      </Box>
    </>
  );
}

function loadStatComparePartyJobs(partyInfo: PartyInfo[]) {
  let jobs = [];
  let ids = [];

  let i = 0;
  for (i = 0; i < 7; i++) {
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
