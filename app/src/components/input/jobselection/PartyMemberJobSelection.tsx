import {
  MenuItem,
  InputLabel,
  Select,
  SelectChangeEvent,
  Typography,
} from "@mui/material";
import { JobMenuItem } from "../../items/JobMenuItem";
import React from "react";
import { CustomFormControl } from "../basicform/BasicInputForm";
import { ColorConfigurations } from "src/Themes";

export function PartyMemberJobSelection(
  id: number,
  jobNames: string[],
  jobNameSetter: React.Dispatch<React.SetStateAction<string[]>>,
  availablePartyIds: number[],
  setAvailablePartyIds: Function
) {
  let playerId = `Party Member ${id}`;
  const updateState = (index: number) => (e: SelectChangeEvent<string>) => {
    const newJobNames = jobNames.map((jobName, i) => {
      if (i === index) {
        return e.target.value;
      }
      return jobName;
    });

    let newAvailablePartyIds = availablePartyIds;
    newAvailablePartyIds = newAvailablePartyIds.filter(
      (partyId) => partyId !== id
    );

    if (e.target.value !== "Empty") {
      newAvailablePartyIds.push(id);
    }
    newAvailablePartyIds.sort((a, b) => a - b);

    setAvailablePartyIds(newAvailablePartyIds);

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
        MenuProps={{
          PaperProps: {
            sx: {
              backgroundColor: ColorConfigurations.backgroundThree,
            },
          },
        }}
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
        <MenuItem value="Empty">
          <Typography variant="body1" color="white">
            Empty
          </Typography>
        </MenuItem>
      </Select>
    </CustomFormControl>
  );
}
