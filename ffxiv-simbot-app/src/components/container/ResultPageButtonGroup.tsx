import { Box, styled, ToggleButton, ToggleButtonGroup } from "@mui/material";
import { ColorConfigurations } from "../../Themes";
import { PartyContributionData } from "../graph/GraphData";
import {
  BEST_TEAMMATE_TEXT,
  DAMAGE_PROFILE_TEXT,
  MY_CONTRIBUTIONS_TEXT,
  ROTATION_LOG_TEXT,
} from "../../page/SimulationResult";

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
  teammatesContributionToMyBuffs: PartyContributionData | null,
  mainPlayerContributionToOthers: PartyContributionData | null
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
    <ToggleButtonCustom value={BEST_TEAMMATE_TEXT}>
      {BEST_TEAMMATE_TEXT}
    </ToggleButtonCustom>
  );

  let myContributionsButton = (
    <ToggleButtonCustom value={MY_CONTRIBUTIONS_TEXT}>
      {MY_CONTRIBUTIONS_TEXT}
    </ToggleButtonCustom>
  );

  return (
    <Box marginTop={4} marginBottom={2}>
      <ToggleButtonGroup
        value={currentlyToggledView}
        exclusive
        onChange={handleViewChange}
        aria-label="resultPage"
      >
        <ToggleButtonCustom value="Damage Profile">
          {DAMAGE_PROFILE_TEXT}
        </ToggleButtonCustom>
        {teammatesContributionToMyBuffs === null ||
        teammatesContributionToMyBuffs.contributionData.length > 0 ? (
          bestTeammateButton
        ) : (
          <Box />
        )}
        {mainPlayerContributionToOthers === null ||
        mainPlayerContributionToOthers.contributionData.length > 0 ? (
          myContributionsButton
        ) : (
          <Box />
        )}
        <ToggleButtonCustom value="Rotation Log">
          {ROTATION_LOG_TEXT}
        </ToggleButtonCustom>
      </ToggleButtonGroup>
    </Box>
  );
}
