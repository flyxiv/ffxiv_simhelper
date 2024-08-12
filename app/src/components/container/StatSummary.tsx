import { styled, Box, Typography } from "@mui/material";
import {
  minusBackgroundColor,
  SingleStatBoxStyle,
  StatSummaryBoxStyle,
} from "./Styles";
import { CharacterStates } from "src/types/CharacterStates";
import { getStatNames } from "src/types/ffxivdatabase/Stats";
import {
  calculateTotalStatsOfItemSet,
  getStatByStatName,
  getStatLostByStatName,
  getStatNeededByStatName,
  getStatPowerByStatName,
  ItemSet,
} from "src/types/ffxivdatabase/ItemSet";
import { ColorConfigurations } from "src/Themes";
import { Materia } from "src/types/ffxivdatabase/Materia";

let StatSummaryBox = styled(Box)`
  ${StatSummaryBoxStyle}
`;

let SingleStatBox = styled(Box)`
  ${SingleStatBoxStyle}
`;

export function StatSummary(
  jobAbbrev: string,
  race: string,
  itemSet: ItemSet,
  foodId: number,
  gearSetMaterias: Map<string, Materia[]>
) {
  let jobStatNames = getStatNames(jobAbbrev);
  let totalStats = calculateTotalStatsOfItemSet(
    itemSet,
    jobAbbrev,
    race,
    foodId,
    gearSetMaterias
  );
  return (
    <Box>
      <StatSummaryBox>
        <SingleStatBox>
          <Typography variant="body1">Name</Typography>
        </SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              <Typography variant="body1">{statName}</Typography>
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox>
        <SingleStatBox>
          <Typography variant="body1">Stat</Typography>
        </SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              <Typography variant="body1">
                {getStatByStatName(totalStats, statName, jobAbbrev)}
              </Typography>
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox>
        <SingleStatBox>
          <Typography variant="body2">Power</Typography>
        </SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              <Typography variant="body2">
                {getStatPowerByStatName(totalStats, statName, jobAbbrev)}
              </Typography>
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox>
        <SingleStatBox>
          <Typography variant="body1">Prev</Typography>
        </SingleStatBox>
        {jobStatNames.map((statName) => {
          let lostStat = getStatLostByStatName(totalStats, statName, jobAbbrev);
          let color =
            lostStat === 0
              ? ColorConfigurations.secondary
              : minusBackgroundColor;
          return (
            <SingleStatBox>
              <Typography variant="body1" color={color}>
                <b>{lostStat}</b>
              </Typography>
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>
      <StatSummaryBox>
        <SingleStatBox>
          <Typography variant="body1">Next</Typography>
        </SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              <Typography variant="body1">
                {getStatNeededByStatName(totalStats, statName, jobAbbrev)}
              </Typography>
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>
    </Box>
  );
}
