import { Typography, Box, styled } from "@mui/material";
import {
  plusBackgroundColor,
  plusTextColor,
  minusBackgroundColor,
  minusTextColor,
  DpsBoxStyle,
  DpsSummaryBoxStyle,
} from "./Styles";
import {
  SimulationData,
  SimulationSummary,
} from "src/types/CombatSimulationResult";
import { ColorConfigurations } from "src/Themes";

const DpsSummaryBox = styled(Box)`
  ${DpsSummaryBoxStyle}
`;

function DpsBox(
  dpsName: string,
  dps: number,
  color: string,
  backgroundColor: string
) {
  let DpsBox = styled(Box)`
    ${DpsBoxStyle}
    background-color: ${backgroundColor};
    color: ${color};
  `;

  return (
    <DpsBox>
      <Box className="DpsTitle" padding={1}>
        <Typography variant="h5">{dpsName}</Typography>
      </Box>
      <Box className="Dps" paddingTop={1}>
        <Typography variant="h3">{Math.round(dps)}</Typography>
      </Box>
    </DpsBox>
  );
}

export const DpsSummary = (props: SimulationData) => {
  const summary = props.simulationSummary;

  return (
    <DpsSummaryBox>
      {DpsBox("DPS", summary.pdps, "white", ColorConfigurations.primary)}
      {DpsBox(
        "RDPS",
        summary.rdps,
        "white",
        ColorConfigurations.primaryVariant
      )}
      {DpsBox("ADPS", summary.adps, "black", ColorConfigurations.secondary)}
      {DpsBox(
        "EDPS",
        summary.edps,
        "white",
        ColorConfigurations.secondaryVariant
      )}
    </DpsSummaryBox>
  );
};

export const StatCompareDpsBox = (
  dpsName: string,
  targetDps: number,
  compareDps: number
) => {
  let dpsCompare = targetDps - compareDps;
  let smallerDps = Math.min(targetDps, compareDps);
  let differencePercentPoint = Math.abs(dpsCompare / smallerDps) * 100;
  let differencePercentPointRounded =
    Math.round(differencePercentPoint * 10) / 10;

  let backgroundColor;
  let percentColor;

  if (dpsCompare > 0) {
    backgroundColor = plusBackgroundColor;
    percentColor = plusTextColor;
  } else if (dpsCompare < 0) {
    backgroundColor = minusBackgroundColor;
    percentColor = minusTextColor;
    differencePercentPointRounded = -differencePercentPointRounded;
  } else {
    backgroundColor = "white";
    percentColor = "black";
  }

  let differencePercentPointString = differencePercentPointRounded.toString();
  if (differencePercentPointRounded > 0) {
    differencePercentPointString = "+" + differencePercentPointString;
  }

  let DpsBox = styled(Box)`
    ${DpsBoxStyle}
    background-color: ${backgroundColor};
    color: white;
  `;

  return (
    <DpsBox>
      <Box className="DpsTitle" padding={1}>
        <Typography variant="h5">{dpsName}</Typography>
      </Box>
      <Box className="Dps">
        <Typography variant="h3">{Math.round(targetDps)}</Typography>
      </Box>
      <Box className="DpsCompare">
        <Typography
          variant="h5"
          fontSize={14}
          color={percentColor}
          fontWeight="bold"
        >
          {differencePercentPointString}%
        </Typography>
      </Box>
    </DpsBox>
  );
};

export const StatCompareDpsSummary = (
  target: SimulationSummary,
  compare: SimulationSummary
) => {
  return (
    <DpsSummaryBox>
      {StatCompareDpsBox("DPS", target.pdps, compare.pdps)}
      {StatCompareDpsBox("RDPS", target.rdps, compare.rdps)}
      {StatCompareDpsBox("EDPS", target.edps, compare.edps)}
    </DpsSummaryBox>
  );
};
