import { FormControl, IconButton, MenuItem, Select, Typography } from "@mui/material";
import { LANGUAGE_MODE_SAVE_NAME, LanguageMode, toLanguageMode, useLanguage } from "../../../LanguageContext";
import LanguageIcon from "@mui/icons-material/Language";
import { AppConfigurations } from "../../../Themes";

const ENGLISH_LANGUAGE_TEXT = "English";
const KOREAN_LANGUAGE_TEXT = "한국어";

export function LanguageInputForm(
) {
    let key = `language-selection`;

    let { language, setLanguage } = useLanguage();

    return (
        <>
            <FormControl variant="outlined" sx={{ minWidth: 120 }}>
                <Select
                    labelId="language-selector-label"
                    id="language-selector"
                    value={language}
                    key={key}
                    onChange={(e) => { const languageMode = toLanguageMode(e.target.value); setLanguage(languageMode); localStorage.setItem(LANGUAGE_MODE_SAVE_NAME, languageMode); }}
                    label="Language"
                    IconComponent={() => (
                        <IconButton>
                            <LanguageIcon />
                        </IconButton>
                    )}
                    MenuProps={{
                        PaperProps: {
                            sx: {
                                backgroundColor: AppConfigurations.backgroundThree,
                            },
                        },
                    }}
                >
                    <MenuItem value={LanguageMode.ENGLISH_MODE}><Typography color={"white"}>{ENGLISH_LANGUAGE_TEXT}</Typography></MenuItem>
                    <MenuItem value={LanguageMode.KOREAN_MODE}><Typography color={"white"}>{KOREAN_LANGUAGE_TEXT}</Typography></MenuItem>
                </Select>
            </FormControl>
        </>
    );
}
