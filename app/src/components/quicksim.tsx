import * as React from 'react';
import Button from '@mui/material/Button';

const button_image = process.env.PUBLIC_URL + "/images/jobicons/dps/Ninja.png";
// startIcon=<img src={button_image} alt="Ninja Icon" />>
export default function QuickSimUIButton() {
    return <Button variant="contained">Button</Button>;
}