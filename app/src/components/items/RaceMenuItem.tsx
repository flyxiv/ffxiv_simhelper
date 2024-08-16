import { MenuItem } from "@mui/material";
import { RaceItem } from "./RaceItem";

export function RaceMenuItem(race: string, jobAbbrev: string) {
  return (
    <MenuItem value={race} key={`${race}_menuitem`}>
      {RaceItem(race, jobAbbrev)}
    </MenuItem>
  );
}
