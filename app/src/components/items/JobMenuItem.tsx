import { MenuItem } from "@mui/material";
import { JobItem } from "./JobItem";

export function JobMenuItem(jobAbbrev: string) {
  return <MenuItem value={jobAbbrev}>{JobItem(jobAbbrev)}</MenuItem>;
}
