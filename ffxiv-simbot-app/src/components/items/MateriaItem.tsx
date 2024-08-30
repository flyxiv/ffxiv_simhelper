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
    >
      <Box marginRight={1}>
        <img
          src={getMateriaIconPath(materiaKey)}
          alt={getMateriaIconPath(materiaKey)}
          width={20}
          height={20}
          style={{ verticalAlign: "middle" }}
        />
      </Box>
      <Box>
        <Typography
          variant="body2"
          alignContent={"center"}
          fontSize={10}
          color={isNotFullyUsed ? "red" : "white"}
        >
          {isSelected
            ? toMateriaDescription(currentlyEquippedMateria)
            : materiaKey}
        </Typography>
      </Box>
    </Box>
  );
}
