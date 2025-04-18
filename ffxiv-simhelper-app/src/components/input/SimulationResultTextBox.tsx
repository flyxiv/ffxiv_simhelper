import { Box, TextField, styled } from "@mui/material";
import { AppConfigurations } from "../../Themes";
import { InputGridItemStyle } from "./Styles";
import { CharacterStats } from "../../types/CharacterStates";
import {
  EquipmentInput,
  SingleEquipmentInputSaveState,
} from "../../types/EquipmentInput";
import { CustomTimeFormControl } from "./basicform/BasicInputForm";
import { TopMenuInput } from "./basicform/EquipmentInputForm";

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

export const inputStyleSimulationResultTextBox = {
  "& .MuiInputBase-input": {
    color: "white",
    backgroundColor: AppConfigurations.backgroundThree,
    textAlign: "right",
    paddingRight: "30px",
    fontSize: AppConfigurations.body1FontSize,
  },
  "& .MuiInputLabel-root": {
    color: AppConfigurations.primary,
  },
  "& .MuiOutlinedInput-root": {
    "& fieldset": {},
    "&:hover fieldset": {},
    notched: false,
  },
  label: {
    transform: "translate(10px, 0) scale(0.8)",
  },
};

export const Input = styled(TextField)(
  ({ }) => inputStyleSimulationResultTextBox
);

const InputBox = styled(Box)`
  ${InputGridItemStyle}
`;

export const SimulationResultTextBox: React.FC<InputFormProps> = ({
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

const TIME_UPPER_LIMIT = 600;

export function SimulationUpperInputTimeTextBox(
  label: string,
  totalEquipmentState: EquipmentInput,
  setTotalState: Function,
) {
  return (
    <CustomTimeFormControl fullWidth>
      <TopMenuInput
        label={label}
        id={label}
        value={
          totalEquipmentState.equipmentDatas[0].combatTimeMillisecond / 1000
        }
        key={label}
        onChange={(e) => {
          let newTimeSeconds = parseInt(e.target.value);
          if (isNaN(newTimeSeconds)) {
            newTimeSeconds = 0;
          }

          let newTotalState = { ...totalEquipmentState };
          newTotalState.equipmentDatas.forEach(
            (data: SingleEquipmentInputSaveState) => {
              data.combatTimeMillisecond =
                Math.min(newTimeSeconds, TIME_UPPER_LIMIT) * 1000;
            }
          );

          setTotalState({ ...newTotalState });
        }}
        sx={{
          "&.Mui-focused .MuiOutlinedInput-notchedOutline": {
            borderColor: "transparent",
          },
        }}
        InputProps={{ sx: { fontSize: AppConfigurations.body1FontSize } }}
      />
    </CustomTimeFormControl>
  );
}
