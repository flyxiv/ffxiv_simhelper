import { Grid, Box, styled } from "@mui/material";
import { PartyMemberJobSelection } from "../jobselection/PartyMemberJobSelection";
import {
  HorizontalInputGridContainerStyle,
  HorizontalInputGridItemStyle,
  InputGridItemStyle,
} from "../Styles";
import {
  inputStyleSimulationResultTextBox,
  SimulationResultTimeTextBox,
} from "../SimulationResultTextBox";

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
  ${inputStyleSimulationResultTextBox}
`;
export function handleTimeChange(textForm: TextTimeForm) {
  const value = textForm.value === "" ? "" : parseInt(textForm.value);
  textForm.setState(value);
}

export function StatComparePartyInput(
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
    <HorizontalInputGridContainer container>
      <HorizontalInputBox marginBottom={0.5}>
        <InputGridItem item xs={xs}>
          <SimulationResultTimeTextBox
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
        </HorizontalInputBox>
      ))}
    </HorizontalInputGridContainer>
  );
}
