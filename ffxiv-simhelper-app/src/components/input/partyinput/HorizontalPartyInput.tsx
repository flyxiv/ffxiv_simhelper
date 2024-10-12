import { Grid, Box, styled } from "@mui/material";
import { PartyMemberJobSelection } from "../jobselection/PartyMemberJobSelection";
import {
  HorizontalInputGridContainerStyle,
  HorizontalInputGridItemStyle,
  InputGridItemStyle,
} from "../Styles";
import {
  inputStyleSimulationResultTextBox,
} from "../SimulationResultTextBox";
import { EquipmentInput } from "../../../types/EquipmentInput";
import { PartyMemberIlvlSelection } from "../PartyMemberIlvlSelection";

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
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function,
  partyMemberIlvlText: string,
  partyMemberLabelText: string,
  tankText: string,
  healerText: string,
  dpsText: string
) {
  let xs = 14;
  return (
    <HorizontalInputGridContainer container>
      <HorizontalInputBox>
        <InputGridItem item xs={xs}>
          <InputBox marginBottom={0.5} key={"time"}>
            <InputJobBox item xs={xs} key={`timeinput`} height={"5vh"}>
              {PartyMemberIlvlSelection(
                partyMemberIlvlText,
                totalEquipmentState,
                setTotalEquipmentState,
              )
              }
            </InputJobBox>
          </InputBox>
        </InputGridItem>
      </HorizontalInputBox>
      {[1, 2, 3, 4, 5, 6, 7].map((playerId) => (
        <HorizontalInputBox key={playerId}>
          <InputGridItem item xs={xs}>
            <InputBox marginBottom={0.5} key={playerId}>
              <InputJobBox item xs={xs} key={`Job-${playerId}`}>
                {PartyMemberJobSelection(
                  playerId,
                  totalEquipmentState,
                  setTotalEquipmentState,
                  partyMemberLabelText,
                  tankText,
                  healerText,
                  dpsText
                )}
              </InputJobBox>
            </InputBox>
          </InputGridItem>
        </HorizontalInputBox>
      ))}

    </HorizontalInputGridContainer>
  );
}
