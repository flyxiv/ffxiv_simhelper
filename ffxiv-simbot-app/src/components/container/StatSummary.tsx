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
  getStatPower,
  POWER_NAMES,
} from "../../types/ffxivdatabase/PlayerPower";
import { SingleEquipmentInputSaveState } from "../../types/SingleEquipmentInputSaveState";

const STAT_SUMMARY_BOX_WIDTH = "2vw";
const POWER_SUMMARY_BOX_WIDTH = "3vw";

let StatSummaryBox = styled(Box)`
  ${StatSummaryBoxStyle}
`;

let SinglePowerBox = styled(Box)`
  ${SingleStatBoxStyle(POWER_SUMMARY_BOX_WIDTH)}
`;

let SingleStatBox = styled(Box)`
  ${SingleStatBoxStyle(STAT_SUMMARY_BOX_WIDTH)}
`;

export function StatSummaryTypography(text: string) {
  return (
    <Typography variant="body1" align="center">
      {text}
    </Typography>
  );
}

export function StatSummary(totalState: SingleEquipmentInputSaveState) {
  let jobStatNames = getStatNames(totalState.mainPlayerJobAbbrev);
  let power = totalState.power;
  let jobAbbrev = totalState.mainPlayerJobAbbrev;
  return (
    <Box display="inline-block">
      <StatSummaryBox sx={{ backgroundColor: ColorConfigurations.backgroundThree }}>
        <SingleStatBox>
          {StatSummaryTypography("Name")}
        </SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(statName)}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox sx={{
        backgroundColor: ColorConfigurations.backgroundFour
      }}>
        <SingleStatBox>
          {StatSummaryTypography("Stat")}
        </SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(getStatByStatName(power, statName, jobAbbrev))}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox sx={{ backgroundColor: ColorConfigurations.backgroundFour }}>
        <SingleStatBox>
          {StatSummaryTypography("Prev")}
        </SingleStatBox>
        {jobStatNames.map((statName) => {
          let lostStat = getStatLostByStatName(power, statName, jobAbbrev);
          let color =
            lostStat === 0
              ? ColorConfigurations.secondary
              : minusBackgroundColor;
          return (
            <SingleStatBox>
              <Typography variant="body1" color={color} align="center">
                <b>{lostStat}</b>
              </Typography>
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox sx={{ backgroundColor: ColorConfigurations.backgroundFour }}>
        <SingleStatBox>
          {StatSummaryTypography("Next")}
        </SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(getStatNeededByStatName(power, statName, jobAbbrev).toString())}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>
    </Box >
  );
}

export function StatPowerSummary(totalState: SingleEquipmentInputSaveState) {
  let power = totalState.power;

  return (
    <Box display="inline-block">
      <StatSummaryBox sx={{ backgroundColor: ColorConfigurations.backgroundThree }}>
        <SinglePowerBox>
          {StatSummaryTypography("Name")}
        </SinglePowerBox>
        {POWER_NAMES.map((powerName) => {
          return (
            <SinglePowerBox>
              {StatSummaryTypography(powerName)}
            </SinglePowerBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox sx={{
        backgroundColor: ColorConfigurations.backgroundFour
      }}>
        <SinglePowerBox>
          {StatSummaryTypography("Values")}
        </SinglePowerBox>
        {POWER_NAMES.map((powerName) => {
          return (
            <SinglePowerBox>
              {StatSummaryTypography(getStatPower(power, powerName))}
            </SinglePowerBox>
          );
        })}
      </StatSummaryBox>
    </Box >
  );
}