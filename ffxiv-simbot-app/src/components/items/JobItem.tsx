import { Box, Typography } from "@mui/material";
import { jobAbbrevToJobIconPath } from "../icon/jobicon/JobIconFactory";
import { ITEM_MIN_HEIGHT } from "./Styles";

export function JobItem(jobAbbrev: string, align: string) {
  console.log(align)
  return (
    <Box display="flex" justifyContent={align} alignItems={"center"} height={ITEM_MIN_HEIGHT}>
      <Box marginRight={1}>
        <img
          src={jobAbbrevToJobIconPath(jobAbbrev)}
          alt={jobAbbrev}
          width={25}
          height={25}
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
