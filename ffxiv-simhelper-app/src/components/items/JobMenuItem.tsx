import { MenuItem } from "@mui/material";
import { JobItem } from "./JobItem";

export function JobMenuItem(jobAbbrev: string, align: string, is_top: boolean = true) {
  return <MenuItem value={jobAbbrev}>{JobItem(jobAbbrev, align, is_top)}</MenuItem>;
}
