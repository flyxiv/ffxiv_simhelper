import { Box, Typography } from "@mui/material";
import { jobAbbrevToJobIconPath } from "../icon/jobicon/JobIconFactory";
import { ITEM_TOP_MENU_MIN_HEIGHT } from "./Styles";
import { AppConfigurations } from "../../Themes";

export function PartnerItem(partyMemberJobId: number, jobAbbrev: string, partyMemberLabelText: string) {
  return (
    <Box display="flex" justifyContent="right" alignItems={"center"} height={ITEM_TOP_MENU_MIN_HEIGHT}>
      <Box marginRight={1}>
        <img
          src={jobAbbrevToJobIconPath(jobAbbrev)}
          alt={jobAbbrev}
          width={50}
          height={50}
          style={{ verticalAlign: "middle" }}
        />
      </Box>
      <Box>
        <Typography variant="body1" alignContent={"center"} color="white" fontSize={AppConfigurations.body1FontSize}>
          {`${partyMemberLabelText} ${partyMemberJobId}`}
        </Typography>
      </Box>
    </Box>
  );
}
