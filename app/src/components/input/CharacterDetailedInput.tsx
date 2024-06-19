import { JobSelectionWithState } from "./JobSelectionWithState";
import { CharacterStates } from "src/types/CharacterStates";
import { styled, Box, Grid } from "@mui/material";
import { CustomInputForm, inputStyleJob } from "./InputForm";
import { InputGridContainerStyle, InputGridItemStyle } from "./Styles";

interface TextForm {
  value: string;
  state: number | string;
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
  textForm.setState(value);
}

export function CharacterDetailedInput(mainCharacterState: CharacterStates) {
  let xs = 15;
  return (
    <InputGridContainer container>
      <InputBox marginBottom={1}>
        <InputJobBox item xs={xs} key="Job">
          {JobSelectionWithState(
            mainCharacterState.jobName,
            mainCharacterState.jobNameSetter
          )}
        </InputJobBox>
      </InputBox>
      <InputBox marginBottom={0.5}>
        <InputGridItem item xs={xs} key="Weapon Attack">
          <CustomInputForm
            label="Weapon Attack"
            state={mainCharacterState.value.weaponDamage}
            setState={mainCharacterState.setter.weaponAttack}
            handleChange={handleChange}
          />
        </InputGridItem>
      </InputBox>
      <InputBox marginBottom={0.5}>
        <InputGridItem item xs={xs}>
          <CustomInputForm
            label="Main Stat"
            state={mainCharacterState.value.mainStat}
            setState={mainCharacterState.setter.mainStat}
            handleChange={handleChange}
          />
        </InputGridItem>
      </InputBox>
      <InputBox marginBottom={0.5}>
        <InputGridItem item xs={xs}>
          <CustomInputForm
            label="Critical Strike"
            state={mainCharacterState.value.criticalStrike}
            setState={mainCharacterState.setter.criticalStrike}
            handleChange={handleChange}
          />
        </InputGridItem>
      </InputBox>
      <InputBox marginBottom={0.5}>
        <InputGridItem item xs={xs}>
          <CustomInputForm
            label="Direct Hit"
            state={mainCharacterState.value.directHit}
            setState={mainCharacterState.setter.directHit}
            handleChange={handleChange}
          />
        </InputGridItem>
      </InputBox>
      <InputBox marginBottom={0.5}>
        <InputGridItem item xs={xs}>
          <CustomInputForm
            label="Determination"
            state={mainCharacterState.value.determination}
            setState={mainCharacterState.setter.determination}
            handleChange={handleChange}
          />
        </InputGridItem>
      </InputBox>
      <InputBox marginBottom={0.5}>
        <InputGridItem item xs={xs}>
          <CustomInputForm
            label="Speed"
            state={mainCharacterState.value.speed}
            setState={mainCharacterState.setter.speed}
            handleChange={handleChange}
          />
        </InputGridItem>
      </InputBox>
      <InputBox marginBottom={0.5}>
        <InputGridItem item xs={xs}>
          <CustomInputForm
            label="Tenacity"
            state={mainCharacterState.value.tenacity}
            setState={mainCharacterState.setter.tenacity}
            handleChange={handleChange}
          />
        </InputGridItem>
      </InputBox>
    </InputGridContainer>
  );
}
