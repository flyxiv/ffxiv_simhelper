import { Box, Typography } from "@mui/material";
import {
  Materia,
  toMateriaDescription,
  toMateriaKey,
} from "../../types/ffxivdatabase/Materia";
import { getMateriaIconPath } from "../icon/materiaicon/MateriaIconFactory";

export function MateriaItem(
  materiaKey: string,
  currentlyEquippedMateria: Materia
) {
  let imageSize = {
    xs: 15,
    sm: 20,
    md: 25,
    lg: 30,
    xl: 35,
  };

  let fontSize = {
    xs: 10,
    sm: 12,
    md: 12,
    lg: 14,
    xl: 14,
  }

  let isSelected = toMateriaKey(currentlyEquippedMateria) === materiaKey;
  let isNotFullyUsed =
    isSelected &&
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
      <Box component={"img"}
        src={getMateriaIconPath(materiaKey)}
        alt={getMateriaIconPath(materiaKey)}
        sx={{ width: imageSize, height: imageSize }}
        style={{ verticalAlign: "middle" }}
      />
      <Box>
        <Typography
          variant="body2"
          alignContent={"center"}
          color={isNotFullyUsed ? "red" : "white"}
          sx={{
            fontSize: fontSize
          }}
        >
          {isSelected
            ? toMateriaDescription(currentlyEquippedMateria)
            : materiaKey}
        </Typography>
      </Box>
    </Box>
  );
}
