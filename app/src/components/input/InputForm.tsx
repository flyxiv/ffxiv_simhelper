import { Box, TextField, styled, FormControl } from "@mui/material";
import { ColorConfigurations } from "src/Themes";
import { InputGridItemStyle } from "./Styles";

export interface InputFormProps {
  label: string;
  state: number | string;
  setState: Function;
  handleChange: Function;
}

export const inputStyle = {
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
    padding: "0 16px", // padding을 통해 하이라이트 된 레이블을 조정합니다.
    transform: "translate(14px, 2.5vh) scale(1)", // 레이블이 Input 내에 위치하도록 합니다.
  },
};

export const inputStyleJob = {
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
    padding: "0 16px", // padding을 통해 하이라이트 된 레이블을 조정합니다.
    transform: "translate(14px, 3vh) scale(1)", // 레이블이 Input 내에 위치하도록 합니다.
  },
};

export const Input = styled(TextField)(({ theme }) => inputStyle);

export const CustomFormControl = styled(FormControl)(({ theme }) => inputStyle);

const InputBox = styled(Box)`
  ${InputGridItemStyle}
`;

export const CustomInputForm: React.FC<InputFormProps> = ({
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
