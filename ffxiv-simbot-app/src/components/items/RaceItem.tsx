import { Box, Typography } from "@mui/material";
import { getMainStatOfRace, getMainStatNameByJob } from "../../const/StartStats";

export function RaceItem(race: string, jobAbbrev: string) {
  let mainStatName = getMainStatNameByJob(jobAbbrev);
  return (
    <Box display="flex" alignContent={"left"}>
      <Box marginRight={1}>
        <Typography variant="body1" color="white">
          {race}
        </Typography>
        <Typography variant="body2" color="white">
          {`${mainStatName} +${getMainStatOfRace(race, mainStatName)}`}
        </Typography>
      </Box>
    </Box>
  );
}
