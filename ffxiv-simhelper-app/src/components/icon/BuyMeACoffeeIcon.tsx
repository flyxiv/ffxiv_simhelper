import { Box, Typography } from "@mui/material";

export function BuyMeACoffee() {
    const handleClick = (event: React.MouseEvent<HTMLAnchorElement, MouseEvent>) => {
        event.preventDefault(); // 기본 링크 동작 방지
        window.open("https://buymeacoffee.com/flyxiv", "_blank"); // 새 창에서 URL 열기
    };

    return (
        <Typography component="a" href="https://buymeacoffee.com/flyxiv" onClick={handleClick} style={{ cursor: 'pointer', textDecoration: 'none' }}>
            <Box component="img" src="https://cdn.buymeacoffee.com/buttons/v2/default-violet.png" alt="Buy me a coffee" height={40} />
        </Typography>
    );
}