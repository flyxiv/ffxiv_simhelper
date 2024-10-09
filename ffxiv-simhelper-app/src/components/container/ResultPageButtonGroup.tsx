import { Box, styled, ToggleButton, ToggleButtonGroup } from "@mui/material";
import { AppConfigurations } from "../../Themes";
import { PartyContributionData } from "../graph/GraphData";
import { BEST_TEAMMATE_BUTTON_TEXT, DAMAGE_PROFILE_BUTTON_TEXT, MY_CONTRIBUTION_BUTTON_TEXT, ROTATION_SAMPLE_BUTTON_TEXT } from "../../const/languageTexts";

type ViewType = "Best Teammate" | "Damage Profile" | "Rotation Log"; // Define possible view types

const ToggleButtonCustomStyle = {
  "&.Mui-selected, &.Mui-selected:hover": {
    color: "white",
    backgroundColor: AppConfigurations.primary,
  },

  "&:hover": {
    color: "white",
    backgroundColor: AppConfigurations.primary,
  },

  color: AppConfigurations.primary,
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
    <ToggleButtonCustom value={BEST_TEAMMATE_BUTTON_TEXT}>
      {BEST_TEAMMATE_BUTTON_TEXT}
    </ToggleButtonCustom>
  );

  let myContributionsButton = (
    <ToggleButtonCustom value={MY_CONTRIBUTION_BUTTON_TEXT}>
      {MY_CONTRIBUTION_BUTTON_TEXT}
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
        <ToggleButtonCustom value={DAMAGE_PROFILE_BUTTON_TEXT}>
          {DAMAGE_PROFILE_BUTTON_TEXT}
        </ToggleButtonCustom>
        {isValidContributions(teammatesContributionToMyBuffs) ? (
          bestTeammateButton
        ) : (
          <Box />
        )}
        {isValidContributions(mainPlayerContributionToOthers) ? (
          myContributionsButton
        ) : (
          <Box />
        )}
        <ToggleButtonCustom value={ROTATION_SAMPLE_BUTTON_TEXT}>
          {ROTATION_SAMPLE_BUTTON_TEXT}
        </ToggleButtonCustom>
      </ToggleButtonGroup>
    </Box>
  );
}

function isValidContributions(
  contributionData: PartyContributionData | null,
) {
  if (contributionData === null) {
    return false;
  }

  for (let data of contributionData.contributionData) {
    if (data.totalRdps > 0) {
      return true;
    }
  }

  return false;
}
