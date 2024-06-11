import {
  FormControl,
  InputLabel,
  Select,
  SelectChangeEvent,
} from "@mui/material";
import { JobItemSelectionMenu } from "./JobItemSelectionMenu";

export function JobSelectionWithState(jobName: string, setJobName: Function) {
  const handleJobChange = (event: SelectChangeEvent<string>) => {
    setJobName(event.target.value);
  };

  return (
    <FormControl fullWidth>
      <InputLabel id="JobSelect">Job</InputLabel>
      <Select
        labelId="job"
        id="partyMember{id}JobName"
        value={jobName}
        label="Job Name"
        onChange={handleJobChange}
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
      </Select>
    </FormControl>
  );
}
