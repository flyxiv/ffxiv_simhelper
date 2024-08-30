import { Box, Typography, styled } from "@mui/material";
import { Equipment } from "../../types/ffxivdatabase/Equipment";
import {
  accessSubStatByKey,
  addMateriaMaxValueToEquipment,
} from "../../types/ffxivdatabase/FinalEquipmentStat";
import { Materia } from "../../types/ffxivdatabase/Materia";
import {
  EquipmentSingleBoxStyle,
  EquipmentSingleSubStatBoxStyle,
  EquipmentSubStatBoxStyle,
} from "./Styles";
import { ColorConfigurations } from "../../Themes";

const EquipmentSubStatBox = styled(Box)`
  ${EquipmentSubStatBoxStyle}
`;

const EquipmentSingleSubStatBoxByNumber = (numberOfSubStats: number) => styled(
  Box
)`
  ${EquipmentSingleSubStatBoxStyle(numberOfSubStats)}
`;

const EquipmentSingleBox = styled(Box)`
  ${EquipmentSingleBoxStyle}
`;

export function EquipmentSubStatTable(
  equipment: Equipment,
  materias: Materia[] | undefined
) {
  let finalStats = addMateriaMaxValueToEquipment(equipment, materias);
  let nonZeroSubStats = [];
  if (finalStats.criticalStrike > 0) {
    nonZeroSubStats.push("CRT");
  }
  if (finalStats.directHit > 0) {
    nonZeroSubStats.push("DH");
  }
  if (finalStats.determination > 0) {
    nonZeroSubStats.push("DET");
  }
  if (finalStats.skillSpeed > 0) {
    nonZeroSubStats.push("SKS");
  }
  if (finalStats.spellSpeed > 0) {
    nonZeroSubStats.push("SPS");
  }
  if (finalStats.tenacity > 0) {
    nonZeroSubStats.push("TEN");
  }

  let numberOfSubStats = nonZeroSubStats.length;
  let EquipmentSingleSubStatBox =
    EquipmentSingleSubStatBoxByNumber(numberOfSubStats);

  return (
    <EquipmentSubStatBox>
      {nonZeroSubStats.map((subStatName) => {
        let subStatNonClippedValue = accessSubStatByKey(
          finalStats,
          subStatName
        );

        let clippedAmount = Math.min(
          finalStats.maxSubstat - subStatNonClippedValue,
          0
        );

        return (
          <EquipmentSingleSubStatBox
            color={clippedAmount < 0 ? "red" : "white"}
            paddingY={2}
          >
            <EquipmentSingleBox sx={{ backgroundColor: ColorConfigurations.backgroundThree }}>
              <Typography variant="body2" align="center">{subStatName}</Typography>
            </EquipmentSingleBox>
            <EquipmentSingleBox alignContent={"center"}>
              <Typography variant="body2" sx={{ backgroundColor: ColorConfigurations.backgroundFour }} align="center">
                {subStatNonClippedValue + clippedAmount}/{finalStats.maxSubstat}
                {clippedAmount < 0 ? `(${clippedAmount})` : ""}
              </Typography>
            </EquipmentSingleBox>
          </EquipmentSingleSubStatBox>
        );
      })}
    </EquipmentSubStatBox>
  );
}
