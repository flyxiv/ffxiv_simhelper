import { Box, TextField, styled, FormControl } from "@mui/material";
import { ColorConfigurations } from "src/Themes";
import { InputGridItemStyle } from "./Styles";
import { CharacterStats } from "src/types/CharacterStates";

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

export const inputStyleOutLabel = {
  "& .MuiInputBase-input": {
    color: "white",
    backgroundColor: ColorConfigurations.backgroundThree,
    textAlign: "right",
    paddingRight: "30px",
  },
  "& .MuiInputLabel-root": {
    color: ColorConfigurations.primary,
  },
  "& .MuiOutlinedInput-root": {
    "& fieldset": {},
    "&:hover fieldset": {},
    notched: false,
  },
  label: {
    transform: "translate(5px, 5px) scale(0.8)", // 레이블이 Input 내에 위치하도록 합니다.
  },
};

export const inputStyleJobOutLabel = {
  "& .MuiInputBase-input": {
    color: "white",
    backgroundColor: ColorConfigurations.backgroundThree,
    textAlign: "right",
    paddingRight: "30px",
  },
  "& .MuiInputLabel-root": {
    color: ColorConfigurations.primary,
  },
  label: {
    transform: "translate(5px, 5px) scale(0.8)", // 레이블이 Input 내에 위치하도록 합니다.
  },
};

export const Input = styled(TextField)(({ theme }) => inputStyleOutLabel);

export const CustomFormControl = styled(FormControl)(
  ({ theme }) => inputStyleOutLabel
);

const InputBox = styled(Box)`
  ${InputGridItemStyle}
`;

export const CustomInputFormOutLabel: React.FC<InputFormProps> = ({
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

export const CustomTimeInputFormOutLabel: React.FC<InputTimeFormProps> = ({
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
