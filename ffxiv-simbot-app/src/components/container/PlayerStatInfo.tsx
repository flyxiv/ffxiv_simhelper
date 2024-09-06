import { Box, Grid, styled, Typography } from "@mui/material";
import {
  PlayerStatInfoBoxStyle,
  StatBoxStyle,
  StatTextBoxStyle,
  StatTitleTextBoxStyle,
} from "./Styles";
import {
  getSpeedStatByJobAbbrev,
  PlayerPower,
} from "../../types/ffxivdatabase/PlayerPower";
import { isCaster } from "../../types/ffxivdatabase/ItemSet";
import { ColorConfigurations } from "../../Themes";

const PlayerStatInfoBox = styled(Box)`
  ${PlayerStatInfoBoxStyle}
`;

const StatBox = styled(Box)`
  ${StatBoxStyle}
`;

const StatTitleBox = styled(Box)`
  ${StatTitleTextBoxStyle}
`;

const StatTextBox = styled(Box)`
  ${StatTextBoxStyle}
`;

export function PlayerStatInfo(
  statInfo: PlayerPower,
  jobAbbrev: string,
  combatTimeMilliseconds: number
) {
  let speed = isCaster(jobAbbrev) ? statInfo.spellSpeed : statInfo.skillSpeed;

  return (
    <PlayerStatInfoBox>
      <Grid container spacing={0}>
        {StatOutput("Weapon Damage", statInfo.weaponDamage)}
        {StatOutput("Main Stat", statInfo.mainStat)}
        {StatOutput("Critical Strike", statInfo.criticalStrike)}
        {StatOutput("Direct Hit", statInfo.directHit)}
        {StatOutput("Determination", statInfo.determination)}
        {StatOutput("Speed", speed)}
        {StatOutput("Tenacity", statInfo.tenacity)}
        {StatOutput("Time(Seconds)", Math.floor(combatTimeMilliseconds / 1000))}
      </Grid>
    </PlayerStatInfoBox>
  );
}

export function StatOutput(statName: string, statValue: number) {
  return (
    <Grid item xs={3}>
      <StatBox>
        <StatTitleBox>
          <Typography
            variant="body2"
            component="div"
            fontSize={11}
            align="center"
          >
            {statName}
          </Typography>
        </StatTitleBox>
        <StatTextBox>
          <Typography variant="body1" align="center">
            {statValue}
          </Typography>
        </StatTextBox>
      </StatBox>
    </Grid>
  );
}

export function StatComparePlayerStatInfo(
  targetStat: PlayerPower,
  compareStat: PlayerPower,
  jobAbbrev: string,
  combatTimeMilliseconds: number
) {
  let weaponDamageColor = "black";
  let mainStatColor = "black";
  let criticalStrikeColor = "black";
  let directHitColor = "black";
  let determinationColor = "black";
  let speedColor = "black";
  let tenacityColor = "black";

  if (targetStat.weaponDamage > compareStat.weaponDamage) {
    weaponDamageColor = ColorConfigurations.primary;
  } else if (targetStat.weaponDamage < compareStat.weaponDamage) {
    weaponDamageColor = "red";
  }

  if (targetStat.mainStat > compareStat.mainStat) {
    mainStatColor = "blue";
  } else if (targetStat.mainStat < compareStat.mainStat) {
    mainStatColor = "red";
  }

  if (targetStat.criticalStrike > compareStat.criticalStrike) {
    criticalStrikeColor = "blue";
  } else if (targetStat.criticalStrike < compareStat.criticalStrike) {
    criticalStrikeColor = "red";
  }

  if (targetStat.directHit > compareStat.directHit) {
    directHitColor = "blue";
  } else if (targetStat.directHit < compareStat.directHit) {
    directHitColor = "red";
  }

  if (targetStat.determination > compareStat.determination) {
    determinationColor = "blue";
  } else if (targetStat.determination < compareStat.determination) {
    determinationColor = "red";
  }

  let targetSpeed = getSpeedStatByJobAbbrev(targetStat, jobAbbrev);
  let compareSpeed = getSpeedStatByJobAbbrev(compareStat, jobAbbrev);

  if (targetSpeed > compareSpeed) {
    speedColor = "blue";
  } else if (targetSpeed < compareSpeed) {
    speedColor = "red";
  }

  if (targetStat.tenacity > compareStat.tenacity) {
    tenacityColor = "blue";
  } else if (targetStat.tenacity < compareStat.tenacity) {
    tenacityColor = "red";
  }

  return (
    <PlayerStatInfoBox>
      <Grid container spacing={0}>
        {StatCompareStatOutput(
          "Weapon Damage",
          targetStat.weaponDamage,
          weaponDamageColor
        )}
        {StatCompareStatOutput("Main Stat", targetStat.mainStat, mainStatColor)}
        {StatCompareStatOutput(
          "Critical Strike",
          targetStat.criticalStrike,
          criticalStrikeColor
        )}
        {StatCompareStatOutput(
          "Direct Hit",
          targetStat.directHit,
          directHitColor
        )}
        {StatCompareStatOutput(
          "Determination",
          targetStat.determination,
          determinationColor
        )}
        {StatCompareStatOutput("Speed", targetSpeed, speedColor)}
        {StatCompareStatOutput("Tenacity", targetStat.tenacity, tenacityColor)}
        {StatOutput("Time(Seconds)", Math.floor(combatTimeMilliseconds / 1000))}
      </Grid>
    </PlayerStatInfoBox>
  );
}

export function StatCompareStatOutput(
  statName: string,
  statValue: number,
  statColor: string
) {
  let style = { color: statColor, fontWeight: "normal" };
  if (statColor !== "black") {
    style = { color: statColor, fontWeight: "bold" };
  }

  return (
    <Grid item xs={3}>
      <StatBox>
        <StatTitleBox>
          <Typography variant="body2" component="div" fontSize={11}>
            {statName}
          </Typography>
        </StatTitleBox>
        <StatTextBox>
          <Typography variant="body1" style={style}>
            {statValue}
          </Typography>
        </StatTextBox>
      </StatBox>
    </Grid>
  );
}
