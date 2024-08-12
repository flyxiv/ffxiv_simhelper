import { MenuItem } from "@mui/material";
import { RaceItem } from "./RaceItem";

export function RaceMenuItem(race: string, jobAbbrev: string) {
  return <MenuItem value={race}>{RaceItem(race, jobAbbrev)}</MenuItem>;
}
