import { MenuItem } from "@mui/material";
import { RaceItem } from "./RaceItem";
import { convertToEnglishRaceName } from "../../const/languageTexts";

export function RaceMenuItem(race: string, jobAbbrev: string) {
  return (
    <MenuItem value={race} key={`${race}_menuitem`}>
      {RaceItem(race, jobAbbrev)}
    </MenuItem>
  );
}
