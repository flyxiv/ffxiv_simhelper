import { MainPlayerJobSelection } from "./jobselection/MainPlayerJobSelection";
import { CharacterStates, CharacterStats } from "src/types/CharacterStates";
import { styled, Box, Grid } from "@mui/material";
import { CustomInputForm, inputStyleJob } from "./basicform/BasicInputForm";
import { InputGridContainerStyle, InputGridItemStyle } from "./Styles";
import { Partner1Selection } from "./PartnerSelection";

interface TextForm {
  value: string;
  field: string;
  state: CharacterStats;
  setState: Function;
}

const InputGridContainer = styled(Grid)`
  ${InputGridContainerStyle}
`;

const InputGridItem = styled(Grid)`
  ${InputGridItemStyle}
`;
const InputBox = styled(Box)`
  ${InputGridItemStyle}
`;
const InputJobBox = styled(Grid)`
  ${inputStyleJob}
`;

export function handleChange(textForm: TextForm) {
  const value = textForm.value === "" ? "" : parseInt(textForm.value);
  const stats = { ...textForm.state, [textForm.field]: value };
  textForm.setState(stats);
}

// 직업뿐만 아니라 세세한 주/부스탯까지 입력
export function CharacterDetailedInput(
  mainCharacterState: CharacterStates,
  availablePartyIds: number[]
) {
  let xs = 15;
  let statNameAndKeys = [
    { name: "Weapon Damage", field: "weaponDamage" },
    { name: "Main Stat", field: "mainStat" },
    { name: "Critical Strike", field: "criticalStrike" },
    { name: "Direct Hit", field: "directHit" },
    { name: "Determination", field: "determination" },
    { name: "Speed", field: "speed" },
    { name: "Tenacity", field: "tenacity" },
  ];

  return (
    <InputGridContainer container>
      <InputBox marginBottom={1}>
        <InputJobBox item xs={xs} key="Job">
          {MainPlayerJobSelection(
            mainCharacterState.jobAbbrev,
            mainCharacterState.jobNameSetter,
            null
          )}
        </InputJobBox>
      </InputBox>
      {statNameAndKeys.map((statNameAndKey) => {
        return (
          <InputBox marginBottom={0.5}>
            <InputGridItem item xs={xs} key={statNameAndKey.name}>
              <CustomInputForm
                label={statNameAndKey.name}
                state={mainCharacterState.stats}
                field={statNameAndKey.field as keyof CharacterStats}
                setState={mainCharacterState.setStats}
                handleChange={handleChange}
              />
            </InputGridItem>
          </InputBox>
        );
      })}
      {Partner1Selection(mainCharacterState, availablePartyIds)}
    </InputGridContainer>
  );
}
