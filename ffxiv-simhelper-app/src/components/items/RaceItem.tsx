import { Box, Typography } from "@mui/material";
import { getMainStatOfRace, getMainStatNameByJob, getMainStatKeyByJob } from "../../const/StartStats";
import { ITEM_TOP_MENU_MIN_HEIGHT } from "./Styles";
import { AppConfigurations } from "../../Themes";
import { convertToRaceTextName, TextDictionary } from "../../const/languageTexts";

export function RaceItem(race: string, jobAbbrev: string, LANGUAGE_TEXTS: TextDictionary) {
  let mainStatName = getMainStatNameByJob(jobAbbrev, LANGUAGE_TEXTS);
  let mainStatKey = getMainStatKeyByJob(jobAbbrev);

  return (
    <Box display="flex" alignItems={"center"} height={ITEM_TOP_MENU_MIN_HEIGHT}>
      <Box>
        <Typography variant="body1" color="white" sx={{ fontSize: AppConfigurations.body1FontSize }}>
          {convertToRaceTextName(race, LANGUAGE_TEXTS)}
        </Typography>
        <Typography variant="body2" color="white" sx={{ fontSize: AppConfigurations.body2FontSize }} align="left">
          {`${mainStatName} +${getMainStatOfRace(race, mainStatKey)}`}
        </Typography>
      </Box>
    </Box>
  );
}
