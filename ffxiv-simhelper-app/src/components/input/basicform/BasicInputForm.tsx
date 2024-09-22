import { Box, TextField, styled, FormControl } from "@mui/material";
import { AppConfigurations } from "../../../Themes";
import { InputGridItemStyle } from "../Styles";
import { CharacterStats } from "../../../types/CharacterStates";

export interface InputFormProps {
  label: string;
  state: CharacterStats;
  field: keyof CharacterStats;
  setState: Function;
  handleChange: Function;
}

export interface InputTimeFormProps {
  label: string;
  state: number;
  setState: Function;
  handleChange: Function;
}

export const inputStyleEquipment = {
  "& .MuiInputBase-input": {
    color: "white",
    backgroundColor: AppConfigurations.backgroundThree,
    textAlign: "right",
    height: "5vh",
  },
  "& .MuiInputLabel-root": {
    color: AppConfigurations.primary,
  },
  "& .MuiOutlinedInput-root": {
    "& fieldset": {},
    "&:hover fieldset": {},
    notched: false,
  },
};

export const inputStyleTime = {
  "& .MuiInputBase-input": {
    color: "white",
    backgroundColor: AppConfigurations.backgroundThree,
    textAlign: "right",
  },
  "& .MuiInputLabel-root": {
    color: AppConfigurations.primary,
  },
  "& .MuiOutlinedInput-root": {
    "& fieldset": {},
    "&:hover fieldset": {},
    notched: false,
  },
};

export const inputStyleJob = {
  "& .MuiInputBase-input": {
    color: "white",
    backgroundColor: AppConfigurations.backgroundThree,
    textAlign: "right",
    paddingRight: "30px",
  },
  "& .MuiInputLabel-root": {
    color: AppConfigurations.primary,
  },
  label: {
    padding: "0 16px", // padding을 통해 하이라이트 된 레이블을 조정합니다.
    transform: "translate(14px, 3vh) scale(1)", // 레이블이 Input 내에 위치하도록 합니다.
  },
};

export const Input = styled(TextField)(({ }) => inputStyleEquipment);

export const CustomFormControl = styled(FormControl)(
  ({ }) => inputStyleEquipment
);

export const CustomTimeFormControl = styled(FormControl)(
  ({ }) => inputStyleTime
);

const InputBox = styled(Box)`
  ${InputGridItemStyle}
`;

export const CustomInputForm: React.FC<InputFormProps> = ({
  label,
  field,
  state,
  setState,
  handleChange,
}) => {
  return (
    <InputBox marginBottom={0.5}>
      <Input
        label={label}
        value={state[field]}
        onChange={(e) => {
          handleChange({ value: e.target.value, state, setState, field });
        }}
        fullWidth
      />
    </InputBox>
  );
};

export const CustomTimeInputForm: React.FC<InputTimeFormProps> = ({
  label,
  state,
  setState,
  handleChange,
}) => {
  return (
    <InputBox marginBottom={0.5}>
      <Input
        label={label}
        value={state}
        onChange={(e) => {
          handleChange({ value: e.target.value, state, setState });
        }}
        fullWidth
      />
    </InputBox>
  );
};

export const CustomPartnerInputForm: React.FC<InputTimeFormProps> = ({
  label,
  state,
  setState,
  handleChange,
}) => {
  return (
    <InputBox marginBottom={0.5}>
      <Input
        label={label}
        value={state}
        onChange={(e) => {
          handleChange({ value: e.target.value, state, setState });
        }}
        fullWidth
      />
    </InputBox>
  );
};
