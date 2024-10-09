import { InputLabel, MenuItem, Select, SelectChangeEvent, Typography } from "@mui/material";
import { CustomFormControl } from "./basicform/BasicInputForm";
import { styled, Box } from "@mui/material";
import { InputGridItemStyle } from "./Styles";
import { EquipmentInput } from "../../types/EquipmentInput";
import { AppConfigurations } from "../../Themes";
import { PartnerMenuItem } from "../items/PartnerMenuItem";
import { EMPTY_PARTY_MEMBER } from "../../types/PartyStates";
import { LANGUAGE_TEXTS } from "../../const/languageTexts";

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

    if (event.target.value === EMPTY_PARTY_MEMBER) {
      newState.equipmentDatas[id].mainPlayerPartner1Id = null;
    } else {
      newState.equipmentDatas[id].mainPlayerPartner1Id = parseInt(event.target.value);
    }

    setTotalEquipmentState({ ...newState });
  };

  let availablePartyIds = totalEquipmentState.equipmentDatas[id].partyMemberIds;
  let availablePartners: number[] = [];

  for (let i = 0; i < availablePartyIds.length; i++) {
    let partyId = availablePartyIds[i];
    if (partyId === totalEquipmentState.equipmentDatas[id].mainPlayerPartner2Id) {
      continue;
    }
    availablePartners.push(availablePartyIds[i]);
  }

  let currentPartnerId = totalEquipmentState.equipmentDatas[id].mainPlayerPartner1Id;

  return (
    <InputBox>
      <CustomFormControl fullWidth>
        <InputLabel id="partner-selection" key={`${id}_partner1_inputlabel`}>
          <Typography sx={{ fontSize: AppConfigurations.body1FontSize }}> {currentPartnerId === null ? labelText : ""} </Typography>

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
            return PartnerMenuItem(id, partyMemberId, totalEquipmentState.equipmentDatas[id].partyMemberJobAbbrevs[partyMemberId - 1]);
          })}
          {<MenuItem key={`${id}_partner1_empty_menuitem`} value="empty" color="white"><Typography sx={{ fontSize: AppConfigurations.body1FontSize }}>{LANGUAGE_TEXTS.EMPTY_TEXT}</Typography></MenuItem>};
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

    if (event.target.value === EMPTY_PARTY_MEMBER) {
      newState.equipmentDatas[id].mainPlayerPartner2Id = null;
    } else {
      newState.equipmentDatas[id].mainPlayerPartner2Id = parseInt(event.target.value);
    }

    setTotalEquipmentState({ ...newState });
  };

  let availablePartyIds = totalEquipmentState.equipmentDatas[id].partyMemberIds;
  let availablePartners: number[] = [];

  for (let i = 0; i < availablePartyIds.length; i++) {
    let partyId = availablePartyIds[i];
    if (partyId === totalEquipmentState.equipmentDatas[id].mainPlayerPartner1Id) {
      continue;
    }
    availablePartners.push(availablePartyIds[i]);
  }

  let currentPartnerId = totalEquipmentState.equipmentDatas[id].mainPlayerPartner2Id;

  return (
    <InputBox>
      <CustomFormControl fullWidth>
        <InputLabel id="partner-selection" key={`${id}_partner1_inputlabel`}>
          <Typography sx={{ fontSize: AppConfigurations.body1FontSize }}> {currentPartnerId === null ? labelText : ""} </Typography>
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
            return PartnerMenuItem(id, partyMemberId, totalEquipmentState.equipmentDatas[id].partyMemberJobAbbrevs[partyMemberId - 1]);
          })}
          {<MenuItem key={`${id}_partner2_empty_menuitem`} value="empty" color="white"><Typography sx={{ fontSize: AppConfigurations.body1FontSize }}>{LANGUAGE_TEXTS.EMPTY_TEXT}</Typography></MenuItem>};
        </Select>
      </CustomFormControl>
    </InputBox>
  );
}
