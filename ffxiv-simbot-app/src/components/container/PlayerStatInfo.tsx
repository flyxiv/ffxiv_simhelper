import { Box, Grid, styled, Typography } from "@mui/material";
import {
  PlayerStatInfoBoxStyle,
  StatBoxStyle,
  StatTextBoxStyle,
  StatTitleTextBoxStyle,
} from "./Styles";

const StatBox = styled(Box)`
  ${StatBoxStyle}
`;

const StatTitleBox = styled(Box)`
  ${StatTitleTextBoxStyle}
`;

const StatTextBox = styled(Box)`
  ${StatTextBoxStyle}
`;

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
