import { Box, Typography } from "@mui/material";
import { jobAbbrevToJobIconPath } from "../jobicon/JobIconFactory";

export function JobItem(jobAbbrev: string) {
  return (
    <Box display="flex" justifyContent="right" alignContent={"center"}>
      <Box marginRight={1}>
        <img
          src={jobAbbrevToJobIconPath(jobAbbrev)}
          alt={jobAbbrev}
          width={17}
          height={17}
          style={{ verticalAlign: "middle" }}
        />
      </Box>
      <Box>
        <Typography variant="body1" alignContent={"center"}>
          {jobAbbrev}
        </Typography>
      </Box>
    </Box>
  );
}
