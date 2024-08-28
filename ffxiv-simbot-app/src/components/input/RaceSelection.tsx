import { InputLabel, Select, SelectChangeEvent } from "@mui/material";
import { CustomFormControl } from "../../components/input/basicform/BasicInputForm";
import { RACES } from "../../const/StartStats";
import { RaceMenuItem } from "../items/RaceMenuItem";
import { ColorConfigurations } from "../../Themes";
import { SingleEquipmentInputSaveState } from "../../types/SingleEquipmentInputSaveState";

export function MainPlayerRaceSelection(
  id: number,
  totalState: SingleEquipmentInputSaveState,
  setTotalState: Function
) {
  const handleRaceChange = (event: SelectChangeEvent<string>) => {
    console.log("a");
    setTotalState({ ...totalState, race: event.target.value });
  };

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
      >
        {RACES.map((race) => {
          return RaceMenuItem(race, totalState.mainPlayerJobAbbrev);
        })}
      </Select>
    </CustomFormControl>
  );
}
