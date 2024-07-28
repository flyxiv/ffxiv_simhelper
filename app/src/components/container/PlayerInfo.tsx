import { PlayerInfoBoxStyle, PlayerInfoJobTitleStyle } from "./Styles";
import { Typography, styled, Box } from "@mui/material";
import { JobIconFactory } from "../jobicon/JobIconFactory";
import { defaultQuickSimRequest } from "src/const/DefaultQuickSimRequest";
import { QuickSimRequest } from "src/types/QuickSimRequest";
import { PlayerStatInfo, StatComparePlayerStatInfo } from "./PlayerStatInfo";
import { QuickSimRequestSaveName } from "src/App";
import { CharacterStats } from "src/types/CharacterStates";

const PlayerInfoBox = styled(Box)`
  ${PlayerInfoBoxStyle}
`;

const PlayerInfoJobTitle = styled(Box)`
  ${PlayerInfoJobTitleStyle}
`;

export function PlayerInfo(job: string, combatTimeMilliseconds: number) {
  let mostRecentRequestState = localStorage.getItem(QuickSimRequestSaveName);
  let mostRecentRequest = null;
  if (mostRecentRequestState == null) {
    mostRecentRequest = defaultQuickSimRequest();
  } else {
    mostRecentRequest = JSON.parse(mostRecentRequestState) as QuickSimRequest;
  }

  let mainPlayerId = mostRecentRequest.mainPlayerId;
  let mainPlayerStats = mostRecentRequest.party[mainPlayerId].stats;

  return (
    <PlayerInfoBox>
      <PlayerInfoJobTitle>
        {JobIconFactory(job, 50)}
        <Typography variant="h3" component="div" color="white">
          {job}
        </Typography>
      </PlayerInfoJobTitle>
      {PlayerStatInfo(mainPlayerStats, combatTimeMilliseconds)}
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
