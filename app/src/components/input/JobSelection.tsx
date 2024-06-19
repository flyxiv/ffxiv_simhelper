import {
  MenuItem,
  FormControl,
  InputLabel,
  Select,
  SelectChangeEvent,
  styled,
} from "@mui/material";
import { JobItemSelectionMenu } from "./JobItemSelectionMenu";
import React, { useEffect } from "react";
import { FormControlStyle } from "./Styles";
import { CustomFormControl } from "./InputForm";

export function JobSelection(
  id: number,
  jobNames: string[],
  jobNameSetter: React.Dispatch<React.SetStateAction<string[]>>
) {
  let playerId = `Party Member ${id}`;
  const updateState = (index: number) => (e: SelectChangeEvent<string>) => {
    const newJobNames = jobNames.map((jobName, i) => {
      if (i === index) {
        return e.target.value;
      }
      return jobName;
    });
    jobNameSetter(newJobNames);
  };

  return (
    <CustomFormControl fullWidth>
      <InputLabel id="JobSelect">{playerId}</InputLabel>
      <Select
        labelId={playerId}
        id="job-select-{id}"
        value={jobNames[id - 1]}
        key="job-select-{id}"
        label="Job Name"
        onChange={(event) => {
          updateState(id - 1)(event);
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
    </CustomFormControl>
  );
}
