import { Box, Typography } from "@mui/material";
import { ColorConfigurations } from "../../Themes";

export const SimLinkIcon = (
  buttonImagePath: string,
  altText: string,
  title: string,
  description: string,
  color: string
) => {
  console.log(buttonImagePath);
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
          {title}
        </Typography>
        <Box marginTop={1}>
          <Typography variant="body2" fontSize={10} color="white">
            {description}
          </Typography>
        </Box>
      </Box>
    </Box>
  );
};
