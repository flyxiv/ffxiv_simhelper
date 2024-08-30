import { useState } from "react";
import { Box, styled } from "@mui/material";
import { GearCompareRequestSaveName } from "../App"
import { EquipmentSelectionMenu } from "../components/input/basicform/EquipmentInputForm";
import { StatPowerSummary } from "../components/container/StatSummary";
import { HorizontalPartyInput } from "../components/input/partyinput/HorizontalPartyInput";
import { MENU_WIDTH_VW, LeftMenuWithLoadout } from "../components/container/LeftMenu";
import { ColorConfigurations } from "../Themes";
import { Footer } from "../components/basic/Footer";
import { AppHeader } from "../components/image/AppHeader";
import { SelectionTitle } from "../components/basic/SelectionTitle";
import { QuickSimBottomMenu } from "../components/container/BottomMenu";
import { CustomizeBoardStyle, EquipmentBoardStyle, InputContainerStyle } from "./Styles";
import { defaultDoubleEquipmentInput } from "../const/DefaultDoubleEquipmentInput";
import { EquipmentInput } from "../types/EquipmentInput";

let INPUT_CONTAINER_WIDTH = "80vw";

let GearCompareInputContainer = styled(Box)`
  ${InputContainerStyle(INPUT_CONTAINER_WIDTH)} 
`;

let CustomizeBoard = styled(Box)`
  ${CustomizeBoardStyle}
`

let EquipmentBoard = styled(Box)`
  ${EquipmentBoardStyle}
`

export function isNotValid(input: EquipmentInput) {
    if (input.equipmentDatas === null) {
        return true;
    }

    return false;
}


export function GearCompare() {
    let mostRecentInputState = localStorage.getItem(GearCompareRequestSaveName);
    let mostRecentInput = null;

    if (mostRecentInputState === null) {
        mostRecentInput = defaultDoubleEquipmentInput();
    } else {
        mostRecentInput = JSON.parse(
            mostRecentInputState
        ) as EquipmentInput;
    }

    if (isNotValid(mostRecentInput)) {
        mostRecentInput = defaultDoubleEquipmentInput();
    }

    const [totalDoubleState, setTotalDoubleState] = useState(
        mostRecentInput
    );

    let bodyWidth = 100 - MENU_WIDTH_VW;

    return (
        <></>
    );
}
/*        <>
            <Box display="flex" sx={{ backgroundColor: ColorConfigurations.backgroundOne }} width="100vw">
                {GearCompareLeftMenu(totalDoubleState, setTotalDoubleState)}
                <Box width={`${bodyWidth}vw`}>
                    {AppHeader()}
                    <Box alignContent={"center"}>
                        <GearCompareInputContainer justifyContent={"center"}>
                            {SelectionTitle("1. Input Your Info")}
                            <EquipmentBoard>
                                {EquipmentSelectionMenu(0, totalState, setTotalState)}
                            </EquipmentBoard>
                            <EquipmentBoard>
                                {EquipmentSelectionMenu(0, totalState, setTotalState)}
                            </EquipmentBoard>
                        </GearCompareInputContainer>

                        <GearCompareInputContainer paddingTop={20}>
                            {SelectionTitle("2. Additional Settings")}
                            <CustomizeBoard>
                                {HorizontalPartyInput(
                                    totalState,
                                    setTotalState
                                )}
                            </CustomizeBoard>
                        </GearCompareInputContainer>

                        <GearCompareInputContainer marginTop={10}>
                            {SelectionTitle("3. Specific Player Power")}
                            <Box display="flex" justifyContent="center" paddingBottom={"20vh"}>
                                {StatPowerSummary(totalState)}
                            </Box>
                        </GearCompareInputContainer>

                        {QuickSimBottomMenu(totalState)}
                    </Box>
                    {Footer()}
                </Box>
            </Box >
        </>
*/