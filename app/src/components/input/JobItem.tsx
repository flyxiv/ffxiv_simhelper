import { Box } from "@mui/material";
import { jobAbbrevToJobIconPath } from "../jobicon/JobIconFactory";
import { Typography } from "mui-core";

export function JobItem(jobAbbrev: string) {
  return (
    <Box display="flex" justifyContent="center">
      <Box marginRight={1}>
        <img src={jobAbbrevToJobIconPath(jobAbbrev)} width={25} height={25} />
      </Box>
      <Typography variant="body1">{jobAbbrev}</Typography>
    </Box>
  );
}
