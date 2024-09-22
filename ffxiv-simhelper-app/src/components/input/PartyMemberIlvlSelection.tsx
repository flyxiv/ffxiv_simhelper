import { Box, InputLabel, MenuItem, Select, Typography } from "@mui/material";
import { CustomFormControl } from "./basicform/BasicInputForm";
import { EquipmentInput } from "../../types/EquipmentInput";
import { AppConfigurations, ENGLISH_MODE } from "../../Themes";
import { ITEM_BOTTOM_MENU_MIN_HEIGHT } from "../items/Styles";

export const PARTY_MIN_ILVL =
  AppConfigurations.languageMode === ENGLISH_MODE ? 710 : 690;
export const PARTY_MAX_ILVL =
  AppConfigurations.languageMode === ENGLISH_MODE ? 730 : 730;

export function PartyMemberIlvlSelection(
  label: string,
  totalEquipmentState: EquipmentInput,
  setTotalState: Function
) {
  let possibleIlvls = [];

  for (let ilvl = PARTY_MIN_ILVL; ilvl <= PARTY_MAX_ILVL; ilvl += 5) {
    possibleIlvls.push(ilvl);
  }

  return (
    <CustomFormControl fullWidth>
      <InputLabel id="JobSelect">
        {" "}
        <Typography sx={{ fontSize: AppConfigurations.body1FontSize }}>
          {label}
        </Typography>
      </InputLabel>
      <Select
        labelId={label}
        id={label}
        value={totalEquipmentState.equipmentDatas[0].partyMemberIlvl}
        key={label}
        label={label}
        onChange={(event) => {
          let newIlvl = event.target.value;

          if (typeof newIlvl === "string") {
            newIlvl = parseInt(newIlvl);
          }

          let newTotalState = { ...totalEquipmentState };
          newTotalState.equipmentDatas.forEach((element) => {
            element.partyMemberIlvl = newIlvl;
          });

          setTotalState(newTotalState);
        }}
        MenuProps={{
          PaperProps: {
            sx: {
              backgroundColor: AppConfigurations.backgroundThree,
            },
          },
        }}
      >
        {possibleIlvls.map((ilvl) => {
          return (
            <MenuItem value={ilvl}>
              <Box
                sx={{ height: ITEM_BOTTOM_MENU_MIN_HEIGHT }}
                display="flex"
                alignItems={"center"}
                justifyContent="center"
              >
                <Typography
                  variant="body1"
                  color="white"
                  sx={{ fontSize: AppConfigurations.body1FontSize }}
                >
                  {ilvl}
                </Typography>
              </Box>
            </MenuItem>
          );
        })}
      </Select>
    </CustomFormControl>
  );
}
