import { InputLabel, Select, SelectChangeEvent } from "@mui/material";
import { CustomFormControl } from "src/components/input/basicform/BasicInputForm";
import { RACES } from "src/const/StartStats";
import { RaceMenuItem } from "../items/RaceMenuItem";

export function MainPlayerRaceSelection(
  race: string,
  setRace: Function,
  jobAbbrev: string
) {
  const handleRaceChange = (event: SelectChangeEvent<string>) => {
    setRace(event.target.value);
  };
  let raceLabel = race;

  if (race !== "") {
    raceLabel = "";
  }

  return (
    <CustomFormControl fullWidth>
      <InputLabel id="RaceSelect">{raceLabel}</InputLabel>
      <Select
        labelId="mainPlayerRace"
        id="mainPlayerRace"
        value={race}
        label="Job Name"
        onChange={handleRaceChange}
      >
        {RACES.map((race) => {
          return RaceMenuItem(race, jobAbbrev);
        })}
      </Select>
    </CustomFormControl>
  );
}
