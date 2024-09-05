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
import { EquipmentInput } from "../../../types/EquipmentInput";

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
) {
  let xs = 14;
  return (
    <HorizontalInputGridContainer container>
      <HorizontalInputBox>
        <InputGridItem item xs={xs}>
          <InputBox marginBottom={0.5} key={"time"}>
            <InputJobBox item xs={xs} key={`timeinput`} height={"5vh"}>
              {SimulationResultTimeTextBox(
                "Combat Time(Seconds)",
                totalEquipmentState,
                setTotalEquipmentState,
                false
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
                  setTotalEquipmentState
                )}
              </InputJobBox>
            </InputBox>
          </InputGridItem>
        </HorizontalInputBox>
      ))}

    </HorizontalInputGridContainer>
  );
}


export function HorizontalDefaultInput(
  totalEquipmentState: EquipmentInput,
  setTotalEquipmentState: Function,
) {
  return (
    <Box width="50%">
      {SimulationResultTimeTextBox(
        "Combat Time(Seconds)",
        totalEquipmentState,
        setTotalEquipmentState)
      }
    </Box>
  )
}
