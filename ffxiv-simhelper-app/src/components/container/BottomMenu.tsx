import { Box } from "@mui/material";
import { AppConfigurations } from "../../Themes";
import { StatSummary } from "./StatSummary";
import { EquipmentInput } from "../../types/EquipmentInput";
import { GearCompareRequestButton } from "../basic/GearCompareRequestButton";
import { BODY_WIDTH } from "../../App";
import { MENU_WIDTH } from "./LeftMenu";
import { TextDictionary } from "../../const/languageTexts";

export function BasicBottomMenu(
  totalState: EquipmentInput,
  buttonComponentFunction: Function,
  LANGUAGE_TEXTS: TextDictionary
) {
  return (
    <Box
      sx={{
        position: "fixed",
        bottom: 0,
        left: MENU_WIDTH,
        width: BODY_WIDTH,
        backgroundColor: AppConfigurations.backgroundTwo,
        display: "flex",
        justifyContent: "space-around",
        zIndex: 1000,
      }}
    >
      <Box
        paddingY={3}
        display="flex"
        flexDirection="column"
        sx={{ backgroundColor: AppConfigurations.backgroundTwo }}
      >
        {StatSummary(totalState.equipmentDatas[0], LANGUAGE_TEXTS)}
        <Box display="inline-block" margin="auto" paddingTop={2}>
          {buttonComponentFunction(totalState)}
        </Box>
      </Box>
    </Box>
  );
}

export function GearCompareBottomMenu(totalState: EquipmentInput, LANGUAGE_TEXTS: TextDictionary) {
  return (
    <Box
      sx={{
        position: "fixed",
        bottom: 0,
        left: MENU_WIDTH,
        width: BODY_WIDTH,
        backgroundColor: AppConfigurations.backgroundTwo,
        display: "flex",
        justifyContent: "space-around",
        alignItems: "center",
        zIndex: 1000,
      }}
    >
      <Box
        paddingY={3}
        display="flex"
        flexDirection="column"
        alignContent="center"
      >
        <Box
          display="flex"
          justifyContent="space-evenly"
          sx={{ width: BODY_WIDTH }}
        >
          {StatSummary(totalState.equipmentDatas[0], LANGUAGE_TEXTS)}
          {StatSummary(totalState.equipmentDatas[1], LANGUAGE_TEXTS)}
        </Box>

        <Box display="inline-block" margin="auto" paddingTop={2}>
          {GearCompareRequestButton(totalState)}
        </Box>
      </Box>
    </Box>
  );
}
