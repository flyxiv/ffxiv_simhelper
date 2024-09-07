import { Box, Grid, styled, Typography } from "@mui/material";
import {
  PlayerStatInfoBoxStyle,
  StatBoxStyle,
  StatTextBoxStyle,
  StatTitleTextBoxStyle,
} from "./Styles";
import {
  PlayerPower,
} from "../../types/ffxivdatabase/PlayerPower";
import { isCaster } from "../../types/ffxivdatabase/ItemSet";

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
