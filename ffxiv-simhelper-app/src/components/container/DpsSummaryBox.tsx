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
} from "../../types/CombatSimulationResult";
import { AppConfigurations } from "../../Themes";

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
      <Box
        className="DpsTitle"
        padding={1}
        sx={{
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
        }}
      >
        <Typography
          variant="h6"
          align="center"
          sx={{ fontSize: AppConfigurations.body2FontSize }}
        >
          {dpsName}
        </Typography>
      </Box>
      <Box className="Dps" paddingTop={1}>
        <Typography variant="h3" align="center">
          {Math.round(dps)}
        </Typography>
      </Box>
    </DpsBox>
  );
}

export const DpsSummary = (props: SimulationData) => {
  const summary = props.simulationSummary;

  return (
    <DpsSummaryBox>
      {DpsBox("DPS", summary.pdps[0], "white", AppConfigurations.primary)}
      {DpsBox(
        "RDPS",
        summary.rdps[0],
        "white",
        AppConfigurations.primaryVariant
      )}
      {DpsBox(
        "95% RDPS",
        summary.maxRdps[0],
        "black",
        AppConfigurations.secondary
      )}
      {DpsBox(
        "EDPS",
        summary.edps[0],
        "white",
        AppConfigurations.secondaryVariant
      )}
    </DpsSummaryBox>
  );
};

export const GearCompareDpsBox = (
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
        <Typography
          variant="h6"
          sx={{
            whiteSpace: "nowrap",
            overflow: "hidden",
            textOverflow: "ellipsis",
            fontSize: "1vw",
          }}
        >
          {dpsName}
        </Typography>
      </Box>
      <Box className="Dps">
        <Typography variant="h3">{Math.round(targetDps)}</Typography>
      </Box>
      <Box className="DpsCompare">
        <Typography
          variant="h5"
          fontSize={"1vw"}
          color={percentColor}
          fontWeight="bold"
        >
          {differencePercentPointString}%
        </Typography>
      </Box>
    </DpsBox>
  );
};

export const GearCompareDpsSummary = (
  target: SimulationSummary,
  compare: SimulationSummary
) => {
  return (
    <DpsSummaryBox>
      {GearCompareDpsBox("DPS", target.pdps[0], compare.pdps[0])}
      {GearCompareDpsBox("RDPS", target.rdps[0], compare.rdps[0])}
      {GearCompareDpsBox("EDPS", target.edps[0], compare.edps[0])}
    </DpsSummaryBox>
  );
};
