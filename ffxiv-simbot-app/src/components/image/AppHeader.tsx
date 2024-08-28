import { Box, Link } from "@mui/material";
import { Logo } from "./Logo";

export function AppHeader() {
    return (
        <Box className="App">
            <header className="App-header">
                <Link href="/">
                    <Logo />
                </Link>
            </header>
        </Box>
    )
}