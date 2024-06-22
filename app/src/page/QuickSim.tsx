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

  const mainPlayerState: CharacterStates = {
    jobName: mainPlayerJob,
    jobNameSetter: setMainPlayerJob,
    stats: mainPlayerStat,
    setStats: setMainPlayerStat,
  };

  let combatTimeSeconds = mostRecentRequest.combatTimeMillisecond / 1000;
  let [combatTimeStateSeconds, setCombatTimeSeconds] =
    useState(combatTimeSeconds);

  let input = loadPartyJobs(mostRecentRequest.party);
  let ids = input.ids;
  let otherPartyJobs = input.jobs;

  let [partyJobs, setPartyJobs] = useState(otherPartyJobs);
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
              {CharacterDetailedInput(mainPlayerState)}
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
