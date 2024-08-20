import { Divider, InputLabel, Select, SelectChangeEvent } from "@mui/material";
import { CustomFormControl } from "../basicform/BasicInputForm";
import { JobMenuItem } from "../../items/JobMenuItem";
import {
  defaultItemSet,
  updatePlayerPower,
} from "../../../types/ffxivdatabase/ItemSet";
import { CharacterEquipmentsData } from "../../../types/ffxivdatabase/PlayerPower";
import { ColorConfigurations } from "../../../Themes";
import { DEFAULT_GEARSET_MATERIAS } from "../../../const/DefaultQuickSimInput";

export function MainPlayerJobSelection(
  id: number,
  jobName: string,
  setJobName: Function,
  data: CharacterEquipmentsData,
  setData: Function
) {
  const handleJobChange = (event: SelectChangeEvent<string>) => {
    setJobName(event.target.value);
    let newData = { ...data };

    newData.itemSet = defaultItemSet();
    newData.gearSetMaterias = DEFAULT_GEARSET_MATERIAS;
    newData.jobAbbrev = event.target.value;

    updatePlayerPower(newData, setData);
  };

  let key = `Job-${id}`;

  return (
    <CustomFormControl fullWidth>
      <InputLabel id="JobSelect">Job</InputLabel>
      <Select
        labelId={key}
        id={key}
        value={jobName}
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
