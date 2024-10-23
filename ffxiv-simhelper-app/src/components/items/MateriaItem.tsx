import { Box, Typography } from "@mui/material";
import {
  Materia,
  materiaKeyToText,
  toMateriaDescription,
  toMateriaKey,
} from "../../types/ffxivdatabase/Materia";
import { getMateriaIconPath } from "../icon/materiaicon/MateriaIconFactory";
import { AppConfigurations } from "../../Themes";
import { TextDictionary } from "../../const/languageTexts";

export function MateriaItem(
  materiaKey: string,
  currentlyEquippedMateria: Materia,
  LANGUAGE_TEXTS: TextDictionary
) {
  let imageSize = "3vh";

  let isSelected = toMateriaKey(currentlyEquippedMateria) === materiaKey;
  let isNotFullyUsed =
    currentlyEquippedMateria.effectiveValue < currentlyEquippedMateria.maxValue;

  return (
    <Box
      className={materiaKey}
      display="flex"
      flexDirection="column"
      alignItems="center"
      justifyContent="center"
      height="100%"
    >
      <Box
        component={"img"}
        src={getMateriaIconPath(materiaKey)}
        alt={getMateriaIconPath(materiaKey)}
        sx={{ width: imageSize, height: imageSize }}
        style={{ verticalAlign: "middle" }}
      />
      <Box>
        <Typography
          variant="body2"
          alignContent={"center"}
          color={isNotFullyUsed && isSelected ? "red" : "white"}
          sx={{
            fontSize: AppConfigurations.body2FontSize,
          }}
        >
          {isSelected
            ? toMateriaDescription(currentlyEquippedMateria, LANGUAGE_TEXTS)
            : materiaKeyToText(materiaKey, LANGUAGE_TEXTS)}
        </Typography>
      </Box>
    </Box>
  );
}
