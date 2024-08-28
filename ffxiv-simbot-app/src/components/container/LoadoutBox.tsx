import { Box, Button, styled, TextField, Typography } from "@mui/material";
import { ColorConfigurations } from "../../Themes";
import { jobAbbrevToJobIconPath } from "../icon/jobicon/JobIconFactory";
import { useState } from "react";
import { SingleEquipmentInputSaveState } from "../../types/SingleEquipmentInputSaveState";
import { defaultSingleEquipmentInput } from "../../const/DefaultSingleEquipmentInput";


export const inputStyle = {
    width: "60%",
    "& .MuiInputBase-input": {
        color: "black",
        backgroundColor: "white",
        textAlign: "left",
        height: "0.5vh"
    },
    "& .MuiInputLabel-root": {
        color: ColorConfigurations.primary,
    },
    "& .MuiOutlinedInput-root": {
        "& fieldset": {},
        "&:hover fieldset": {},
        notched: false,
    },
    label: {
        transform: "translate(0, -1.5vh) scale(0.8)", // 레이블이 Input 내에 위치하도록 합니다.
    },
};

export const LoadoutInput = styled(TextField)(({ }) => inputStyle);


export function SingleLoadoutBox(loadoutId: number, simulationName: string, totalState: SingleEquipmentInputSaveState, setTotalState: Function) {
    let [textFieldInputLoadoutName, setTextFieldInputLoadoutName] = useState("");
    let loadoutSaveKey = `${simulationName}-${loadoutId}`;
    let loadoutNameSaveKey = `${simulationName}-loadoutName-${loadoutId}`;

    let savedLoadoutName = localStorage.getItem(loadoutNameSaveKey);
    if (savedLoadoutName === null) {
        savedLoadoutName = "Default Loadout";
    }
    let [loadoutName, setLoadoutName] = useState(savedLoadoutName);


    return (
        <Box sx={{ backgroundColor: ColorConfigurations.backgroundFour, borderRadius: 4, paddingX: 2 }}>
            <Box sx={{ display: "flex", alignItems: "center", padding: 2 }}>
                <Typography variant="h2" color={ColorConfigurations.primary} align="left" fontSize={15}>{loadoutId}. {loadoutName}</Typography>
                <Box marginX={1}>
                    <img
                        src={jobAbbrevToJobIconPath(totalState.mainPlayerJobAbbrev)}
                        alt={totalState.mainPlayerJobAbbrev}
                        width={25}
                        height={25}
                        style={{ verticalAlign: "middle" }}
                    />
                </Box>
            </Box>
            <Box padding={2} display="flex" >
                <LoadoutInput
                    label="New Loadout Name"
                    value={textFieldInputLoadoutName}
                    onChange={(e) => {
                        setTextFieldInputLoadoutName(e.target.value)
                    }}
                    fullWidth
                    sx={{ backgroundColor: "white" }}
                />
                {LoadoutOverwriteButton(loadoutSaveKey, loadoutNameSaveKey, textFieldInputLoadoutName, totalState, setLoadoutName, setTextFieldInputLoadoutName)}
                {LoadoutLoadButton(loadoutSaveKey, setTotalState, setTextFieldInputLoadoutName)}
            </Box>
        </Box >
    )
}

function LoadoutOverwriteButton(loadoutSaveKey: string, loadoutNameSaveKey: string, textFieldInputLoadoutName: string, totalState: SingleEquipmentInputSaveState, setLoadoutName: Function, setTextFieldInputLoadoutName: Function) {
    return (
        <Button sx={{ marginX: 2, backgroundColor: ColorConfigurations.primary, color: "black", borderRadius: 2 }} onClick={(_) => {
            localStorage.setItem(loadoutSaveKey, JSON.stringify(totalState)); setLoadoutName("");
            localStorage.setItem(loadoutNameSaveKey, textFieldInputLoadoutName);
            setLoadoutName(textFieldInputLoadoutName);
            setTextFieldInputLoadoutName("");
        }}>
            <Typography sx={{ fontWeight: "bold", fontSize: 12 }}>Overwrite</Typography>
        </Button >
    )
}

function LoadoutLoadButton(loadoutSaveKey: string, setTotalState: Function, setTextFieldInputLoadoutName: Function) {
    return (
        <Button sx={{ backgroundColor: ColorConfigurations.primary, color: "black", borderRadius: 2 }} onClick={(_) => {
            let savedLoadoutDataString = localStorage.getItem(loadoutSaveKey);
            let savedLoadoutData = defaultSingleEquipmentInput();
            if (savedLoadoutDataString !== null) {
                savedLoadoutData = JSON.parse(savedLoadoutDataString);
            }

            setTotalState(savedLoadoutData);
            setTextFieldInputLoadoutName("Load Complete");
        }}>
            <Typography sx={{ fontWeight: "bold", fontSize: 12 }}>Load</Typography>
        </Button >
    )
}