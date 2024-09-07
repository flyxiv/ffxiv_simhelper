import { InputLabel, MenuItem, Select, SelectChangeEvent, Typography } from "@mui/material";
import { CustomFormControl } from "./basicform/BasicInputForm";
import { styled, Box } from "@mui/material";
import { InputGridItemStyle } from "./Styles";
import { EquipmentInput } from "../../types/EquipmentInput";
import { AppConfigurations } from "../../Themes";
import { PartnerMenuItem } from "../items/PartnerMenuItem";
import { EMPTY_PARTY_MEMBER } from "../../types/PartyStates";

const InputBox = styled(Box)`
  ${InputGridItemStyle}
`;

export function Partner1Selection(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function,
  labelText: string
) {

  const handlePartnerChange = (event: SelectChangeEvent<string>) => {
    let newState = { ...totalEquipmentState };

    newState.equipmentDatas.forEach((data) => {
      if (event.target.value === EMPTY_PARTY_MEMBER) {
        data.mainPlayerPartner1Id = null;
      } else {
        data.mainPlayerPartner1Id = parseInt(event.target.value);
      }
    });

    setTotalEquipmentState({ ...newState });
  };

  let availablePartyIds = totalEquipmentState.equipmentDatas[0].partyMemberIds;
  let availablePartners: number[] = [];

  for (let i = 0; i < availablePartyIds.length; i++) {
    let partyId = availablePartyIds[i];
    if (partyId === totalEquipmentState.equipmentDatas[0].mainPlayerPartner2Id) {
      continue;
    }
    availablePartners.push(availablePartyIds[i]);
  }

  let currentPartnerId = totalEquipmentState.equipmentDatas[0].mainPlayerPartner1Id;

  return (
    <InputBox>
      <CustomFormControl fullWidth>
        <InputLabel id="partner-selection" key={`${id}_partner1_inputlabel`}>
          {currentPartnerId === null ? labelText : ""}
        </InputLabel>
        <Select
          value={currentPartnerId === null ? "empty" : currentPartnerId.toString()}
          key={`${id}_partner1`}
          onChange={handlePartnerChange}
          MenuProps={{
            PaperProps: {
              sx: {
                backgroundColor: AppConfigurations.backgroundThree,
                color: "white"
              },
            },
          }}
        >
          {availablePartyIds.map((partyMemberId) => {
            return PartnerMenuItem(id, partyMemberId, totalEquipmentState.equipmentDatas[0].partyMemberJobAbbrevs[partyMemberId - 1]);
          })}
          {<MenuItem key={`${id}_partner1_empty_menuitem`} value="empty" color="white">Empty</MenuItem>};
        </Select>
      </CustomFormControl>
    </InputBox>
  );
}

export function Partner2Selection(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function,
  labelText: string
) {

  const handlePartnerChange = (event: SelectChangeEvent<string>) => {
    let newState = { ...totalEquipmentState };

    newState.equipmentDatas.forEach((data) => {
      if (event.target.value === "empty") {
        data.mainPlayerPartner2Id = null;
      } else {
        data.mainPlayerPartner2Id = parseInt(event.target.value);
      }
    });

    setTotalEquipmentState({ ...newState });
  };

  let availablePartyIds = totalEquipmentState.equipmentDatas[0].partyMemberIds;
  let availablePartners: number[] = [];

  for (let i = 0; i < availablePartyIds.length; i++) {
    let partyId = availablePartyIds[i];
    if (partyId === totalEquipmentState.equipmentDatas[0].mainPlayerPartner1Id) {
      continue;
    }
    availablePartners.push(availablePartyIds[i]);
  }

  let currentPartnerId = totalEquipmentState.equipmentDatas[0].mainPlayerPartner2Id;

  return (
    <InputBox>
      <CustomFormControl fullWidth>
        <InputLabel id="partner-selection" key={`${id}_partner1_inputlabel`}>
          {currentPartnerId === null ? labelText : ""}
        </InputLabel>
        <Select
          value={currentPartnerId === null ? "empty" : currentPartnerId.toString()}
          key={`${id}_partner1`}
          onChange={handlePartnerChange}
          MenuProps={{
            PaperProps: {
              sx: {
                backgroundColor: AppConfigurations.backgroundThree,
                color: "white"
              },
            },
          }}
        >
          {availablePartyIds.map((partyMemberId) => {
            return PartnerMenuItem(id, partyMemberId, totalEquipmentState.equipmentDatas[0].partyMemberJobAbbrevs[partyMemberId - 1]);
          })}
          {<MenuItem key={`${id}_partner2_empty_menuitem`} value="empty" color="white">Empty</MenuItem>};
        </Select>
      </CustomFormControl>
    </InputBox>
  );
}
