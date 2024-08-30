import { Box, TextField, styled } from "@mui/material";
import { ColorConfigurations } from "../..//Themes";
import { InputGridItemStyle } from "./Styles";
import { CharacterStats } from "../../types/CharacterStates";
import { EquipmentInput, SingleEquipmentInputSaveState } from "../../types/EquipmentInput";
import { ITEM_MIN_HEIGHT } from "../items/Styles";

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
    transform: "translate(5px, 5px) scale(0.8)",
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

export function SimulationResultTimeTextBox(label: string, totalEquipmentState: EquipmentInput, setTotalState: Function) {
  return (
    <Input
      label={label}
      value={totalEquipmentState.equipmentDatas[0].combatTimeMillisecond / 1000}
      onChange={(e) => {
        let newTimeSeconds = parseInt(e.target.value);

        let newTotalState = { ...totalEquipmentState };
        newTotalState.equipmentDatas.forEach((data: SingleEquipmentInputSaveState) => {
          data.combatTimeMillisecond = newTimeSeconds * 1000;
        });

        setTotalState({ ...newTotalState });
      }}
      fullWidth
      sx={{
        '& .MuiInputBase-input': {
          height: ITEM_MIN_HEIGHT,
          textAlign: "center",
        },
      }}
    />
  );
};
