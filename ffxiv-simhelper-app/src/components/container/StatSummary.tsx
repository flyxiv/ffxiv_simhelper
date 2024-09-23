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
import {
  NAME_TEXT,
  NEXT_TEXT,
  PREV_TEXT,
  STAT_TEXT,
  TIME_TEXT,
  VALUES_TEXT,
} from "../../const/languageTexts";

const STAT_SUMMARY_BOX_WIDTH = "3vw";
const STAT_SUMMARY_TIME_BOX_WIDTH = "6vw";
const POWER_SUMMARY_BOX_WIDTH = "6vw";

let StatSummaryBox = styled(Box)`
  ${StatSummaryBoxStyle}
`;

let SinglePowerBox = styled(Box)`
  ${SingleStatBoxStyle(POWER_SUMMARY_BOX_WIDTH)}
`;

let SingleStatBox = styled(Box)`
  ${SingleStatBoxStyle(STAT_SUMMARY_BOX_WIDTH)}
`;

let SingleStatCombatTimeBox = styled(Box)`
  ${SingleStatBoxStyle(STAT_SUMMARY_TIME_BOX_WIDTH)}
`;

export function StatSummaryTypography(text: string) {
  return (
    <Typography variant="body1" align="center" fontSize={"1vw"}>
      {text}
    </Typography>
  );
}

export function StatSummaryTypographyCompare(
  stat: string,
  compareStat: string
) {
  let statValue = parseFloat(stat);
  let compareStatValue = parseFloat(compareStat);

  let color =
    statValue > compareStatValue
      ? AppConfigurations.secondary
      : statValue < compareStatValue
        ? AppConfigurations.alert
        : "white";

  return (
    <Typography variant="body1" align="center" fontSize={"1vw"} color={color}>
      {stat}
    </Typography>
  );
}

export function SimulationInputSummary(
  power: PlayerPower,
  jobAbbrev: string,
  combatTimeMilliseconds: number
) {
  let simulationInputNames = getStatNames(jobAbbrev);

  return (
    <Box display="inline-block">
      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundThree }}
      >
        <SingleStatBox>{StatSummaryTypography(NAME_TEXT)}</SingleStatBox>
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(TIME_TEXT)}
        </SingleStatCombatTimeBox>
        {simulationInputNames.map((statName) => {
          return (
            <SingleStatBox>{StatSummaryTypography(statName)}</SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox
        sx={{
          backgroundColor: AppConfigurations.backgroundFour,
        }}
      >
        <SingleStatBox>{StatSummaryTypography(VALUES_TEXT)}</SingleStatBox>
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(`${combatTimeMilliseconds}`)}
        </SingleStatCombatTimeBox>
        {simulationInputNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(
                getStatByStatName(power, statName, jobAbbrev)
              )}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>
    </Box>
  );
}

export function StatSummaryGearCompare(
  jobAbbrev: string,
  power: PlayerPower,
  comparePower: PlayerPower,
  combatTimeMilliseconds: number
) {
  let jobStatNames = getStatNames(jobAbbrev);

  return (
    <Box display="inline-block">
      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundThree }}
      >
        <SingleStatBox>{StatSummaryTypography(NAME_TEXT)}</SingleStatBox>
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(TIME_TEXT)}
        </SingleStatCombatTimeBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>{StatSummaryTypography(statName)}</SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox
        sx={{
          backgroundColor: AppConfigurations.backgroundFour,
        }}
      >
        <SingleStatBox>{StatSummaryTypography(VALUES_TEXT)}</SingleStatBox>
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(`${combatTimeMilliseconds / 1000}`)}
        </SingleStatCombatTimeBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypographyCompare(
                getStatByStatName(power, statName, jobAbbrev),
                getStatByStatName(comparePower, statName, jobAbbrev)
              )}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>
    </Box>
  );
}

export function StatSummary(totalState: SingleEquipmentInputSaveState) {
  let jobStatNames = getStatNames(totalState.mainPlayerJobAbbrev);
  let power = totalState.power;
  let jobAbbrev = totalState.mainPlayerJobAbbrev;
  return (
    <Box display="flex" flexDirection={"column"}>
      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundThree }}
      >
        <SingleStatBox>{StatSummaryTypography(NAME_TEXT)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>{StatSummaryTypography(statName)}</SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox
        sx={{
          backgroundColor: AppConfigurations.backgroundFour,
        }}
      >
        <SingleStatBox>{StatSummaryTypography(STAT_TEXT)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(
                getStatByStatName(power, statName, jobAbbrev)
              )}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundFour }}
      >
        <SingleStatBox>{StatSummaryTypography(PREV_TEXT)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          let lostStat = getStatLostByStatName(power, statName, jobAbbrev);
          let color =
            lostStat === 0 ? AppConfigurations.secondary : minusBackgroundColor;
          return (
            <SingleStatBox>
              <Typography
                variant="body1"
                color={color}
                align="center"
                fontSize={"1vw"}
              >
                <b>{lostStat}</b>
              </Typography>

            </SingleStatBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundFour }}
      >
        <SingleStatBox>{StatSummaryTypography(NEXT_TEXT)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(
                getStatNeededByStatName(power, statName, jobAbbrev).toString()
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
          <SingleStatBox>{StatSummaryTypography(STAT_TEXT)}</SingleStatBox>
        </StatSummaryBox>
        <StatSummaryBox
          sx={{ backgroundColor: AppConfigurations.backgroundThree }}
        >
          <SingleStatBox>{StatSummaryTypography(VALUES_TEXT)}</SingleStatBox>
        </StatSummaryBox>
      </Box>

      {statWeightsData.map((statWeightData) => {
        return (
          <Box display="flex">
            <StatSummaryBox
              sx={{ backgroundColor: AppConfigurations.backgroundFour }}
            >
              <SingleStatBox>
                {StatSummaryTypography(statWeightData.statName)}
              </SingleStatBox>
            </StatSummaryBox>
            <StatSummaryBox
              sx={{ backgroundColor: AppConfigurations.backgroundFour }}
            >
              <SingleStatBox>
                {StatSummaryTypography(
                  `${statWeightData.rdpsIncreasePerPoint.toFixed(2)}`
                )}
              </SingleStatBox>
            </StatSummaryBox>
          </Box>
        );
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
        <SinglePowerBox sx={{ flexGrow: 1 }}>
          {StatSummaryTypography(NAME_TEXT)}
        </SinglePowerBox>
        {POWER_NAMES.map((powerName) => {
          return (
            <SinglePowerBox>{StatSummaryTypography(powerName)}</SinglePowerBox>
          );
        })}
      </StatSummaryBox>

      <StatSummaryBox
        sx={{
          backgroundColor: AppConfigurations.backgroundFour,
        }}
      >
        <SinglePowerBox>{StatSummaryTypography(VALUES_TEXT)}</SinglePowerBox>
        {POWER_NAMES.map((powerName) => {
          return (
            <SinglePowerBox>
              {StatSummaryTypography(getStatPower(power, powerName))}
            </SinglePowerBox>
          );
        })}
      </StatSummaryBox>
    </Box>
  );
}
