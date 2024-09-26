import { MenuItem } from "@mui/material";
import { JobItem } from "./JobItem";

// Resembles one of the job options in the job selection dropdown menu.
export function JobMenuItem(jobAbbrev: string, align: string, is_top: boolean = true) {
  return <MenuItem value={jobAbbrev}>{JobItem(jobAbbrev, align, is_top)}</MenuItem>;
}
