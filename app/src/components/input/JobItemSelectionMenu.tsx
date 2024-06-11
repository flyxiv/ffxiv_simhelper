import { MenuItem } from "@mui/material";
import { JobItem } from "./JobItem";

export function JobItemSelectionMenu(jobAbbrev: string) {
  return <MenuItem value={jobAbbrev}>{JobItem(jobAbbrev)}</MenuItem>;
}
