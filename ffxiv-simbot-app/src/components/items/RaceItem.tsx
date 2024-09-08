import { Box, Typography } from "@mui/material";
import { getMainStatOfRace, getMainStatNameByJob } from "../../const/StartStats";
import { ITEM_TOP_MENU_MIN_HEIGHT } from "./Styles";
import { convertToRaceText } from "../../const/languageTexts";
import { AppConfigurations } from "../../Themes";

export function RaceItem(race: string, jobAbbrev: string) {
  let mainStatName = getMainStatNameByJob(jobAbbrev);

  return (
    <Box display="flex" alignItems={"center"} height={ITEM_TOP_MENU_MIN_HEIGHT}>
      <Box>
        <Typography variant="body1" color="white" sx={{ fontSize: AppConfigurations.body1FontSize }}>
          {convertToRaceText(race)}
        </Typography>
        <Typography variant="body2" color="white" sx={{ fontSize: AppConfigurations.body2FontSize }} align="left">
          {`${mainStatName} +${getMainStatOfRace(race, mainStatName)}`}
        </Typography>
      </Box>
    </Box>
  );
}
