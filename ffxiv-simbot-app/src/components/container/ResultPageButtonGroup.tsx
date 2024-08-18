import { Box, styled, ToggleButton, ToggleButtonGroup } from "@mui/material";
import { ColorConfigurations } from "../../Themes";

type ViewType = "Best Teammate" | "Damage Profile" | "Rotation Log"; // Define possible view types

const ToggleButtonCustomStyle = {
  "&.Mui-selected, &.Mui-selected:hover": {
    color: "white",
    backgroundColor: ColorConfigurations.primary,
  },

  "&:hover": {
    color: "white",
    backgroundColor: ColorConfigurations.primary,
  },

  color: ColorConfigurations.primary,
  backgroundColor: "white",
};

const ToggleButtonCustom = styled(ToggleButton)(ToggleButtonCustomStyle);

export function ResultPageButtonGroup(
  currentlyToggledView: string,
  setCurrentlyToggledView: Function,
  mainPlayerJob: string
) {
  const handleViewChange = (
    _: React.MouseEvent<HTMLElement>,
    newView: ViewType | null
  ) => {
    if (newView !== null) {
      setCurrentlyToggledView(newView);
    }
  };

  let bestTeammateButton = (
    <ToggleButtonCustom value="Best Teammate">Best Teammate</ToggleButtonCustom>
  );

  if (isNotBuffJob(mainPlayerJob)) {
    bestTeammateButton = <></>;
  }

  return (
    <Box marginTop={4} marginBottom={2}>
      <ToggleButtonGroup
        value={currentlyToggledView}
        exclusive
        onChange={handleViewChange}
        aria-label="resultPage"
      >
        <ToggleButtonCustom value="Damage Profile">
          Damage Profile
        </ToggleButtonCustom>
        {bestTeammateButton}
        <ToggleButtonCustom value="Rotation Log">
          Rotation Log
        </ToggleButtonCustom>
      </ToggleButtonGroup>
    </Box>
  );
}

function isNotBuffJob(job: string) {
  return (
    job === "GNB" ||
    job === "DRK" ||
    job === "WAR" ||
    job === "PLD" ||
    job === "WHM" ||
    job === "SGE" ||
    job === "SAM" ||
    job === "MCH" ||
    job === "BLM"
  );
}
