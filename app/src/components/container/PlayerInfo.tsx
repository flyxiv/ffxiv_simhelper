import { PlayerInfoBoxStyle, PlayerInfoJobTitleStyle } from "./Styles";
import { Typography, styled, Box } from "@mui/material";
import { JobIconFactory } from "../jobicon/JobIconFactory";
import { defaultQuickSimRequest } from "src/const/DefaultQuickSimRequest";
import { QuickSimRequest } from "src/types/QuickSimRequest";
import { PlayerStatInfo } from "./PlayerStatInfo";

const PlayerInfoBox = styled(Box)`
  ${PlayerInfoBoxStyle}
`;

const PlayerInfoJobTitle = styled(Box)`
  ${PlayerInfoJobTitleStyle}
`;

export function PlayerInfo(job: string) {
  let mostRecentRequestState = localStorage.getItem("mostRecentRequest");
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
        {JobIconFactory(job)}
        <Typography variant="h3" component="div" color="white">
          {job}
        </Typography>
      </PlayerInfoJobTitle>
      {PlayerStatInfo(mainPlayerStats)}
    </PlayerInfoBox>
  );
}
