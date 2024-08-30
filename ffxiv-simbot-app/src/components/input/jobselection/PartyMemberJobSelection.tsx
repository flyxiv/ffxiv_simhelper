import {
  MenuItem,
  InputLabel,
  Select,
  SelectChangeEvent,
  Typography,
  Divider,
} from "@mui/material";
import { JobMenuItem } from "../../items/JobMenuItem";
import { CustomFormControl } from "../basicform/BasicInputForm";
import { ColorConfigurations } from "../../../Themes";
import { EquipmentInput } from "../../../types/EquipmentInput";

let ALIGN = "center";

export function PartyMemberJobSelection(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalState: Function
) {
  let playerId = `Party Member ${id}`;
  const updateState = (index: number) => (e: SelectChangeEvent<string>) => {
    const newJobNames = totalEquipmentState.equipmentDatas[0].partyMemberJobAbbrevs.map((jobName, i) => {
      if (i === index) {
        return e.target.value;
      }
      return jobName;
    });

    let newAvailablePartyIds = totalEquipmentState.equipmentDatas[0].partyMemberIds;
    newAvailablePartyIds = newAvailablePartyIds.filter(
      (partyId) => partyId !== id
    );

    if (e.target.value !== "Empty") {
      newAvailablePartyIds.push(id);
    }
    newAvailablePartyIds.sort((a, b) => a - b);

    let newTotalState = { ...totalEquipmentState };

    newTotalState.equipmentDatas.forEach((data) => {
      data.partyMemberJobAbbrevs = newJobNames;
      data.partyMemberIds = newAvailablePartyIds;
    })

    setTotalState({ ...newTotalState });
  };

  let key = `job-select-partymember-${id}`;

  return (
    <CustomFormControl fullWidth>
      <InputLabel id="JobSelect">{playerId}</InputLabel>
      <Select
        labelId={playerId}
        id={key}
        value={totalEquipmentState.equipmentDatas[0].partyMemberJobAbbrevs[id - 1]}
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
        {JobMenuItem("PLD", ALIGN)}
        {JobMenuItem("WAR", ALIGN)}
        {JobMenuItem("DRK", ALIGN)}
        {JobMenuItem("GNB", ALIGN)}
        <Divider />
        {JobMenuItem("WHM", ALIGN)}
        {JobMenuItem("AST", ALIGN)}
        {JobMenuItem("SCH", ALIGN)}
        {JobMenuItem("SGE", ALIGN)}
        <Divider />
        {JobMenuItem("DRG", ALIGN)}
        {JobMenuItem("MNK", ALIGN)}
        {JobMenuItem("NIN", ALIGN)}
        {JobMenuItem("SAM", ALIGN)}
        {JobMenuItem("RPR", ALIGN)}
        {JobMenuItem("VPR", ALIGN)}
        <Divider />
        {JobMenuItem("BRD", ALIGN)}
        {JobMenuItem("MCH", ALIGN)}
        {JobMenuItem("DNC", ALIGN)}
        <Divider />
        {JobMenuItem("SMN", ALIGN)}
        {JobMenuItem("BLM", ALIGN)}
        {JobMenuItem("RDM", ALIGN)}
        {JobMenuItem("PCT", ALIGN)}
        <MenuItem value="Empty">
          <Typography variant="body1" color="white">
            Empty
          </Typography>
        </MenuItem>
      </Select>
    </CustomFormControl>
  );
}
