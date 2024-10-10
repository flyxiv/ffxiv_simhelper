import { Typography } from "@mui/material";
import { AppConfigurations } from "../../Themes";

export function WarningText(text: string) {
    return (
        <Typography
            sx={{ fontSize: AppConfigurations.body2FontSize, color: "white" }}
        >
            {text}
        </Typography>
    )
}