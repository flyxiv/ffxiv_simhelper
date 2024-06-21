import { Typography, Box, styled } from "@mui/material";
import { DpsBoxStyle, DpsSummaryBoxStyle } from "./Styles";
import { SimulationData } from "src/types/QuickSimResponse";
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
