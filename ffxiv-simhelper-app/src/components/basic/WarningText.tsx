import { Box, Typography } from "@mui/material";
import { AppConfigurations } from "../../Themes";

export function WarningText(text: string) {
    return (
        <Typography
            sx={{ fontSize: AppConfigurations.body2FontSize, color: "white" }}
            align="center"
        >
            {text}
        </Typography >
    )
}

export function DemoWarningText(text: string) {
    return AppConfigurations.isApp ? <Box /> : WarningText(text)
}