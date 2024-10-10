import { Box, MenuItem, Select, Typography } from "@mui/material";
import { CustomFormControl } from "./BasicInputForm";
import { AppConfigurations } from "../../../Themes";
import { LanguageMode, toLanguageMode, useLanguage } from "../../../LanguageContext";

const ENGLISH_LANGUAGE_TEXT = "English";
const KOREAN_LANGUAGE_TEXT = "한국어";

export function LanguageInputForm(
) {
    let key = `language-selection`;

    let { language, setLanguage } = useLanguage();

    return (
        <>
            <CustomFormControl fullWidth>
                <Select
                    labelId={key}
                    id={key}
                    value={language}
                    key={key}
                    label={key}
                    onChange={(e) => { setLanguage(toLanguageMode(e.target.value)) }}
                    MenuProps={{
                        PaperProps: {
                            sx: {
                                backgroundColor: AppConfigurations.backgroundThree,
                            },
                        },
                    }}
                >
                    <MenuItem value={LanguageMode.ENGLISH_MODE}>
                        <Box
                            display="flex"
                            alignItems="center"
                            justifyContent="flex-end"
                        >
                            <Typography variant="body2" color="white" sx={{ fontSize: AppConfigurations.body1FontSize }}>
                                {ENGLISH_LANGUAGE_TEXT}
                            </Typography>
                        </Box>
                    </MenuItem>
                    <MenuItem value={LanguageMode.KOREAN_MODE}>
                        <Box
                            display="flex"
                            alignItems="center"
                            justifyContent="flex-end"
                        >
                            <Typography variant="body2" color="white" sx={{ fontSize: AppConfigurations.body1FontSize }}>
                                {KOREAN_LANGUAGE_TEXT}
                            </Typography>
                        </Box>
                    </MenuItem>
                </Select>
            </CustomFormControl>
        </>
    );
}
