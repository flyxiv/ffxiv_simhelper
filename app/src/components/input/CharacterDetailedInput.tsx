import { TextField } from "@mui/material";
import { JobSelectionWithState } from "./JobSelectionWithState";
import { CharacterStates } from "src/types/CharacterStates";
import { styled, Grid } from "@mui/material";
import { InputGridContainerStyle, InputGridItemStyle } from "./Styles";
import { ChangeEventHandler } from "react";

interface InputFormProps {
  label: string;
  state: number | string;
  setState: Function;
  handleChange: Function;
}

const InputForm: React.FC<InputFormProps> = ({
  label,
  state,
  setState,
  handleChange,
}) => {
  return (
    <TextField
      label={label}
      InputProps={{
        inputProps: {
          style: { textAlign: "center" },
        },
      }}
      value={state}
      onChange={(e) => {
        handleChange({ value: e.target.value, state, setState });
      }}
      fullWidth
    />
  );
};

interface TextForm {
  value: string;
  state: number | string;
  setState: Function;
}

export function CharacterDetailedInput(mainCharacterState: CharacterStates) {
  let xs = 15;
  let InputGridContainer = styled(Grid)`
    ${InputGridContainerStyle}
  `;
  let InputGridItem = styled(Grid)`
    ${InputGridItemStyle}
  `;

  function handleChange(textForm: TextForm) {
    const value = textForm.value === "" ? "" : parseInt(textForm.value);
    textForm.setState(value);
  }

  return (
    <InputGridContainer container>
      <InputGridItem item xs={xs}>
        {JobSelectionWithState(
          mainCharacterState.jobName,
          mainCharacterState.jobNameSetter
        )}
      </InputGridItem>
      <InputGridItem item xs={xs} key="Weapon Attack">
        <InputForm
          label="Weapon Attack"
          state={mainCharacterState.value.weaponDamage}
          setState={mainCharacterState.setter.weaponAttack}
          handleChange={handleChange}
        />
      </InputGridItem>
      <InputGridItem item xs={xs}>
        <TextField
          label="Main Stat"
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          value={mainCharacterState.value.mainStat}
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.mainStat(value);
          }}
          fullWidth
        />
      </InputGridItem>
      <InputGridItem item xs={xs}>
        <TextField
          label="Critical Strike"
          value={mainCharacterState.value.criticalStrike}
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          fullWidth
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.criticalStrike(value);
          }}
        />
      </InputGridItem>
      <InputGridItem item xs={xs}>
        <TextField
          label="Direct Hit"
          value={mainCharacterState.value.directHit}
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          fullWidth
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.directHit(value);
          }}
        />
      </InputGridItem>
      <InputGridItem item xs={xs}>
        <TextField
          label="Determination"
          value={mainCharacterState.value.determination}
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          fullWidth
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.determination(value);
          }}
        />
      </InputGridItem>
      <InputGridItem item xs={xs}>
        <TextField
          label="Speed"
          value={mainCharacterState.value.speed}
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          fullWidth
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.speed(value);
          }}
        />
      </InputGridItem>
      <InputGridItem item xs={xs}>
        <TextField
          label="Tenacity"
          value={mainCharacterState.value.tenacity}
          InputProps={{
            inputProps: {
              style: { textAlign: "center" },
            },
          }}
          fullWidth
          onChange={(e) => {
            const value = e.target.value === "" ? "" : parseInt(e.target.value);
            mainCharacterState.setter.tenacity(value);
          }}
        />
      </InputGridItem>
    </InputGridContainer>
  );
}
