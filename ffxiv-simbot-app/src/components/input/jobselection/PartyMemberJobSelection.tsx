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

    if (newTotalState.equipmentDatas[0].mainPlayerPartner1Id === id) {
      newTotalState.equipmentDatas[0].mainPlayerPartner1Id = null;
    }

    if (newTotalState.equipmentDatas[0].mainPlayerPartner2Id === id) {
      newTotalState.equipmentDatas[0].mainPlayerPartner2Id = null;
    }

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
        {JobMenuItem("PLD", ALIGN, false)}
        {JobMenuItem("WAR", ALIGN, false)}
        {JobMenuItem("DRK", ALIGN, false)}
        {JobMenuItem("GNB", ALIGN, false)}
        <Divider />
        {JobMenuItem("WHM", ALIGN, false)}
        {JobMenuItem("AST", ALIGN, false)}
        {JobMenuItem("SCH", ALIGN, false)}
        {JobMenuItem("SGE", ALIGN, false)}
        <Divider />
        {JobMenuItem("DRG", ALIGN, false)}
        {JobMenuItem("MNK", ALIGN, false)}
        {JobMenuItem("NIN", ALIGN, false)}
        {JobMenuItem("SAM", ALIGN, false)}
        {JobMenuItem("RPR", ALIGN, false)}
        {JobMenuItem("VPR", ALIGN, false)}
        <Divider />
        {JobMenuItem("BRD", ALIGN, false)}
        {JobMenuItem("MCH", ALIGN, false)}
        {JobMenuItem("DNC", ALIGN, false)}
        <Divider />
        {JobMenuItem("SMN", ALIGN, false)}
        {JobMenuItem("BLM", ALIGN, false)}
        {JobMenuItem("RDM", ALIGN, false)}
        {JobMenuItem("PCT", ALIGN, false)}
        <MenuItem value="Empty">
          <Typography variant="body1" color="white">
            Empty
          </Typography>
        </MenuItem>
      </Select>
    </CustomFormControl>
  );
}
