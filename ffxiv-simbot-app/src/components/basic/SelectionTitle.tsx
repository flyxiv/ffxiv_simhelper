import { Box, styled, Typography } from "@mui/material";
import { SelectionTitleStyle } from "./Style";

let SelectionTitleBox = styled(Box)`
    ${SelectionTitleStyle}
`;

const BORDER_RADIUS = 3;


export function SelectionTitle(title: string) {
    return (
        <Box display="flex" width="100%" justifyContent={"center"}>
            <SelectionTitleBox borderRadius={BORDER_RADIUS} display="inline-block">
                <Typography variant="h5" align="center">{title}</Typography>
            </SelectionTitleBox>
        </Box>
    )
}