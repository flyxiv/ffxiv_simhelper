import { Box, Typography } from "@mui/material";
import { AppConfigurations } from "../../Themes";
import { isMobile } from "../../util";

export const SimLinkIcon = (
  buttonImagePath: string,
  altText: string,
  title: string,
  description: string,
  color: string
) => {
  return (
    <Box display="flex">
      <Box className="SimIcon" marginRight={3}>
        <img
          src={buttonImagePath}
          alt={altText}
          className="ButtonIcon"
          height={75}
          width={75}
        />
      </Box>
      <Box>
        <Typography variant={isMobile() ? "h5" : "h4"} color={color}>
          <b>{title}</b>
        </Typography>
        <Box marginTop={1}>
          <Typography variant="body2" sx={{
            fontSize: AppConfigurations.logoFontSize
          }} color="white">
            {description}
          </Typography>
        </Box>
      </Box>
    </Box>
  );
};
