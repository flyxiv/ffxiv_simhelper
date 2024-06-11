import { MenuItem, FormControl, InputLabel, Select } from "@mui/material";
import { JobItemSelectionMenu } from "./JobItemSelectionMenu";

export function JobSelection(
  id: number,
  jobNames: string[],
  jobNameSetter: React.Dispatch<React.SetStateAction<string[]>>
) {
  let playerId = `Party Member ${id}`;

  return (
    <FormControl fullWidth>
      <InputLabel id="JobSelect">{playerId}</InputLabel>
      <Select
        labelId={playerId}
        id="job-select-{id}"
        value={jobNames[id - 1]}
        key="job-select-{id}"
        label="Job Name"
        onChange={(event) => {
          const newJobNames = [...jobNames];

          newJobNames[id - 1] = event.target.value;

          jobNameSetter(newJobNames);
        }}
      >
        {JobItemSelectionMenu("PLD")}
        {JobItemSelectionMenu("WAR")}
        {JobItemSelectionMenu("DRK")}
        {JobItemSelectionMenu("GNB")}
        {JobItemSelectionMenu("WHM")}
        {JobItemSelectionMenu("AST")}
        {JobItemSelectionMenu("SCH")}
        {JobItemSelectionMenu("SGE")}
        {JobItemSelectionMenu("DRG")}
        {JobItemSelectionMenu("MNK")}
        {JobItemSelectionMenu("NIN")}
        {JobItemSelectionMenu("SAM")}
        {JobItemSelectionMenu("RPR")}
        {JobItemSelectionMenu("BRD")}
        {JobItemSelectionMenu("MCH")}
        {JobItemSelectionMenu("DNC")}
        {JobItemSelectionMenu("SMN")}
        {JobItemSelectionMenu("BLM")}
        {JobItemSelectionMenu("RDM")}
        <MenuItem value="Empty">Empty</MenuItem>
      </Select>
    </FormControl>
  );
}
