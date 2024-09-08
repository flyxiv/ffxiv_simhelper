import { InputLabel, Select, SelectChangeEvent } from "@mui/material";
import { CustomFormControl } from "../../components/input/basicform/BasicInputForm";
import { RaceMenuItem } from "../items/RaceMenuItem";
import { AppConfigurations } from "../../Themes";
import { EquipmentInput } from "../../types/EquipmentInput";
import { updateOnePlayerPower } from "../../types/ffxivdatabase/ItemSet";
import { RACES } from "../../const/languageTexts";

export function MainPlayerRaceSelection(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalState: Function
) {
  const handleRaceChange = (event: SelectChangeEvent<string>) => {
    let newTotalData = { ...totalEquipmentState };
    newTotalData.equipmentDatas[id].race = event.target.value;
    updateOnePlayerPower(id, newTotalData, setTotalState);
  };

  let totalState = totalEquipmentState.equipmentDatas[id];

  let raceLabel = totalState.race;

  if (totalState.race !== "") {
    raceLabel = "";
  }

  let playerRaceId = `mainPlayerRace${id}`;

  return (
    <CustomFormControl fullWidth>
      <InputLabel id="RaceSelect" key={`${totalState.race}_inputlabel`}>
        {raceLabel}
      </InputLabel>
      <Select
        labelId={playerRaceId}
        id={playerRaceId}
        value={totalState.race}
        key={totalState.race}
        label="Job Name"
        onChange={handleRaceChange}
        MenuProps={{
          PaperProps: {
            sx: {
              backgroundColor: AppConfigurations.backgroundThree,
            },
          },
        }}
        sx={{
          '&.Mui-focused .MuiOutlinedInput-notchedOutline': {
            borderColor: 'transparent'
          },
        }}
      >
        {RACES.map((race) => {
          return RaceMenuItem(race, totalState.mainPlayerJobAbbrev);
        })}
      </Select>
    </CustomFormControl>
  );
}
