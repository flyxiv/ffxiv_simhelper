import { InputLabel, Select, SelectChangeEvent } from "@mui/material";
import { CustomFormControl } from "../basicform/BasicInputForm";
import { JobMenuItem } from "../../items/JobMenuItem";

export function MainPlayerJobSelection(jobName: string, setJobName: Function) {
  const handleJobChange = (event: SelectChangeEvent<string>) => {
    setJobName(event.target.value);
  };

  return (
    <CustomFormControl fullWidth>
      <InputLabel id="JobSelect">Job</InputLabel>
      <Select
        labelId="job"
        id="partyMember{id}JobName"
        value={jobName}
        label="Job Name"
        onChange={handleJobChange}
      >
        {JobMenuItem("PLD")}
        {JobMenuItem("WAR")}
        {JobMenuItem("DRK")}
        {JobMenuItem("GNB")}
        {JobMenuItem("WHM")}
        {JobMenuItem("AST")}
        {JobMenuItem("SCH")}
        {JobMenuItem("SGE")}
        {JobMenuItem("DRG")}
        {JobMenuItem("MNK")}
        {JobMenuItem("NIN")}
        {JobMenuItem("SAM")}
        {JobMenuItem("RPR")}
        {JobMenuItem("VPR")}
        {JobMenuItem("BRD")}
        {JobMenuItem("MCH")}
        {JobMenuItem("DNC")}
        {JobMenuItem("SMN")}
        {JobMenuItem("BLM")}
        {JobMenuItem("RDM")}
        {JobMenuItem("PCT")}
      </Select>
    </CustomFormControl>
  );
}
