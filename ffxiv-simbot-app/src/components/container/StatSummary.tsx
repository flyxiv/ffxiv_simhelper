import { styled, Box, Typography } from "@mui/material";
import {
  minusBackgroundColor,
  SingleStatBoxStyle,
  StatSummaryBoxStyle,
} from "./Styles";
import { getStatNames } from "../../types/ffxivdatabase/Stats";
import { AppConfigurations } from "../../Themes";
import {
  getStatByStatName,
  getStatLostByStatName,
  getStatNeededByStatName,
  getStatPower,
  PlayerPower,
  POWER_NAMES,
} from "../../types/ffxivdatabase/PlayerPower";
import { SingleEquipmentInputSaveState } from "../../types/EquipmentInput";
import { StatWeightsData } from "../../page/StatWeightsResult";
import { NAME_TEXT, NEXT_TEXT, PREV_TEXT, STAT_TEXT, TIME_TEXT, VALUES_TEXT } from "../../const/languageTexts";

const STAT_SUMMARY_BOX_WIDTH = "3vw";
const POWER_SUMMARY_BOX_WIDTH = "4vw";

let StatSummaryBox = styled(Box)`
  ${StatSummaryBoxStyle}
`;

let SinglePowerBox = styled(Box)`
  ${SingleStatBoxStyle(POWER_SUMMARY_BOX_WIDTH)}
`;

let SingleStatBox = styled(Box)`
  ${SingleStatBoxStyle(STAT_SUMMARY_BOX_WIDTH)}
`;

export function StatSummaryTypography(text: string, smaller: boolean) {
  let fontSize = 16;

  if (smaller) {
    fontSize = 13;
  }

  return (
    <Typography variant="body1" align="center" fontSize={fontSize}>
      {text}
    </Typography>
  );
}

export function StatSummaryTypographyCompare(stat: string, compareStat: string, smaller: boolean) {
  let fontSize = 16;

  if (smaller) {
    fontSize = 13;
  }

  let statValue = parseFloat(stat);
  let compareStatValue = parseFloat(compareStat);

  let color = statValue > compareStatValue ? AppConfigurations.secondary : statValue < compareStatValue ? AppConfigurations.alert : "white";


  return (
    <Typography variant="body1" align="center" fontSize={fontSize} color={color}>
      {stat}
    </Typography>
  );
}

export function SimulationInputSummary(power: PlayerPower, jobAbbrev: string, combatTimeMilliseconds: number) {
  let simulationInputNames = getStatNames(jobAbbrev);

  return (
    <Box display="inline-block">
      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundThree }}
      >
        <SingleStatBox>{StatSummaryTypography(NAME_TEXT, false)}</SingleStatBox>
        <SingleStatBox>{StatSummaryTypography(TIME_TEXT, false)}</SingleStatBox>
        {simulationInputNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(statName, false)}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox
        sx={{
          backgroundColor: AppConfigurations.backgroundFour,
        }}
      >
        <SingleStatBox>{StatSummaryTypography(VALUES_TEXT, false)}</SingleStatBox>
        <SingleStatBox>{StatSummaryTypography(`${combatTimeMilliseconds}`, false)}</SingleStatBox>
        {simulationInputNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(
                getStatByStatName(power, statName, jobAbbrev),
                false
              )}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>
    </Box>
  );
}

export function StatSummaryGearCompare(jobAbbrev: string, power: PlayerPower, comparePower: PlayerPower, combatTimeMilliseconds: number) {
  let jobStatNames = getStatNames(jobAbbrev);

  return (
    <Box display="inline-block">
      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundThree }}
      >
        <SingleStatBox>{StatSummaryTypography(NAME_TEXT, false)}</SingleStatBox>
        <SingleStatBox>{StatSummaryTypography(TIME_TEXT, false)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(statName, false)}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox
        sx={{
          backgroundColor: AppConfigurations.backgroundFour,
        }}
      >
        <SingleStatBox>{StatSummaryTypography(VALUES_TEXT, false)}</SingleStatBox>
        <SingleStatBox>{StatSummaryTypography(`${combatTimeMilliseconds / 1000}`, false)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypographyCompare(
                getStatByStatName(power, statName, jobAbbrev),
                getStatByStatName(comparePower, statName, jobAbbrev),
                false
              )}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>
    </Box>
  )
}


export function StatSummary(totalState: SingleEquipmentInputSaveState) {
  let jobStatNames = getStatNames(totalState.mainPlayerJobAbbrev);
  let power = totalState.power;
  let jobAbbrev = totalState.mainPlayerJobAbbrev;
  return (
    <Box display="inline-block">
      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundThree }}
      >
        <SingleStatBox>{StatSummaryTypography(NAME_TEXT, false)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(statName, false)}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox
        sx={{
          backgroundColor: AppConfigurations.backgroundFour,
        }}
      >
        <SingleStatBox>{StatSummaryTypography(STAT_TEXT, false)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(
                getStatByStatName(power, statName, jobAbbrev),
                false
              )}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundFour }}
      >
        <SingleStatBox>{StatSummaryTypography(PREV_TEXT, false)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          let lostStat = getStatLostByStatName(power, statName, jobAbbrev);
          let color =
            lostStat === 0
              ? AppConfigurations.secondary
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

      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundFour }}
      >
        <SingleStatBox>{StatSummaryTypography(NEXT_TEXT, false)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(
                getStatNeededByStatName(power, statName, jobAbbrev).toString(),
                false
              )}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>
    </Box>
  );
}

export function StatWeightSummary(statWeightsData: StatWeightsData[]) {
  return (
    <Box display="inline-block">
      <Box display="flex">
        <StatSummaryBox
          sx={{ backgroundColor: AppConfigurations.backgroundThree }}
        >
          <SingleStatBox>{StatSummaryTypography(STAT_TEXT, false)}</SingleStatBox>

        </StatSummaryBox>
        <StatSummaryBox
          sx={{ backgroundColor: AppConfigurations.backgroundThree }}
        >
          <SingleStatBox>{StatSummaryTypography(VALUES_TEXT, false)}</SingleStatBox>

        </StatSummaryBox>
      </Box>

      {statWeightsData.map((statWeightData) => {
        return (
          <Box display="flex">
            <StatSummaryBox
              sx={{ backgroundColor: AppConfigurations.backgroundFour }}
            >
              <SingleStatBox>{StatSummaryTypography(statWeightData.statName, false)}</SingleStatBox>

            </StatSummaryBox>
            <StatSummaryBox
              sx={{ backgroundColor: AppConfigurations.backgroundFour }}
            >
              <SingleStatBox>{StatSummaryTypography(`${statWeightData.rdpsIncreasePerPoint.toFixed(2)}`, false)}</SingleStatBox>

            </StatSummaryBox>
          </Box>
        )

      })}
    </Box>
  );
}


export function StatPowerSummary(totalState: SingleEquipmentInputSaveState) {
  let power = totalState.power;

  return (
    <Box display="inline-block">
      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundThree }}
      >

        <SinglePowerBox sx={{ flexGrow: 1 }}>{StatSummaryTypography(NAME_TEXT, true)}</SinglePowerBox>
        {POWER_NAMES.map((powerName) => {
          return (
            <SinglePowerBox>
              {StatSummaryTypography(powerName, true)}
            </SinglePowerBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox
        sx={{
          backgroundColor: AppConfigurations.backgroundFour,
        }}
      >
        <SinglePowerBox>{StatSummaryTypography(VALUES_TEXT, true)}</SinglePowerBox>
        {POWER_NAMES.map((powerName) => {
          return (
            <SinglePowerBox>
              {StatSummaryTypography(getStatPower(power, powerName), true)}
            </SinglePowerBox>
          );
        })}
      </StatSummaryBox>
    </Box>
  );
}
