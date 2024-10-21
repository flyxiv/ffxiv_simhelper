import { Box, Link, Typography } from "@mui/material";
import { AppConfigurations } from "../../Themes";
import { LanguageMode, useLanguage } from "../../LanguageContext";

export function WarningText(text: string) {
    let { language } = useLanguage();
    return (
        <Link href={language === LanguageMode.KOREAN_MODE ? "https://junyeopn.github.io/blog/docs/ffxivsimhelperkr" : "https://junyeopn.github.io/blog/docs/ffxivsimhelperen/"}>
            <Typography
                sx={{ fontSize: AppConfigurations.body2FontSize, color: "white" }}
                align="center"
            >
                {text}
            </Typography>
        </Link>
    )
}

function DonationText(text: string) {
    return (
        <Typography
            sx={{ fontSize: AppConfigurations.body2FontSize, color: "white" }}
            align="center"
        >
            {text}
        </Typography>
    )
}

export function DemoWarningText(text: string) {
    return AppConfigurations.isApp ? <Box /> : WarningText(text)
}

export function DonationComponent(text: string) {
    return AppConfigurations.isApp ? <Box /> : DonationText(text)
}
