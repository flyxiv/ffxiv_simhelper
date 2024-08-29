import { MenuItem } from "@mui/material";
import { JobItem } from "./JobItem";

export function JobMenuItem(jobAbbrev: string, align: string) {
  console.log(align);
  return <MenuItem value={jobAbbrev}>{JobItem(jobAbbrev, align)}</MenuItem>;
}
