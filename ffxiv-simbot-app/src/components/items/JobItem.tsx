import { Box, Typography } from "@mui/material";
import { jobAbbrevToJobIconPath } from "../icon/jobicon/JobIconFactory";
import { ITEM_BOTTOM_MENU_MIN_HEIGHT, ITEM_TOP_MENU_MIN_HEIGHT } from "./Styles";
import { convertToJobText } from "../../const/languageTexts";
import { AppConfigurations } from "../../Themes";

export function JobItem(jobAbbrev: string, align: string, is_top: boolean) {
  return (
    <Box display="flex" justifyContent={align} alignItems={"center"} height={is_top ? ITEM_TOP_MENU_MIN_HEIGHT : ITEM_BOTTOM_MENU_MIN_HEIGHT}>
      <Box marginRight={1}>
        <img
          src={jobAbbrevToJobIconPath(jobAbbrev)}
          alt={jobAbbrev}
          width={25}
          height={25}
          style={{ verticalAlign: "middle" }}
        />
      </Box>
      <Box>
        <Typography variant="body1" alignContent={"center"} color="white" sx={{ fontSize: AppConfigurations.body1FontSize }}>
          {convertToJobText(jobAbbrev)}
        </Typography>
      </Box>
    </Box>
  );
}
