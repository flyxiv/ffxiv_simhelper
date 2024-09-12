import { PlayerInfoBoxStyle, PlayerInfoJobTitleStyle } from "./Styles";
import { Typography, styled, Box } from "@mui/material";
import { JobIconFactory } from "../icon/jobicon/JobIconFactory";
import { PlayerPower } from "../../types/ffxivdatabase/PlayerPower";
import { SimulationInputSummary, StatSummaryGearCompare } from "./StatSummary";
import { PARTY_MEMBERS_TEXT } from "../../const/languageTexts";

const PlayerInfoBox = styled(Box)`
  ${PlayerInfoBoxStyle}
`;

const PlayerInfoJobTitle = styled(Box)`
  ${PlayerInfoJobTitleStyle}
`;

export function PlayerInfo(power: PlayerPower, job: string, combatTimeMilliseconds: number, partyMemberJobAbbrevs: string[] | null = null) {

  return (
    <PlayerInfoBox>
      <PlayerInfoJobTitle marginBottom="30px">
        {JobIconFactory(job, 50)}
        <Typography variant="h3" component="div" color="white">
          {job}
        </Typography>
      </PlayerInfoJobTitle>
      {partyMemberJobAbbrevs != null
        ? PartyMemberJobsInfo(partyMemberJobAbbrevs)
        : null}

      {SimulationInputSummary(power, job, combatTimeMilliseconds / 1000)}
    </PlayerInfoBox>
  );
}

function PartyMemberJobsInfo(partyMemberJobAbbrevs: string[]) {
  return (
    <Box display="flex" alignItems="center" justifyContent="left" marginBottom="30px" border="1px solid black" padding={1}>
      <Box marginRight={2}>
        <Typography variant="h6" color="white"> {PARTY_MEMBERS_TEXT} </Typography>
      </Box>

      {partyMemberJobAbbrevs.map((partyMemberJobAbbrev) => {
        return JobIconFactory(partyMemberJobAbbrev, 30);
      })}
    </Box>
  );
}

export function StatComparePlayerInfo(
  jobAbbrev: string,
  targetStat: PlayerPower,
  compareStat: PlayerPower,
  combatTimeMilliseconds: number,
  partyMemberJobAbbrevs: string[]
) {
  return (
    <PlayerInfoBox>
      <PlayerInfoJobTitle>
        {JobIconFactory(jobAbbrev, 50)}
        <Typography variant="h3" component="div" color="white">
          {jobAbbrev}
        </Typography>
      </PlayerInfoJobTitle>

      {PartyMemberJobsInfo(partyMemberJobAbbrevs)}
      {StatSummaryGearCompare(jobAbbrev, targetStat, compareStat, combatTimeMilliseconds)}
    </PlayerInfoBox>
  );
}
