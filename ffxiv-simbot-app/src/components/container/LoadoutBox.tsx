import { Box, Button, styled, TextField, Typography } from "@mui/material";
import { ColorConfigurations } from "../../Themes";
import { jobAbbrevToJobIconPath } from "../icon/jobicon/JobIconFactory";
import { useState } from "react";
import { EquipmentInput } from "../../types/EquipmentInput";
import {
  defaultBestPartnerEquipmentInput,
  defaultSingleEquipmentInput,
} from "../../const/DefaultSingleEquipmentInput";
import { BEST_PARTNER_URL } from "../../App";

export const inputStyle = {
  width: "60%",
  "& .MuiInputBase-input": {
    color: "black",
    backgroundColor: "white",
    textAlign: "left",
    height: "0.5vh",
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

interface LoadoutMetaData {
  loadoutName: string;
  jobAbbrev: string;
}

export function DefaultLoadoutMetadata(): LoadoutMetaData {
  return {
    loadoutName: "Default Loadout",
    jobAbbrev: "PLD",
  };
}

export function DefaultBestPartnerLoadoutMetadata(): LoadoutMetaData {
  return {
    loadoutName: "Default Loadout",
    jobAbbrev: "SCH",
  };
}

export function LoadoutBox(
  loadoutId: number,
  simulationName: string,
  totalState: EquipmentInput,
  setTotalState: Function,
  numberOfEquipmentSets: number
) {
  let [textFieldInputLoadoutName, setTextFieldInputLoadoutName] = useState("");
  let loadoutSaveKey = `${simulationName}-${loadoutId}`;
  let loadoutMetadataSaveKey = `${simulationName}-loadoutMetadata-${loadoutId}`;

  let savedLoadoutMetadataString = localStorage.getItem(loadoutMetadataSaveKey);
  let savedLoadoutMetadata =
    simulationName !== BEST_PARTNER_URL
      ? DefaultLoadoutMetadata()
      : DefaultBestPartnerLoadoutMetadata();

  if (savedLoadoutMetadataString !== null) {
    savedLoadoutMetadata = JSON.parse(savedLoadoutMetadataString);
  }
  let [loadoutMetadata, setLoadoutMetadata] = useState(savedLoadoutMetadata);

  return (
    <Box
      sx={{
        backgroundColor: ColorConfigurations.backgroundFour,
        borderRadius: 4,
        paddingX: 2,
      }}
    >
      <Box sx={{ display: "flex", alignItems: "center", padding: 2 }}>
        <Typography
          variant="h2"
          color={ColorConfigurations.primary}
          align="left"
          fontSize={15}
        >
          {loadoutId}. {loadoutMetadata.loadoutName}
        </Typography>
        <Box marginX={1}>
          <img
            src={jobAbbrevToJobIconPath(loadoutMetadata.jobAbbrev)}
            alt={loadoutMetadata.jobAbbrev}
            width={25}
            height={25}
            style={{ verticalAlign: "middle" }}
          />
        </Box>
      </Box>
      <Box padding={1} display="flex">
        <LoadoutInput
          label="Name"
          value={textFieldInputLoadoutName}
          onChange={(e) => {
            setTextFieldInputLoadoutName(e.target.value);
          }}
          fullWidth
          sx={{ backgroundColor: "white" }}
        />
        {LoadoutOverwriteButton(
          loadoutSaveKey,
          loadoutMetadataSaveKey,
          textFieldInputLoadoutName,
          totalState,
          setLoadoutMetadata,
          setTextFieldInputLoadoutName
        )}
        {LoadoutLoadButton(
          loadoutSaveKey,
          simulationName,
          setTotalState,
          setTextFieldInputLoadoutName,
          numberOfEquipmentSets
        )}
      </Box>
    </Box>
  );
}

function LoadoutOverwriteButton(
  loadoutSaveKey: string,
  loadoutMetadataSaveKey: string,
  textFieldInputLoadoutName: string,
  totalState: EquipmentInput,
  setLoadoutMetadata: Function,
  setTextFieldInputLoadoutName: Function
) {
  return (
    <Button
      sx={{
        marginX: 1,
        backgroundColor: ColorConfigurations.primary,
        color: "black",
        borderRadius: 2,
      }}
      onClick={(_) => {
        let newMetaData = {
          loadoutName: textFieldInputLoadoutName,
          jobAbbrev: totalState.equipmentDatas[0].mainPlayerJobAbbrev,
        };

        localStorage.setItem(loadoutSaveKey, JSON.stringify(totalState));
        localStorage.setItem(
          loadoutMetadataSaveKey,
          JSON.stringify(newMetaData)
        );
        setLoadoutMetadata(newMetaData);
        setTextFieldInputLoadoutName("");
      }}
    >
      <Typography sx={{ fontWeight: "bold", fontSize: 10 }}>Write</Typography>
    </Button>
  );
}

function LoadoutLoadButton(
  loadoutSaveKey: string,
  simulationName: string,
  setTotalState: Function,
  setTextFieldInputLoadoutName: Function,
  numberOfEquipmentSets: number
) {
  return (
    <Button
      sx={{
        backgroundColor: ColorConfigurations.primary,
        color: "black",
        borderRadius: 2,
      }}
      onClick={(_) => {
        let savedLoadoutDataString = localStorage.getItem(loadoutSaveKey);
        let savedLoadoutData =
          simulationName !== BEST_PARTNER_URL
            ? defaultSingleEquipmentInput()
            : defaultBestPartnerEquipmentInput();

        let defaultSet = savedLoadoutData.equipmentDatas[0];
        for (let i = 1; i < numberOfEquipmentSets; i++) {
          savedLoadoutData.equipmentDatas.push({ ...defaultSet });
        }

        if (savedLoadoutDataString !== null) {
          savedLoadoutData = JSON.parse(savedLoadoutDataString);
        }

        setTotalState(savedLoadoutData);
        setTextFieldInputLoadoutName("Load Complete");
      }}
    >
      <Typography sx={{ fontWeight: "bold", fontSize: 10 }}>Load</Typography>
    </Button>
  );
}
