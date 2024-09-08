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
import { AppConfigurations } from "../../../Themes";
import { EquipmentInput } from "../../../types/EquipmentInput";
import { isHealer, isTank } from "../../../types/ffxivdatabase/PlayerPower";
import { BUFF_JOBS_LIST, DPS_JOBS, HEALER_JOBS, TANK_JOBS } from "../../../types/ffxivdatabase/PartyCompositionMaker";
import { PARTY_MEMBER_LABEL_TEXT } from "../../../const/languageTexts";

let ALIGN = "center";

function filterDuplicateBuffJobs(jobList: Array<string>, mainCharacterJob: string, partyMemberJobAbbrevs: Array<string>) {
  return jobList.filter((jobAbbrev) => {
    if (!BUFF_JOBS_LIST.includes(jobAbbrev)) {
      return true
    }

    return jobAbbrev !== mainCharacterJob && !partyMemberJobAbbrevs.includes(jobAbbrev)
  })
}

export function getRoleByIdAndMainCharacterJob(id: number, mainCharacterJob: string, partyMemberJobAbbrevs: Array<string>) {
  let otherPartyMemberJobAbbrevs = partyMemberJobAbbrevs.filter((_, index) => index !== id - 1);
  let tank_jobs = TANK_JOBS;
  let healer_jobs = filterDuplicateBuffJobs(HEALER_JOBS, mainCharacterJob, otherPartyMemberJobAbbrevs);
  let dps_jobs = filterDuplicateBuffJobs(DPS_JOBS, mainCharacterJob, otherPartyMemberJobAbbrevs);

  if (id == 1) {
    return tank_jobs;
  }

  if (id == 2) {
    if (isTank(mainCharacterJob)) {
      return healer_jobs;
    } else {
      return tank_jobs;
    }
  }

  if (id == 3) {
    return healer_jobs;
  }

  if (id == 4) {
    if (isTank(mainCharacterJob) || isHealer(mainCharacterJob)) {
      return dps_jobs;
    } else {
      return healer_jobs;
    }
  }

  return dps_jobs;
}

export function PartyMemberJobSelection(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalState: Function
) {
  let playerId = `${PARTY_MEMBER_LABEL_TEXT} ${id}`;
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

  let jobAbbrevs = getRoleByIdAndMainCharacterJob(id, totalEquipmentState.equipmentDatas[0].mainPlayerJobAbbrev, totalEquipmentState.equipmentDatas[0].partyMemberJobAbbrevs);

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
              backgroundColor: AppConfigurations.backgroundThree,
            },
          },
        }}
      >
        {jobAbbrevs.map((jobAbbrev) => {
          return JobMenuItem(jobAbbrev, ALIGN, false)
        })}

        <Divider />
        <MenuItem value="Empty">
          <Typography variant="body1" color="white">
            Empty
          </Typography>
        </MenuItem>
      </Select>
    </CustomFormControl>
  );
}
