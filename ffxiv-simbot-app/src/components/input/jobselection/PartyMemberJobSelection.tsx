import {
  MenuItem,
  InputLabel,
  Select,
  SelectChangeEvent,
  Typography,
} from "@mui/material";
import { JobMenuItem } from "../../items/JobMenuItem";
import { CustomFormControl } from "../basicform/BasicInputForm";
import { ColorConfigurations } from "../../../Themes";
import { SingleEquipmentInputSaveState } from "../../../types/SingleEquipmentInputSaveState";

export function PartyMemberJobSelection(
  id: number,
  totalState: SingleEquipmentInputSaveState,
  setTotalState: Function
) {
  let playerId = `Party Member ${id}`;
  const updateState = (index: number) => (e: SelectChangeEvent<string>) => {
    const newJobNames = totalState.partyMemberJobAbbrevs.map((jobName, i) => {
      if (i === index) {
        return e.target.value;
      }
      return jobName;
    });

    let newAvailablePartyIds = totalState.partyMemberIds;
    newAvailablePartyIds = newAvailablePartyIds.filter(
      (partyId) => partyId !== id
    );

    if (e.target.value !== "Empty") {
      newAvailablePartyIds.push(id);
    }
    newAvailablePartyIds.sort((a, b) => a - b);

    let newTotalState = { ...totalState, partyMemberJobAbbrevs: newJobNames, partyMemberIds: newAvailablePartyIds };

    setTotalState(newTotalState);
  };

  let key = `job-select-partymember-${id}`;

  return (
    <CustomFormControl fullWidth>
      <InputLabel id="JobSelect">{playerId}</InputLabel>
      <Select
        labelId={playerId}
        id={key}
        value={totalState.partyMemberJobAbbrevs[id - 1]}
        key={key}
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
