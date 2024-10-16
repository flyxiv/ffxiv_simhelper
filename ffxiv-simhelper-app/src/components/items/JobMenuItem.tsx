import { MenuItem } from "@mui/material";
import { JobItem } from "./JobItem";
import { TextDictionary } from "../../const/languageTexts";

// Resembles one of the job options in the job selection dropdown menu.
export function JobMenuItem(jobAbbrev: string, align: string, LANGUAGE_TEXTS: TextDictionary, is_top: boolean = true) {
  return <MenuItem value={jobAbbrev}>{JobItem(jobAbbrev, align, LANGUAGE_TEXTS, is_top)}</MenuItem>;
}
