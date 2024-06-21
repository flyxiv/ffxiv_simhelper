import { CharacterStats } from "src/types/QuickSimRequest";
import { Box, Grid, styled, Typography } from "@mui/material";
import {
  PlayerStatInfoBoxStyle,
  StatBoxStyle,
  StatOneLineBoxStyle,
  StatTextBoxStyle,
  StatTitleTextBoxStyle,
} from "./Styles";

const PlayerStatInfoBox = styled(Box)`
  ${PlayerStatInfoBoxStyle}
`;

const StatOneLineBox = styled(Box)`
  ${StatOneLineBoxStyle}
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

export function PlayerStatInfo(statInfo: CharacterStats) {
  return (
    <PlayerStatInfoBox>
      <Grid container spacing={0}>
        {StatOutput("Weapon Damage", statInfo.weaponDamage)}
        {StatOutput("Main Stat", statInfo.mainStat)}
        {StatOutput("Critical Strike", statInfo.criticalStrike)}
        {StatOutput("Direct Hit", statInfo.directHit)}
        {StatOutput("Determination", statInfo.determination)}
        {StatOutput("Speed", statInfo.speed)}
        {StatOutput("Tenacity", statInfo.tenacity)}
      </Grid>
    </PlayerStatInfoBox>
  );
}

export function StatOutput(statName: string, statValue: number) {
  return (
    <Grid item xs={3}>
      <StatBox>
        <StatTitleBox>
          <Typography variant="body2" component="div" fontSize={11}>
            {statName}
          </Typography>
        </StatTitleBox>
        <StatTextBox>
          <Typography variant="body1">{statValue}</Typography>
        </StatTextBox>
      </StatBox>
    </Grid>
  );
}
