import { Box, Typography } from "@mui/material";
import { jobAbbrevToJobIconPath } from "../icon/jobicon/JobIconFactory";
import { ITEM_TOP_MENU_MIN_HEIGHT } from "./Styles";
import { AppConfigurations } from "../../Themes";

export function PartnerItem(partyMemberJobId: number, jobAbbrev: string, partyMemberLabelText: string) {
  return (
    <Box display="flex" justifyContent={"left"} alignItems={"center"} height={ITEM_TOP_MENU_MIN_HEIGHT}>
      <Box
        component={"img"}
        src={jobAbbrevToJobIconPath(jobAbbrev)}
        alt={jobAbbrev}
        sx={{ width: "50%", maxWidth: "25px", height: 'auto', marginRight: { xs: 0.5, sm: 1 } }} // 부모의 50% 너비로 설정, 높이는 자동 비율 유지
      />
      <Box>
        <Typography variant="body1" alignContent={"center"} color="white" fontSize={AppConfigurations.body1FontSize}>
          {`${partyMemberLabelText} ${partyMemberJobId}`}
        </Typography>
      </Box>
    </Box>
  );
}
