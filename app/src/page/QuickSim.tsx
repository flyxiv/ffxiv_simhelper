import { useState } from "react";
import { QuickSimRequestButton } from "../components/basic/QuickSimRequestButton";
import { Box, Typography } from "@mui/material";
import "./QuickSim.css";
import { CharacterDetailedInput } from "../components/input/CharacterDetailedInput";
import { defaultQuickSimRequest } from "src/const/DefaultQuickSimRequest";
import { PartyInfo, QuickSimRequest } from "src/types/QuickSimRequest";
import { CharacterStates } from "src/types/CharacterStates";
import { QuickSimPartyInput } from "../components/input/QuickSimPartyInput";
import {
  QuickSimPartyMemberState,
  QuickSimPartyState,
} from "src/types/QuickSimPartyStates";

export function QuickSim() {
  let mostRecentRequestState = localStorage.getItem("mostRecentRequest");
  let mostRecentRequest = null;

  if (mostRecentRequestState == null) {
    mostRecentRequest = defaultQuickSimRequest();
  } else {
    mostRecentRequest = JSON.parse(mostRecentRequestState) as QuickSimRequest;
  }

  let mainPlayerId = mostRecentRequest.mainPlayerId;
  let mainPlayerInfo = mostRecentRequest.party[mainPlayerId];
  const [mainPlayerJob, setMainPlayerJob] = useState(
    mostRecentRequest.party[mostRecentRequest.mainPlayerId].job
  );

  const [mainPlayerWeaponDamageState, setMainPlayerWeaponDamageState] =
    useState(mainPlayerInfo.stats.weaponDamage);
  const [mainPlayerMainStatState, setMainPlayerMainStatState] = useState(
    mainPlayerInfo.stats.mainStat
  );
  const [mainPlayerCriticalStrikeState, setMainPlayerCriticalStrikeState] =
    useState(mainPlayerInfo.stats.criticalStrike);
  const [mainPlayerDirectHitState, setMainPlayerDirectHitState] = useState(
    mainPlayerInfo.stats.directHit
  );
  const [mainPlayerDeterminationState, setMainPlayerDeterminationState] =
    useState(mainPlayerInfo.stats.determination);
  const [mainPlayerSpeedState, setMainPlayerSpeedState] = useState(
    mainPlayerInfo.stats.speed
  );
  const [mainPlayerTenacityState, setMainPlayerTenacityState] = useState(
    mainPlayerInfo.stats.tenacity
  );

  const mainPlayerState: CharacterStates = {
    jobName: mainPlayerJob,
    jobNameSetter: setMainPlayerJob,
    value: {
      weaponDamage: mainPlayerWeaponDamageState,
      mainStat: mainPlayerMainStatState,
      criticalStrike: mainPlayerCriticalStrikeState,
      directHit: mainPlayerDirectHitState,
      determination: mainPlayerDeterminationState,
      speed: mainPlayerSpeedState,
      tenacity: mainPlayerTenacityState,
    },
    setter: {
      weaponAttack: setMainPlayerWeaponDamageState,
      mainStat: setMainPlayerMainStatState,
      criticalStrike: setMainPlayerCriticalStrikeState,
      directHit: setMainPlayerDirectHitState,
      determination: setMainPlayerDeterminationState,
      speed: setMainPlayerSpeedState,
      tenacity: setMainPlayerTenacityState,
    },
  };

  let combatTimeSeconds = mostRecentRequest.combatTimeMillisecond / 1000;
  let [combatTimeStateSeconds, setCombatTimeSeconds] =
    useState(combatTimeSeconds);

  let input = loadPartyJobs(mostRecentRequest.party);
  let ids = input.ids;
  let otherPartyJobs = input.jobs;

  let [partyJobs, setPartyJobs] = useState(otherPartyJobs);

  return (
    <>
      <Box className="CharacterDetailCustomizeBoard">
        <Box className="SelectionTitle">
          <Typography variant="h5">1. Input Your Info</Typography>
        </Box>
        <Box className="CustomizeBoard">
          {CharacterDetailedInput(mainPlayerState)}
        </Box>
      </Box>
      <Box className="QuickSimPartyInput">
        <Box className="SelectionTitle">
          <Typography variant="h5">2. Input Combat Info</Typography>
        </Box>
        <Box className="PartyJobInput">
          {QuickSimPartyInput(
            ids,
            partyJobs,
            setPartyJobs,
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
