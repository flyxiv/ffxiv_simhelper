import { Box, Typography } from "@mui/material";

export function Footer() {
    return (
        <Box className="Footer" component="footer" sx={{ width: "100%", border: "1px solid black", textAlign: "center" }} display="flex" justifyContent="center" color="white">
            <Typography variant="caption">
                <p>Creator: Fly Ninetynine@Aegis(Elemental DC)</p>
                <p>Email: ns090200@gmail.com</p>
                <p>License: MIT</p>
            </Typography>
        </Box>
    )
}