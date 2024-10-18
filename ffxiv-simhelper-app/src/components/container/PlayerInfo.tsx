import { PlayerInfoBoxStyle, PlayerInfoJobTitleStyle } from "./Styles";
import { Typography, styled, Box } from "@mui/material";
import { JobIconFactory } from "../icon/jobicon/JobIconFactory";
import { PlayerPower } from "../../types/ffxivdatabase/PlayerPower";
import { SimulationInputSummary, StatSummaryGearCompare } from "./StatSummary";
import { TextDictionary } from "../../const/languageTexts";

const PlayerInfoBox = styled(Box)`
  ${PlayerInfoBoxStyle}
`;

const PlayerInfoJobTitle = styled(Box)`
  ${PlayerInfoJobTitleStyle}
`;

export function PlayerInfo(power: PlayerPower, job: string, combatTimeMilliseconds: number, partyMemberJobAbbrevs: string[] | null = null, iterationCount: number, variancePercent: number, LANGUAGE_TEXTS: TextDictionary, showStats: boolean = true) {

  return (
    <PlayerInfoBox>
      <PlayerInfoJobTitle marginBottom="30px">
        {JobIconFactory(job, 50)}
        <Typography variant="h3" component="div" color="white">
          {job}
        </Typography>
      </PlayerInfoJobTitle>
      {partyMemberJobAbbrevs != null
        ? PartyMemberJobsInfo(partyMemberJobAbbrevs, LANGUAGE_TEXTS.PARTY_MEMBERS_TEXT)
        : null}

      {SimulationInputSummary(power, job, combatTimeMilliseconds / 1000, iterationCount, variancePercent, LANGUAGE_TEXTS, showStats)}
    </PlayerInfoBox>
  );
}

function PartyMemberJobsInfo(partyMemberJobAbbrevs: string[], partyMembersText: string) {
  return (
    <Box display="flex" justifyContent="center" alignItems="center" flexDirection="column">
      <Box>
        <Typography variant="h6" color="white" align="center"> {partyMembersText} </Typography>
      </Box>


      <Box display="flex" alignItems="center" justifyContent="left" marginBottom="30px" border="1px solid black" padding={1}>
        {partyMemberJobAbbrevs.map((partyMemberJobAbbrev) => {
          return JobIconFactory(partyMemberJobAbbrev, 30);
        })}
      </Box>
    </Box>
  );
}

export function StatComparePlayerInfo(
  jobAbbrev: string,
  targetStat: PlayerPower,
  compareStat: PlayerPower,
  combatTimeMilliseconds: number,
  partyMemberJobAbbrevs: string[],
  iterationCount: number,
  variancePercent: number,
  LANGUAGE_TEXTS: TextDictionary
) {
  return (
    <PlayerInfoBox>
      <PlayerInfoJobTitle>
        {JobIconFactory(jobAbbrev, 50)}
        <Typography variant="h3" component="div" color="white">
          {jobAbbrev}
        </Typography>
      </PlayerInfoJobTitle>

      {PartyMemberJobsInfo(partyMemberJobAbbrevs, LANGUAGE_TEXTS.PARTY_MEMBERS_TEXT)}
      {StatSummaryGearCompare(jobAbbrev, targetStat, compareStat, combatTimeMilliseconds, iterationCount, variancePercent, LANGUAGE_TEXTS)}
    </PlayerInfoBox>
  );
}
