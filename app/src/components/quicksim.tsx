import { Button } from '@mui/material'
// startIcon=<img src={button_image} alt="Ninja Icon" />>
const buttonImagePath = process.env.PUBLIC_URL + "/images/quickstart.svg";

export const QuickSimUIButton = () => {
    return (
        <div>
            <Button variant='contained'>
                <img src={buttonImagePath} alt="quicksim-button"/>
            </Button>
        </div>
    )
}