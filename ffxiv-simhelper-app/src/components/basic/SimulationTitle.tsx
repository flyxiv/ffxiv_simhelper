import { TitleBoxStyle } from "../container/Styles";
import { styled, Box, Typography } from "@mui/material";

const TitleBox = styled(Box)`
  ${TitleBoxStyle}
`;

export function SimulationTitle(title: string) {
  return (
    <TitleBox borderRadius={4}>
      <Typography variant="h5" component="div" align="center">
        {title}
      </Typography>
    </TitleBox>
  );
}
