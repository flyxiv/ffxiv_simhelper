import { Divider, Select, SelectChangeEvent } from "@mui/material";
import { CustomFormControl } from "../basicform/BasicInputForm";
import { JobMenuItem } from "../../items/JobMenuItem";
import {
  defaultItemSet,
  updateAllPlayerPower,
} from "../../../types/ffxivdatabase/ItemSet";
import { ColorConfigurations } from "../../../Themes";
import { DEFAULT_GEARSET_MATERIAS } from "../../../const/DefaultSingleEquipmentInput";
import { EquipmentInput, SingleEquipmentInputSaveState } from "../../../types/EquipmentInput";

let ALIGN = "left";

export function MainPlayerJobSelection(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalState: Function,
) {
  const handleJobChange = (event: SelectChangeEvent<string>) => {
    let newJobAbbrev = event.target.value;
    let newTotalState = { ...totalEquipmentState };

    newTotalState.equipmentDatas.forEach((data: SingleEquipmentInputSaveState) => {
      data.itemSet = defaultItemSet();
      data.gearSetMaterias = DEFAULT_GEARSET_MATERIAS;
      data.mainPlayerJobAbbrev = newJobAbbrev;
    });

    updateAllPlayerPower(newTotalState, setTotalState);
  };

  let key = `Job-${id}`;

  return (
    <CustomFormControl fullWidth sx={{ height: '100%' }}>
      <Select
        labelId={key}
        id={key}
        value={totalEquipmentState.equipmentDatas[id].mainPlayerJobAbbrev}
        label={key}
        onChange={handleJobChange}
        MenuProps={{
          PaperProps: {
            sx: {
              backgroundColor: ColorConfigurations.backgroundThree,
            },
          },
        }}

        sx={{
          height: '100%',
          display: 'flex',
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
      </Select>
    </CustomFormControl>
  );
}
