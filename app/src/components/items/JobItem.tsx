import { Box, Typography } from "@mui/material";
import { jobAbbrevToJobIconPath } from "../icon/jobicon/JobIconFactory";

// Container item of job icon at the left and job abbreviation at the right
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
        <Typography variant="body1" alignContent={"center"} color="white">
          {jobAbbrev}
        </Typography>
      </Box>
    </Box>
  );
}
