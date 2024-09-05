import { InputLabel, Select, SelectChangeEvent } from "@mui/material";
import { CustomFormControl } from "../../components/input/basicform/BasicInputForm";
import { RACES } from "../../const/StartStats";
import { RaceMenuItem } from "../items/RaceMenuItem";
import { ColorConfigurations } from "../../Themes";
import { EquipmentInput } from "../../types/EquipmentInput";
import { updateOnePlayerPower } from "../../types/ffxivdatabase/ItemSet";

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
              backgroundColor: ColorConfigurations.backgroundThree,
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
