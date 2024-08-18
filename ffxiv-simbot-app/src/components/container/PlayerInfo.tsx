import { PlayerInfoBoxStyle, PlayerInfoJobTitleStyle } from "./Styles";
import { Typography, styled, Box } from "@mui/material";
import { JobIconFactory } from "../icon/jobicon/JobIconFactory";
import { PlayerStatInfo, StatComparePlayerStatInfo } from "./PlayerStatInfo";
import { QuickSimResponseSaveName } from "../../App";
import { CharacterStats } from "../../types/CharacterStates";
import { QuickSimResponse } from "../../types/QuickSimResponse";

const PlayerInfoBox = styled(Box)`
  ${PlayerInfoBoxStyle}
`;

const PlayerInfoJobTitle = styled(Box)`
  ${PlayerInfoJobTitleStyle}
`;

export function PlayerInfo(job: string, combatTimeMilliseconds: number) {
  let mostRecentResponseState = localStorage.getItem(QuickSimResponseSaveName);
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
  job: string,
  targetStat: CharacterStats,
  compareStat: CharacterStats,
  combatTimeMilliseconds: number
) {
  return (
    <PlayerInfoBox>
      <PlayerInfoJobTitle>
        {JobIconFactory(job, 30)}
        <Typography variant="h3" component="div" color="white">
          {job}
        </Typography>
      </PlayerInfoJobTitle>
      {StatComparePlayerStatInfo(
        targetStat,
        compareStat,
        combatTimeMilliseconds
      )}
    </PlayerInfoBox>
  );
}
