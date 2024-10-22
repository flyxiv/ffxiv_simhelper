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
import { isMobile } from "../../util";


const STAT_SUMMARY_BOX_WIDTH = "50px";
const STAT_SUMMARY_TIME_BOX_WIDTH = "65px";
const POWER_SUMMARY_BOX_WIDTH = "70px";
const STAT_WEIGHT_SUMMARY_BOX_WIDTH = "200px";
const SUMMARY_FONT_SIZE = { xs: 11, sm: 13 };

let StatSummaryBox = styled(Box)`
  ${StatSummaryBoxStyle}
`;

let SinglePowerBox = styled(Box)`
  ${SingleStatBoxStyle(POWER_SUMMARY_BOX_WIDTH, 70)}
`;

let SingleStatBox = styled(Box)`
  ${SingleStatBoxStyle(STAT_SUMMARY_BOX_WIDTH, 40)}
`;

let SingleStatCombatTimeBox = styled(Box)`
  ${SingleStatBoxStyle(STAT_SUMMARY_TIME_BOX_WIDTH, 85)}
`;

let SingleStatWeightBox = styled(Box)`
  ${SingleStatBoxStyle(STAT_WEIGHT_SUMMARY_BOX_WIDTH, 200)}
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
  LANGUAGE_TEXTS: TextDictionary,
  showStats: boolean
) {
  let simulationInputNames = [LANGUAGE_TEXTS.GCD_NAME];
  if (showStats) {
    simulationInputNames = getStatNames(jobAbbrev, LANGUAGE_TEXTS.GCD_NAME);
  }

  return (
    <>
      <Box display="flex">
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
        </StatSummaryBox>
      </Box>
      <Box display="flex">
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
        </StatSummaryBox>
      </Box>
      <Box display="flex">
        <StatSummaryBox
          sx={{
            backgroundColor: AppConfigurations.backgroundThree,
          }}
        >

          {simulationInputNames.map((statName) => {
            return (
              <SingleStatBox>{StatSummaryTypography(statName)}</SingleStatBox>
            );
          })}
        </StatSummaryBox>
      </Box>
      <Box display="flex">
        <StatSummaryBox
          sx={{
            backgroundColor: AppConfigurations.backgroundFour,
          }}
        >
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
    </>
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
    <Box>
      <Box display="flex">
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
        </StatSummaryBox>
      </Box>

      <Box display="flex">
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
        </StatSummaryBox>
      </Box>

      <Box display="flex">
        <StatSummaryBox
          sx={{
            backgroundColor: AppConfigurations.backgroundThree,
          }}
        >
          {jobStatNames.map((statName) => {
            return (
              <SingleStatBox>{StatSummaryTypography(statName)}</SingleStatBox>
            );
          })}
        </StatSummaryBox>
      </Box>

      <Box display="flex">
        <StatSummaryBox
          sx={{
            backgroundColor: AppConfigurations.backgroundFour,
          }}
        >

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
    </Box >
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
    <Box>
      <Box display="flex">
        <StatSummaryBox
          sx={{ backgroundColor: AppConfigurations.backgroundThree }}
        >
          <SingleStatBox>{StatSummaryTypography(LANGUAGE_TEXTS.STAT_TEXT)}</SingleStatBox>
        </StatSummaryBox>
        <StatSummaryBox
          sx={{ backgroundColor: AppConfigurations.backgroundThree }}
        >
          <SingleStatWeightBox>{StatSummaryTypography(LANGUAGE_TEXTS.BEST_STATS_NAME_TEXT)}</SingleStatWeightBox>
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
              <SingleStatWeightBox>
                {StatSummaryTypography(
                  `${statWeightData.rdpsIncreasePerPoint.toFixed(2)}`
                )}
              </SingleStatWeightBox>
            </StatSummaryBox>
          </Box>
        );
      })}
    </Box>
  );
}

export function StatPowerSummary(totalState: SingleEquipmentInputSaveState, LANGUAGE_TEXTS: TextDictionary) {
  let { POWER_NAMES } = loadPowerNames(LANGUAGE_TEXTS);
  if (isMobile()) {
    return (
      <Box width="100%" display="flex" justifyContent="center" flexDirection="column">
        {StatPowerSummaryInputPower(POWER_NAMES.slice(0, 5), totalState, LANGUAGE_TEXTS)}
        {StatPowerSummaryInputPower(POWER_NAMES.slice(5), totalState, LANGUAGE_TEXTS)}
      </Box>
    );
  }
  else {
    return StatPowerSummaryInputPower(POWER_NAMES, totalState, LANGUAGE_TEXTS)
  }
}


function StatPowerSummaryInputPower(powerNames: Array<string>, totalState: SingleEquipmentInputSaveState, LANGUAGE_TEXTS: TextDictionary) {
  let power = totalState.power;

  return (
    <Box>
      <Box display="flex" justifyContent="center">
        <StatSummaryBox
          sx={{ backgroundColor: AppConfigurations.backgroundThree }}
        >
          {powerNames.map((powerName) => {
            return (
              <SinglePowerBox>{StatSummaryTypography(powerName)}</SinglePowerBox>
            );
          })}
        </StatSummaryBox>
      </Box>

      <Box display="flex" justifyContent="center">

        <StatSummaryBox
          sx={{
            backgroundColor: AppConfigurations.backgroundFour,
          }}
        >
          {powerNames.map((powerName) => {
            return (
              <SinglePowerBox>
                {StatSummaryTypography(getStatPower(power, powerName, LANGUAGE_TEXTS))}
              </SinglePowerBox>
            );
          })}
        </StatSummaryBox>
      </Box>
    </Box>
  );
}
