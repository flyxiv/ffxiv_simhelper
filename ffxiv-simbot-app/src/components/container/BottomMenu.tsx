import { Box } from "@mui/material";
import { AppConfigurations } from "../../Themes";
import { StatSummary } from "./StatSummary";
import { EquipmentInput } from "../../types/EquipmentInput";
import { GearCompareRequestButton } from "../basic/GearCompareRequestButton";
import { BODY_WIDTH } from "../../App";
import { MENU_WIDTH } from "./LeftMenu";

export function BasicBottomMenu(
  totalState: EquipmentInput,
  buttonComponentFunction: Function
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
        {StatSummary(totalState.equipmentDatas[0])}
        <Box display="inline-block" margin="auto" paddingTop={2}>
          {buttonComponentFunction(totalState)}
        </Box>
      </Box>
    </Box>
  );
}

export function GearCompareBottomMenu(totalState: EquipmentInput) {
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
          {StatSummary(totalState.equipmentDatas[0])}
          {StatSummary(totalState.equipmentDatas[1])}
        </Box>

        <Box display="inline-block" margin="auto" paddingTop={2}>
          {GearCompareRequestButton(totalState)}
        </Box>
      </Box>
    </Box>
  );
}
