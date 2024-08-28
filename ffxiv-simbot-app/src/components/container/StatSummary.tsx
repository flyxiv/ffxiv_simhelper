import { styled, Box, Typography } from "@mui/material";
import {
  minusBackgroundColor,
  SingleStatBoxStyle,
  StatSummaryBoxStyle,
} from "./Styles";
import { getStatNames } from "../../types/ffxivdatabase/Stats";
import { ColorConfigurations } from "../../Themes";
import {
  getStatByStatName,
  getStatLostByStatName,
  getStatNeededByStatName,
  getStatPowerByStatName,
} from "../../types/ffxivdatabase/PlayerPower";
import { SingleEquipmentInputSaveState } from "../../types/SingleEquipmentInputSaveState";

let StatSummaryBox = styled(Box)`
  ${StatSummaryBoxStyle}
`;

let SingleStatBox = styled(Box)`
  ${SingleStatBoxStyle}
`;

export function StatSummary(totalState: SingleEquipmentInputSaveState) {
  let jobStatNames = getStatNames(totalState.mainPlayerJobAbbrev);
  let power = totalState.power;
  let jobAbbrev = totalState.mainPlayerJobAbbrev;
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
                {getStatByStatName(power, statName, jobAbbrev)}
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
                {getStatPowerByStatName(power, statName)}
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
          let lostStat = getStatLostByStatName(power, statName, jobAbbrev);
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
                {getStatNeededByStatName(power, statName, jobAbbrev)}
              </Typography>
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>
    </Box>
  );
}
