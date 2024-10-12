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
  loadPowerNames,
  PlayerPower,
} from "../../types/ffxivdatabase/PlayerPower";
import { SingleEquipmentInputSaveState } from "../../types/EquipmentInput";
import { StatWeightsData } from "../../page/StatWeightsResult";
import { TextDictionary } from "../../const/languageTexts";

const STAT_SUMMARY_BOX_WIDTH = "3vw";
const STAT_SUMMARY_TIME_BOX_WIDTH = "6vw";
const POWER_SUMMARY_BOX_WIDTH = "6vw";
const SUMMARY_FONT_SIZE = "0.9vw";

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
    <Typography variant="body1" align="center" fontSize={SUMMARY_FONT_SIZE}>
      {text}
    </Typography>
  );
}

export function StatSummaryTypographyCompare(
  stat: string,
  compareStat: string,
  statName: string,
  gcdName: string
) {
  let statValue = parseFloat(stat);
  let compareStatValue = parseFloat(compareStat);

  let isBetter = statName === gcdName ? statValue < compareStatValue : statValue > compareStatValue;
  let isWorse = statName === gcdName ? statValue > compareStatValue : statValue < compareStatValue;

  let color =
    isBetter
      ? AppConfigurations.secondary
      : isWorse
        ? AppConfigurations.alert
        : "white";

  return (
    <Typography variant="body1" align="center" fontSize={SUMMARY_FONT_SIZE} color={color}>
      {stat}
    </Typography>
  );
}

export function SimulationInputSummary(
  power: PlayerPower,
  jobAbbrev: string,
  combatTimeMilliseconds: number,
  iterationCount: number,
  variancePercent: number,
  LANGUAGE_TEXTS: TextDictionary
) {
  let simulationInputNames = getStatNames(jobAbbrev, LANGUAGE_TEXTS.GCD_NAME);

  return (
    <Box display="inline-block">
      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundThree }}
      >
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(LANGUAGE_TEXTS.TIME_TEXT)}
        </SingleStatCombatTimeBox>
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(LANGUAGE_TEXTS.ITERATION_NAME)}
        </SingleStatCombatTimeBox>
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(LANGUAGE_TEXTS.VARIANCE_NAME)}
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
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(`${combatTimeMilliseconds.toFixed(0)}`)}
        </SingleStatCombatTimeBox>
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(iterationCount.toString())}
        </SingleStatCombatTimeBox>
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(`${variancePercent.toFixed(1)}%`)}
        </SingleStatCombatTimeBox>
        {simulationInputNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(
                getStatByStatName(power, statName, jobAbbrev, LANGUAGE_TEXTS.GCD_NAME)
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
  combatTimeMilliseconds: number,
  iterationCount: number,
  variancePercent: number,
  LANGUAGE_TEXTS: TextDictionary
) {
  let jobStatNames = getStatNames(jobAbbrev, LANGUAGE_TEXTS.GCD_NAME);

  return (
    <Box display="inline-block">
      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundThree }}
      >
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(LANGUAGE_TEXTS.TIME_TEXT)}
        </SingleStatCombatTimeBox>
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(LANGUAGE_TEXTS.ITERATION_NAME)}
        </SingleStatCombatTimeBox>
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(LANGUAGE_TEXTS.VARIANCE_NAME)}
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
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(`${combatTimeMilliseconds / 1000}`)}
        </SingleStatCombatTimeBox>
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(iterationCount.toString())}
        </SingleStatCombatTimeBox>
        <SingleStatCombatTimeBox>
          {StatSummaryTypography(`${variancePercent.toFixed(1)}%`)}
        </SingleStatCombatTimeBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypographyCompare(
                getStatByStatName(power, statName, jobAbbrev, LANGUAGE_TEXTS.GCD_NAME),
                getStatByStatName(comparePower, statName, jobAbbrev, LANGUAGE_TEXTS.GCD_NAME),
                statName,
                LANGUAGE_TEXTS.GCD_NAME
              )}
            </SingleStatBox>
          );
        })}
      </StatSummaryBox>
    </Box>
  );
}

export function StatSummary(totalState: SingleEquipmentInputSaveState, LANGUAGE_TEXTS: TextDictionary) {
  let jobStatNames = getStatNames(totalState.mainPlayerJobAbbrev, LANGUAGE_TEXTS.GCD_NAME);
  let power = totalState.power;
  let jobAbbrev = totalState.mainPlayerJobAbbrev;

  return (
    <Box display="flex" flexDirection={"column"}>
      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundThree }}
      >
        <SingleStatBox>{StatSummaryTypography(LANGUAGE_TEXTS.NAME_TEXT)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>{StatSummaryTypography(statName)}</SingleStatBox>
          );
        })}
        <SingleStatBox>{StatSummaryTypography(LANGUAGE_TEXTS.COMPOSITION_NAME)}</SingleStatBox>
      </StatSummaryBox>

      <StatSummaryBox
        sx={{
          backgroundColor: AppConfigurations.backgroundFour,
        }}
      >
        <SingleStatBox>{StatSummaryTypography(LANGUAGE_TEXTS.STAT_TEXT)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(
                getStatByStatName(power, statName, jobAbbrev, LANGUAGE_TEXTS.GCD_NAME)
              )}
            </SingleStatBox>
          );
        })}
        <SingleStatBox>{StatSummaryTypography(`${totalState.compositionBuffPercent}%`)}</SingleStatBox>
      </StatSummaryBox>

      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundFour }}
      >
        <SingleStatBox>{StatSummaryTypography(LANGUAGE_TEXTS.PREV_TEXT)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          let lostStat = getStatLostByStatName(power, statName, jobAbbrev, LANGUAGE_TEXTS.GCD_NAME);
          let color =
            lostStat === 0 ? AppConfigurations.secondary : minusBackgroundColor;
          return (
            <SingleStatBox>
              <Typography
                variant="body1"
                color={color}
                align="center"
                fontSize={SUMMARY_FONT_SIZE}
              >
                <b>{lostStat}</b>
              </Typography>

            </SingleStatBox>
          );
        })}
        <SingleStatBox>{StatSummaryTypography("0")}</SingleStatBox>
      </StatSummaryBox>

      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundFour }}
      >
        <SingleStatBox>{StatSummaryTypography(LANGUAGE_TEXTS.NEXT_TEXT)}</SingleStatBox>
        {jobStatNames.map((statName) => {
          return (
            <SingleStatBox>
              {StatSummaryTypography(
                getStatNeededByStatName(power, statName, jobAbbrev, LANGUAGE_TEXTS.GCD_NAME).toString()
              )}
            </SingleStatBox>
          );
        })}
        <SingleStatBox>{StatSummaryTypography("0")}</SingleStatBox>
      </StatSummaryBox>
    </Box>
  );
}

export function StatWeightSummary(statWeightsData: StatWeightsData[], LANGUAGE_TEXTS: TextDictionary) {
  return (
    <Box display="inline-block">
      <Box display="flex">
        <StatSummaryBox
          sx={{ backgroundColor: AppConfigurations.backgroundThree }}
        >
          <SingleStatBox>{StatSummaryTypography(LANGUAGE_TEXTS.STAT_TEXT)}</SingleStatBox>
        </StatSummaryBox>
        <StatSummaryBox
          sx={{ backgroundColor: AppConfigurations.backgroundThree }}
        >
          <SingleStatBox sx={{ width: "15vw" }}>{StatSummaryTypography(LANGUAGE_TEXTS.BEST_STATS_NAME_TEXT)}</SingleStatBox>
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
              sx={{ backgroundColor: AppConfigurations.backgroundFour, width: "15vw" }}
            >
              <SingleStatBox sx={{ width: "15vw" }}>
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

export function StatPowerSummary(totalState: SingleEquipmentInputSaveState, LANGUAGE_TEXTS: TextDictionary) {
  let power = totalState.power;
  let { POWER_NAMES } = loadPowerNames(LANGUAGE_TEXTS);

  return (
    <Box display="inline-block">
      <StatSummaryBox
        sx={{ backgroundColor: AppConfigurations.backgroundThree }}
      >
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
        {POWER_NAMES.map((powerName) => {
          return (
            <SinglePowerBox>
              {StatSummaryTypography(getStatPower(power, powerName, LANGUAGE_TEXTS))}
            </SinglePowerBox>
          );
        })}
      </StatSummaryBox>
    </Box>
  );
}
