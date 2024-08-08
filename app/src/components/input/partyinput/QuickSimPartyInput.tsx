import { Grid, Box, styled } from "@mui/material";
import { PartyMemberJobSelection } from "src/components/input/jobselection/PartyMemberJobSelection";
import { InputGridContainerStyle, InputGridItemStyle } from "../Styles";
import {
  inputStyleJob,
  CustomTimeInputForm,
} from "src/components/input/basicform/BasicInputForm";

interface TextTimeForm {
  value: string;
  state: number;
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
export function handleTimeChange(textForm: TextTimeForm) {
  const value = textForm.value === "" ? "" : parseInt(textForm.value);
  textForm.setState(value);
}

export function QuickSimPartyInput(
  playerIds: number[],
  partyJobs: string[],
  partySetter: React.Dispatch<React.SetStateAction<string[]>>,
  availablePartyIds: number[],
  setAvailablePartyIds: Function,
  time: number,
  timeSetter: Function
) {
  let xs = 15;
  return (
    <InputGridContainer container>
      <InputBox marginBottom={0.5}>
        <InputGridItem item xs={xs}>
          <CustomTimeInputForm
            label="Combat Time(Seconds)"
            state={time}
            setState={timeSetter}
            handleChange={handleTimeChange}
          />
        </InputGridItem>
      </InputBox>
      {playerIds.map((playerId) => (
        <InputBox marginBottom={0.5} key={playerId}>
          <InputGridItem item xs={xs}>
            <InputBox marginBottom={0.5} key={playerId}>
              <InputJobBox item xs={xs} key={`Job-${playerId}`}>
                {PartyMemberJobSelection(
                  playerId,
                  partyJobs,
                  partySetter,
                  availablePartyIds,
                  setAvailablePartyIds
                )}
              </InputJobBox>
            </InputBox>
          </InputGridItem>
        </InputBox>
      ))}
    </InputGridContainer>
  );
}
