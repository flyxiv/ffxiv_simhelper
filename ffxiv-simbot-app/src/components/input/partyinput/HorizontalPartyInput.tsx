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
import { SingleEquipmentInputSaveState } from "../../../types/SingleEquipmentInputSaveState";

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

export function HorizontalPartyInput(
  totalState: SingleEquipmentInputSaveState,
  setTotalState: Function
) {
  let xs = 15;
  return (
    <HorizontalInputGridContainer container>
      <HorizontalInputBox marginBottom={0.5}>
        <InputGridItem item xs={xs}>
          {SimulationResultTimeTextBox(
            "Combat Time(Seconds)",
            totalState,
            setTotalState)
          }
        </InputGridItem>
      </HorizontalInputBox>
      {[1, 2, 3, 4, 5, 6, 7].map((playerId) => (
        <HorizontalInputBox marginBottom={0.5} key={playerId}>
          <InputGridItem item xs={xs}>
            <InputBox marginBottom={0.5} key={playerId}>
              <InputJobBox item xs={xs} key={`Job-${playerId}`}>
                {PartyMemberJobSelection(
                  playerId,
                  totalState,
                  setTotalState
                )}
              </InputJobBox>
            </InputBox>
          </InputGridItem>
        </HorizontalInputBox>
      ))}
    </HorizontalInputGridContainer>
  );
}
