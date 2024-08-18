import { MenuItem, InputLabel, Select } from "@mui/material";
import { CustomFormControl } from "./basicform/BasicInputForm";
import { CharacterStates } from "src/types/CharacterStates";
import { partnerJobs } from "./Utils";
import { styled, Box } from "@mui/material";
import { InputGridItemStyle } from "./Styles";

const InputBox = styled(Box)`
  ${InputGridItemStyle}
`;

export function Partner1Selection(
  characterState: CharacterStates,
  availablePartyIds: number[]
) {
  if (!partnerJobs.includes(characterState.jobAbbrev)) {
    return <></>;
  }

  if (characterState.partner1Id === null) {
    characterState.setPartner1Id(-1);
  }

  let availablePartners: number[] = [];
  for (let i = 0; i < availablePartyIds.length; i++) {
    let partyId = availablePartyIds[i];
    if (partyId === characterState.partner2Id) {
      continue;
    }
    availablePartners.push(availablePartyIds[i]);
  }

  return (
    <InputBox>
      <CustomFormControl fullWidth>
        <InputLabel id="PartnerSelection1">Partner 1</InputLabel>
        <Select
          labelId="Partner 1"
          id="partner-select-1"
          value={characterState.partner1Id}
          key="partner-select-1"
          label="Partner 1"
          onChange={(event) => {
            if (
              event.target.value === -1 ||
              event.target.value === null ||
              typeof event.target.value === "string"
            ) {
              characterState.setPartner1Id(-1);
            } else {
              characterState.setPartner1Id(event.target.value);
            }
          }}
        >
          {availablePartners.map((partnerId) => {
            return (
              <MenuItem value={partnerId}>Party Member {partnerId}</MenuItem>
            );
          })}
          <MenuItem value={-1}>No Partner</MenuItem>
        </Select>
      </CustomFormControl>
    </InputBox>
  );
}
