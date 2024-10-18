import { Box } from "@mui/material";
import { AppConfigurations } from "../../Themes";
import { StatSummary } from "./StatSummary";
import { EquipmentInput } from "../../types/EquipmentInput";
import { GearCompareRequestButton } from "../basic/GearCompareRequestButton";
import { BODY_WIDTH } from "../../App";
import { TextDictionary } from "../../const/languageTexts";
import { isMobile } from "../../util";

export function BasicBottomMenu(
  totalState: EquipmentInput,
  buttonComponentFunction: Function,
  LANGUAGE_TEXTS: TextDictionary,
  needsGcdName: boolean = false
) {
  return (
    <Box
      sx={{
        position: "fixed",
        bottom: 0,
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
          {needsGcdName ? buttonComponentFunction(totalState, LANGUAGE_TEXTS.GCD_NAME) : buttonComponentFunction(totalState)}
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

        sx={{ width: "100%" }}
      >
        {isMobile() ? MobileStatSummaryMenu(totalState, LANGUAGE_TEXTS) : PCStatSummaryMenu(totalState, LANGUAGE_TEXTS)}

        <Box display="inline-block" margin="auto" paddingTop={2}>
          {GearCompareRequestButton(totalState)}
        </Box>
      </Box>
    </Box>
  );
}


function MobileStatSummaryMenu(totalState: EquipmentInput, LANGUAGE_TEXTS: TextDictionary) {
  return (
    <Box
      paddingY={3}
      display="flex"
      flexDirection="column"
      alignContent="center"
    >
      <Box
        display="flex"
        flexDirection={"column"}
        justifyContent={"center"}
        sx={{ width: BODY_WIDTH }}
      >
        <Box marginBottom={1}>
          {StatSummary(totalState.equipmentDatas[0], LANGUAGE_TEXTS)}
        </Box>
        <Box marginTop={1}>
          {StatSummary(totalState.equipmentDatas[1], LANGUAGE_TEXTS)}
        </Box>
      </Box>
    </Box>
  );
}

function PCStatSummaryMenu(totalState: EquipmentInput, LANGUAGE_TEXTS: TextDictionary) {
  return (
    <Box
      display="flex"
      flexDirection={"row"}
      justifyContent={"space-evenly"}
    >
      {StatSummary(totalState.equipmentDatas[0], LANGUAGE_TEXTS)}
      {StatSummary(totalState.equipmentDatas[1], LANGUAGE_TEXTS)}
    </Box>
  );
}
