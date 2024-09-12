import { Box, Link } from "@mui/material";
import { Logo } from "./Logo";

export function AppHeader() {
    return (
        <Box className="LogoBox" display="flex" width="100%" justifyContent="center">
            <Link href="/">
                <Logo />
            </Link>
        </Box>
    )
}