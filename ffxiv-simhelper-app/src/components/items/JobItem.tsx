import { Box, Typography } from "@mui/material";
import { jobAbbrevToJobIcon } from "../icon/jobicon/JobIconFactory";
import { ITEM_BOTTOM_MENU_MIN_HEIGHT, ITEM_TOP_MENU_MIN_HEIGHT } from "./Styles";
import { convertToJobText, TextDictionary } from "../../const/languageTexts";
import { AppConfigurations } from "../../Themes";

export function JobItem(jobAbbrev: string, align: string, LANGUAGE_TEXTS: TextDictionary, is_top: boolean) {
  return (
    <Box display="flex" justifyContent={align} alignItems={"center"} height={is_top ? ITEM_TOP_MENU_MIN_HEIGHT : ITEM_BOTTOM_MENU_MIN_HEIGHT}>
      <Box
        component={"img"}
        src={jobAbbrevToJobIcon(jobAbbrev)}
        alt={jobAbbrev}
        sx={{ width: "50%", maxWidth: "25px", height: 'auto', marginRight: { xs: 0.5, sm: 1 } }} // 부모의 50% 너비로 설정, 높이는 자동 비율 유지
      />
      <Box>
        <Typography variant="body1" alignContent={"center"} color="white" sx={{ fontSize: AppConfigurations.body1FontSize }}>
          {convertToJobText(jobAbbrev, LANGUAGE_TEXTS)}
        </Typography>
      </Box>
    </Box>
  );
}
