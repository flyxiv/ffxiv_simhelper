import { Box } from '@mui/material';
import { ColorConfigurations } from '../../Themes';
import { MENU_WIDTH_VW } from './LeftMenu';
import { StatSummary } from './StatSummary';
import { EquipmentInput } from '../../types/EquipmentInput';
import { QuickSimRequestButton } from '../basic/QuickSimRequestButton';
import { GEAR_COMPARE_INPUT_CONTAINER_WIDTH } from '../../page/GearCompare';

export function QuickSimBottomMenu(totalState: EquipmentInput) {
    return (
        <Box
            sx={{
                position: 'fixed',
                bottom: 0,
                left: `${MENU_WIDTH_VW}vs`,
                width: `${100 - MENU_WIDTH_VW}vw`,
                backgroundColor: ColorConfigurations.backgroundTwo,
                display: 'flex',
                justifyContent: 'space-around',
                alignItems: 'center',
                zIndex: 1000,
            }}
        >
            <Box paddingY={3} display="flex" flexDirection="column" alignContent="center">
                {StatSummary(totalState.equipmentDatas[0])}
                <Box display="inline-block" margin="auto" paddingTop={2}>
                    {QuickSimRequestButton(totalState)}
                </Box>
            </Box>
        </Box>
    );
};

export function GearCompareBottomMenu(totalState: EquipmentInput) {
    return (
        <Box
            sx={{
                position: 'fixed',
                bottom: 0,
                left: `${MENU_WIDTH_VW}vs`,
                width: `${100 - MENU_WIDTH_VW}vw`,
                backgroundColor: ColorConfigurations.backgroundTwo,
                display: 'flex',
                justifyContent: 'space-around',
                alignItems: 'center',
                zIndex: 1000,
            }}
        >
            <Box paddingY={3} display="flex" flexDirection="column" alignContent="center">
                <Box display="flex" justifyContent="space-evenly" width={GEAR_COMPARE_INPUT_CONTAINER_WIDTH}>
                    {StatSummary(totalState.equipmentDatas[0])}
                    {StatSummary(totalState.equipmentDatas[1])}
                </Box>

                <Box display="inline-block" margin="auto" paddingTop={2}>
                    {QuickSimRequestButton(totalState)}
                </Box>
            </Box>
        </Box>
    );
};
