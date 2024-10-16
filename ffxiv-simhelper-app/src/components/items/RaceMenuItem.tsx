import { MenuItem } from "@mui/material";
import { RaceItem } from "./RaceItem";
import { TextDictionary } from "../../const/languageTexts";

export function RaceMenuItem(race: string, jobAbbrev: string, LANGUAGE_TEXTS: TextDictionary) {
  return (
    <MenuItem value={race} key={`${race}_menuitem`}>
      {RaceItem(race, jobAbbrev, LANGUAGE_TEXTS)}
    </MenuItem>
  );
}
