import { Box, Typography } from "@mui/material";
import { getMainStatOfRace, getMainStatNameByJob } from "../../const/StartStats";
import { ITEM_MIN_HEIGHT } from "./Styles";

export function RaceItem(race: string, jobAbbrev: string) {
  let mainStatName = getMainStatNameByJob(jobAbbrev);
  return (
    <Box display="flex" alignItems={"center"} height={ITEM_MIN_HEIGHT}>
      <Box>
        <Typography variant="body1" color="white" fontSize={12}>
          {race}
        </Typography>
        <Typography variant="body2" color="white" fontSize={10} align="left">
          {`${mainStatName} +${getMainStatOfRace(race, mainStatName)}`}
        </Typography>
      </Box>
    </Box>
  );
}
