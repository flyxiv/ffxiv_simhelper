import { MenuItem, Typography } from "@mui/material";
import { CustomFormControl } from "./basicform/BasicInputForm";
import { styled, Box } from "@mui/material";
import { InputGridItemStyle } from "./Styles";
import { EquipmentInput } from "../../types/EquipmentInput";
import { AppConfigurations } from "../../Themes";
import { PartnerMenuItem } from "../items/PartnerMenuItem";
import { EMPTY_PARTY_MEMBER } from "../../types/PartyStates";
import { TopMenuInput } from "./basicform/EquipmentInputForm";
import { ITEM_TOP_MENU_MIN_HEIGHT } from "../items/Styles";

const InputBox = styled(Box)`
  ${InputGridItemStyle}
`;

export function Partner1Selection(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function,
  labelText: string,
  partyMemberLabelText: string,
  emptyText: string
) {
  const handlePartnerChange = (value: string) => {
    let newState = { ...totalEquipmentState };

    if (value === EMPTY_PARTY_MEMBER) {
      newState.equipmentDatas[id].mainPlayerPartner1Id = null;
    } else {
      newState.equipmentDatas[id].mainPlayerPartner1Id = parseInt(value);
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
        <TopMenuInput
          select
          value={currentPartnerId === null ? EMPTY_PARTY_MEMBER : currentPartnerId.toString()}
          key={`${id}_partner1`}
          label={labelText}
          onChange={(e: React.ChangeEvent<HTMLInputElement>): void => handlePartnerChange(e.target.value)}
          SelectProps={{
            MenuProps: {
              PaperProps: {
                sx: {
                  backgroundColor: AppConfigurations.backgroundThree,
                },
              },
            },
          }}
        >
          {availablePartyIds.map((partyMemberId) => {
            return PartnerMenuItem(id, partyMemberId, totalEquipmentState.equipmentDatas[id].partyMemberJobAbbrevs[partyMemberId - 1], partyMemberLabelText);
          })}
          {<MenuItem key={`${id}_partner1_empty_menuitem`} value={EMPTY_PARTY_MEMBER} color="white"><Box height={ITEM_TOP_MENU_MIN_HEIGHT} display="flex" justifyContent="flex-end" alignItems="center"><Typography sx={{ fontSize: AppConfigurations.body1FontSize }} color="white">{emptyText}</Typography></Box></MenuItem>};
        </TopMenuInput>
      </CustomFormControl>
    </InputBox>
  );
}

export function Partner2Selection(
  id: number,
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function,
  labelText: string,
  partyMemberLabelText: string,
  emptyText: string
) {
  const handlePartnerChange = (value: string) => {
    let newState = { ...totalEquipmentState };

    if (value === EMPTY_PARTY_MEMBER) {
      newState.equipmentDatas[id].mainPlayerPartner2Id = null;
    } else {
      newState.equipmentDatas[id].mainPlayerPartner2Id = parseInt(value);
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
        <TopMenuInput
          select
          value={currentPartnerId === null ? EMPTY_PARTY_MEMBER : currentPartnerId.toString()}
          key={`${id}_partner2`}
          label={labelText}
          onChange={(e: React.ChangeEvent<HTMLInputElement>): void => handlePartnerChange(e.target.value)}
          SelectProps={{
            MenuProps: {
              PaperProps: {
                sx: {
                  backgroundColor: AppConfigurations.backgroundThree,
                },
              },
            },
          }}
        >
          {availablePartyIds.map((partyMemberId) => {
            return PartnerMenuItem(id, partyMemberId, totalEquipmentState.equipmentDatas[id].partyMemberJobAbbrevs[partyMemberId - 1], partyMemberLabelText);
          })}
          {<MenuItem key={`${id}_partner2_empty_menuitem`} value={EMPTY_PARTY_MEMBER} color="white"><Box height={ITEM_TOP_MENU_MIN_HEIGHT} display="flex" justifyContent="flex-end" alignItems="center"><Typography sx={{ fontSize: AppConfigurations.body1FontSize }} color="white">{emptyText}</Typography></Box></MenuItem>};
        </TopMenuInput>
      </CustomFormControl>
    </InputBox >
  );
}
