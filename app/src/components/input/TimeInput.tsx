import { CustomFormControl, InputFormProps } from "./basicform/BasicInputForm";
import { MenuItem, Box, InputLabel, Select } from "@mui/material";

export function TimeInput(inputFormProps: InputFormProps) {
  return (
    <CustomFormControl fullWidth>
      <InputLabel id="JobSelect">Combat Time</InputLabel>
      <Select
        labelId={inputFormProps.label}
        id="job-select-{id}"
        value={inputFormProps.state}
        key="job-select-{id}"
        label="Job Name"
        onChange={(event) => {
          inputFormProps.setState(event.target.value);
          console.log(event.target.value);
        }}
      >
        {TimeMenu(0, 30)}
        {TimeMenu(1, 0)}
        {TimeMenu(1, 30)}
        {TimeMenu(2, 0)}
        {TimeMenu(2, 30)}
        {TimeMenu(3, 0)}
        {TimeMenu(3, 30)}
        {TimeMenu(4, 0)}
        {TimeMenu(4, 30)}
        {TimeMenu(5, 0)}
      </Select>
    </CustomFormControl>
  );
}

function formatTime(minutes: number, seconds: number) {
  const formattedMinutes = String(minutes).padStart(2, "0");
  const formattedSeconds = String(seconds).padStart(2, "0");

  return `${formattedMinutes}:${formattedSeconds}`;
}

export function TimeMenu(minutes: number, seconds: number) {
  return (
    <MenuItem value={minutes * 60 + seconds}>
      <Box alignItems={"center"}>{formatTime(minutes, seconds)}</Box>
    </MenuItem>
  );
}
