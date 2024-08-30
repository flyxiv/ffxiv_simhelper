import { MenuItem } from "@mui/material";
import { JobItem } from "./JobItem";

export function JobMenuItem(jobAbbrev: string, align: string) {
  return <MenuItem value={jobAbbrev}>{JobItem(jobAbbrev, align)}</MenuItem>;
}
