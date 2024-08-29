import { Box, Typography } from '@mui/material';

export function QuickSimBottomMenu() {
    return (
        <Box
            sx={{
                position: 'fixed',
                bottom: 0,
                left: 0,
                width: '100%',
                backgroundColor: '#ffffff',
                boxShadow: '0 -2px 10px rgba(0,0,0,0.1)',
                display: 'flex',
                justifyContent: 'space-around',
                alignItems: 'center',
                padding: '10px',
                zIndex: 1000,  // Ensure it's above other elements
            }}
        >
            <Typography>Test</Typography>
        </Box>
    );
};
