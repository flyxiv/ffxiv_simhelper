import { PlayerInfoBoxStyle, PlayerInfoJobTitleStyle } from "./Styles";
import { Typography, styled, Box } from "@mui/material";
import { JobIconFactory } from "../icon/jobicon/JobIconFactory";
import { PlayerStatInfo, StatComparePlayerStatInfo } from "./PlayerStatInfo";
import { QUICK_SIM_RESPONSE_SAVE_NAME } from "../../App";
import { QuickSimResponse } from "../../types/QuickSimResponse";
import { PlayerPower } from "../../types/ffxivdatabase/PlayerPower";

const PlayerInfoBox = styled(Box)`
  ${PlayerInfoBoxStyle}
`;

const PlayerInfoJobTitle = styled(Box)`
  ${PlayerInfoJobTitleStyle}
`;

export function PlayerInfo(job: string, combatTimeMilliseconds: number) {
  let mostRecentResponseState = localStorage.getItem(
    QUICK_SIM_RESPONSE_SAVE_NAME
  );
  let mostRecentResponse = null;
  if (mostRecentResponseState == null) {
    return;
  } else {
    mostRecentResponse = JSON.parse(
      mostRecentResponseState
    ) as QuickSimResponse;
  }

  let mainPlayerStats = mostRecentResponse.mainPlayerPower;
  let jobAbbrev =
    mostRecentResponse.simulationData[mostRecentResponse.mainPlayerId]
      .jobAbbrev;

  return (
    <PlayerInfoBox>
      <PlayerInfoJobTitle>
        {JobIconFactory(job, 50)}
        <Typography variant="h3" component="div" color="white">
          {job}
        </Typography>
      </PlayerInfoJobTitle>
      {PlayerStatInfo(mainPlayerStats, jobAbbrev, combatTimeMilliseconds)}
    </PlayerInfoBox>
  );
}

export function StatComparePlayerInfo(
  jobAbbrev: string,
  targetStat: PlayerPower,
  compareStat: PlayerPower,
  combatTimeMilliseconds: number
) {
  return (
    <PlayerInfoBox>
      <PlayerInfoJobTitle>
        {JobIconFactory(jobAbbrev, 30)}
        <Typography variant="h3" component="div" color="white">
          {jobAbbrev}
        </Typography>
      </PlayerInfoJobTitle>
      {StatComparePlayerStatInfo(
        targetStat,
        compareStat,
        jobAbbrev,
        combatTimeMilliseconds
      )}
    </PlayerInfoBox>
  );
}
