import { Box, Typography } from "@mui/material";
import { getMainStatOfRace, getMainStatNameByJob } from "../../const/StartStats";
import { ITEM_TOP_MENU_MIN_HEIGHT } from "./Styles";

export function RaceItem(race: string, jobAbbrev: string) {
  let mainStatName = getMainStatNameByJob(jobAbbrev);
  let titleFontSize = {
    xs: 10,
    sm: 12,
    md: 14,
    lg: 16,
    xl: 18
  }

  let descriptionFontSize = {
    xs: 8,
    sm: 10,
    md: 10,
    lg: 12,
    xl: 12
  }


  return (
    <Box display="flex" alignItems={"center"} height={ITEM_TOP_MENU_MIN_HEIGHT}>
      <Box>
        <Typography variant="body1" color="white" sx={{ fontSize: titleFontSize }}>
          {race}
        </Typography>
        <Typography variant="body2" color="white" sx={{ fontSize: descriptionFontSize }} align="left">
          {`${mainStatName} +${getMainStatOfRace(race, mainStatName)}`}
        </Typography>
      </Box>
    </Box>
  );
}
