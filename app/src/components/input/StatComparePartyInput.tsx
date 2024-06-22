import { Grid, Box, styled } from "@mui/material";
import { JobSelection } from "./JobSelection";
import {
  HorizontalInputGridContainerStyle,
  HorizontalInputGridItemStyle,
  InputGridItemStyle,
} from "./Styles";
import {
  inputStyleJobOutLabel,
  CustomTimeInputFormOutLabel,
} from "./InputFormOutLabel";
import { handleChange } from "./CharacterDetailedInput";

interface TextTimeForm {
  value: string;
  state: number;
  setState: Function;
}

const HorizontalInputGridContainer = styled(Grid)`
  ${HorizontalInputGridContainerStyle}
`;

const InputGridItem = styled(Grid)`
  ${InputGridItemStyle}
`;
const InputBox = styled(Box)`
  ${InputGridItemStyle}
`;
const HorizontalInputBox = styled(Box)`
  ${HorizontalInputGridItemStyle}
`;
const InputJobBox = styled(Grid)`
  ${inputStyleJobOutLabel}
`;
export function handleTimeChange(textForm: TextTimeForm) {
  const value = textForm.value === "" ? "" : parseInt(textForm.value);
  textForm.setState(value);
}

export function StatComparePartyInput(
  playerIds: number[],
  partyJobs: string[],
  partySetter: React.Dispatch<React.SetStateAction<string[]>>,
  time: number,
  timeSetter: Function
) {
  let xs = 15;
  return (
    <HorizontalInputGridContainer container>
      <HorizontalInputBox marginBottom={0.5}>
        <InputGridItem item xs={xs}>
          <CustomTimeInputFormOutLabel
            label="Combat Time(Seconds)"
            state={time}
            setState={timeSetter}
            handleChange={handleTimeChange}
          />
        </InputGridItem>
      </HorizontalInputBox>
      {playerIds.map((playerId) => (
        <HorizontalInputBox marginBottom={0.5} key={playerId}>
          <InputGridItem item xs={xs}>
            <InputBox marginBottom={0.5} key={playerId}>
              <InputJobBox item xs={xs} key={`Job-${playerId}`}>
                {JobSelection(playerId, partyJobs, partySetter)}
              </InputJobBox>
            </InputBox>
          </InputGridItem>
        </HorizontalInputBox>
      ))}
    </HorizontalInputGridContainer>
  );
}
