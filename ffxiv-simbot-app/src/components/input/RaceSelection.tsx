import { InputLabel, Select, SelectChangeEvent } from "@mui/material";
import { CustomFormControl } from "../../components/input/basicform/BasicInputForm";
import { RACES } from "../../const/StartStats";
import { RaceMenuItem } from "../items/RaceMenuItem";
import { CharacterEquipmentsData } from "src/types/ffxivdatabase/PlayerPower";
import { ColorConfigurations } from "../../Themes";

export function MainPlayerRaceSelection(
  id: number,
  jobAbbrev: string,
  data: CharacterEquipmentsData,
  setData: Function
) {
  const handleRaceChange = (event: SelectChangeEvent<string>) => {
    setData({ ...data, race: event.target.value });
  };

  let raceLabel = data.race;

  if (data.race !== "") {
    raceLabel = "";
  }

  let playerRaceId = `mainPlayerRace${id}`;

  return (
    <CustomFormControl fullWidth>
      <InputLabel id="RaceSelect" key={`${data.race}_inputlabel`}>
        {raceLabel}
      </InputLabel>
      <Select
        labelId={playerRaceId}
        id={playerRaceId}
        value={data.race}
        key={data.race}
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
          return RaceMenuItem(race, jobAbbrev);
        })}
      </Select>
    </CustomFormControl>
  );
}
