import { Box, Typography } from "@mui/material";
import { AppConfigurations } from "../../Themes";

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
        <Typography variant="h4" color={color}>
          <b>{title}</b>
        </Typography>
        <Box marginTop={1}>
          <Typography variant="body2" sx={{
            fontSize: {
              xs: 10,
              sm: 12,
              md: 12,
              lg: 14,
            }
          }} color="white">
            {description}
          </Typography>
        </Box>
      </Box>
    </Box>
  );
};
