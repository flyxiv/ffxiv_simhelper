import { Divider, InputLabel, Select, SelectChangeEvent } from "@mui/material";
import { CustomFormControl } from "../basicform/BasicInputForm";
import { JobMenuItem } from "../../items/JobMenuItem";
import {
  defaultItemSet,
  updatePlayerPower,
} from "../../../types/ffxivdatabase/ItemSet";
import { ColorConfigurations } from "../../../Themes";
import { DEFAULT_GEARSET_MATERIAS } from "../../../const/DefaultSingleEquipmentInput";
import { SingleEquipmentInputSaveState } from "../../../types/SingleEquipmentInputSaveState";

export function MainPlayerJobSelection(
  id: number,
  totalState: SingleEquipmentInputSaveState,
  setTotalState: Function,
) {
  const handleJobChange = (event: SelectChangeEvent<string>) => {
    let newJobAbbrev = event.target.value;
    let newTotalState = { ...totalState };

    newTotalState.itemSet = defaultItemSet();
    newTotalState.gearSetMaterias = DEFAULT_GEARSET_MATERIAS;
    newTotalState.mainPlayerJobAbbrev = newJobAbbrev;

    updatePlayerPower(newTotalState, setTotalState);
  };

  let key = `Job-${id}`;

  return (
    <CustomFormControl fullWidth>
      <InputLabel id="JobSelect">Job</InputLabel>
      <Select
        labelId={key}
        id={key}
        value={totalState.mainPlayerJobAbbrev}
        label={key}
        onChange={handleJobChange}
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
        <Divider />
        {JobMenuItem("WHM")}
        {JobMenuItem("AST")}
        {JobMenuItem("SCH")}
        {JobMenuItem("SGE")}
        <Divider />
        {JobMenuItem("DRG")}
        {JobMenuItem("MNK")}
        {JobMenuItem("NIN")}
        {JobMenuItem("SAM")}
        {JobMenuItem("RPR")}
        {JobMenuItem("VPR")}
        <Divider />
        {JobMenuItem("BRD")}
        {JobMenuItem("MCH")}
        {JobMenuItem("DNC")}
        <Divider />
        {JobMenuItem("SMN")}
        {JobMenuItem("BLM")}
        {JobMenuItem("RDM")}
        {JobMenuItem("PCT")}
      </Select>
    </CustomFormControl>
  );
}
