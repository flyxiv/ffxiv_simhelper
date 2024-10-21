import { Box, Link } from "@mui/material";

export function BuyMeACoffee() {
    return (
        <Link href="https://buymeacoffee.com/flyxiv">
            <Box component="img" src="https://cdn.buymeacoffee.com/buttons/v2/default-violet.png" alt="Buy me a coffee" height={40} />
        </Link>
    );
}