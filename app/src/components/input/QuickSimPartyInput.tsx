import { Grid, TextField, Box, styled } from "@mui/material";
import { JobSelection } from "./JobSelection";
import { InputGridContainerStyle, InputGridItemStyle } from "./Styles";
import { inputStyleJob, CustomInputForm } from "./InputForm";
import { handleChange } from "./CharacterDetailedInput";
import { TimeInput, TimeMenu } from "./TimeInput";

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

export function QuickSimPartyInput(
  playerIds: number[],
  partyJobs: string[],
  partySetter: React.Dispatch<React.SetStateAction<string[]>>,
  time: number,
  timeSetter: Function
) {
  let xs = 15;
  return (
    <InputGridContainer container>
      <InputBox marginBottom={1.0}>
        <InputJobBox item xs={xs}>
          <TimeInput
            state={time}
            setState={timeSetter}
            handleChange={handleChange}
            label="Time"
          />
        </InputJobBox>
      </InputBox>
      {playerIds.map((playerId) => (
        <InputBox marginBottom={0.5} key={playerId}>
          <InputGridItem item xs={xs}>
            <InputBox marginBottom={0.5} key={playerId}>
              <InputJobBox item xs={xs} key={`Job-${playerId}`}>
                {JobSelection(playerId, partyJobs, partySetter)}
              </InputJobBox>
            </InputBox>
          </InputGridItem>
        </InputBox>
      ))}
    </InputGridContainer>
  );
}
