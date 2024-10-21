import { Box, Button, Dialog, DialogActions, DialogContent, DialogContentText, styled, TextField, Typography } from "@mui/material";
import { AppConfigurations } from "../../Themes";
import { jobAbbrevToJobIcon } from "../icon/jobicon/JobIconFactory";
import { useEffect, useState } from "react";
import { EquipmentInput } from "../../types/EquipmentInput";
import {
  defaultBestPartnerEquipmentInput,
  defaultSingleEquipmentInput,
} from "../../const/DefaultSingleEquipmentInput";
import { BEST_PARTNER_URL } from "../../App";
import { PLD_EN_NAME, SCH_EN_NAME, TextDictionary } from "../../const/languageTexts";

export const inputStyle = {
  width: "60%",
  "& .MuiInputBase-input": {
    color: "black",
    backgroundColor: "white",
    textAlign: "left",
    height: "1.5vh",
  },
  "& .MuiInputLabel-root": {
    color: AppConfigurations.primary,
  },
  "& .MuiOutlinedInput-root": {
    "& fieldset": {},
    "&:hover fieldset": {},
    notched: false,
  },
  label: {
    transform: "translate(0, -2.3vh)",
  },
};

export const LoadoutInput = styled(TextField)(({ }) => inputStyle);

interface LoadoutMetaData {
  loadoutName: string;
  jobAbbrev: string;
}

export function DefaultLoadoutMetadata(defaultLoadoutText: string): LoadoutMetaData {
  return {
    loadoutName: defaultLoadoutText,
    jobAbbrev: PLD_EN_NAME,
  };
}

export function DefaultBestPartnerLoadoutMetadata(defaultLoadoutText: string): LoadoutMetaData {
  return {
    loadoutName: defaultLoadoutText,
    jobAbbrev: SCH_EN_NAME,
  };
}

const BLUR_COLOR = "gray";
const FOCUS_COLOR = "black";

export function LoadoutBox(
  loadoutId: number,
  simulationName: string,
  totalState: EquipmentInput,
  setTotalState: Function,
  numberOfEquipmentSets: number,
  LANGUAGE_TEXTS: TextDictionary
) {
  let [textFieldInputLoadoutName, setTextFieldInputLoadoutName] = useState(LANGUAGE_TEXTS.LOADOUT_NAME_TEXT);
  let [textColor, setColor] = useState(BLUR_COLOR);
  let loadoutSaveKey = `${simulationName}-${loadoutId}`;
  let loadoutMetadataSaveKey = `${simulationName}-loadoutMetadata-${loadoutId}`;
  let [blur, setBlur] = useState(true);

  let savedLoadoutMetadataString = localStorage.getItem(loadoutMetadataSaveKey);
  let savedLoadoutMetadata =
    simulationName !== BEST_PARTNER_URL
      ? DefaultLoadoutMetadata(LANGUAGE_TEXTS.DEFAULT_LOADOUT_TEXT)
      : DefaultBestPartnerLoadoutMetadata(LANGUAGE_TEXTS.DEFAULT_LOADOUT_TEXT);

  if (savedLoadoutMetadataString !== null) {
    savedLoadoutMetadata = JSON.parse(savedLoadoutMetadataString);
  }
  let [loadoutMetadata, setLoadoutMetadata] = useState(savedLoadoutMetadata);

  const handleBlur = () => {
    setColor(BLUR_COLOR);
  };

  const handleFocus = () => {
    setTextFieldInputLoadoutName("");
    setColor(FOCUS_COLOR);
    setBlur(false);
  }


  return (
    <Box
      sx={{
        backgroundColor: AppConfigurations.backgroundFour,
        borderRadius: 4,
        paddingX: 2,
      }}
    >
      <Box
        sx={{
          display: "flex",
          alignItems: "center",
          padding: 2,
          width: "100%",
        }}
      >
        <Typography
          variant="h2"
          color={AppConfigurations.primary}
          align="left"
          fontSize={AppConfigurations.body2FontSize}
        >
          {loadoutId}. {loadoutMetadata.loadoutName}
        </Typography>
        <Box marginX={1}>
          <img
            src={jobAbbrevToJobIcon(loadoutMetadata.jobAbbrev)}
            alt={loadoutMetadata.jobAbbrev}
            width={25}
            height={25}
            style={{ verticalAlign: "middle" }}
          />
        </Box>
      </Box>
      <Box>
        <LoadoutInput
          InputProps={{
            sx: {
              "& .MuiInputBase-input": {
                color: textColor, // 입력 필드 텍스트 색상 지정
              },
            },
          }}
          value={blur ? LANGUAGE_TEXTS.LOADOUT_NAME_TEXT : textFieldInputLoadoutName}
          onChange={(e) => {
            if (!blur) {
              setTextFieldInputLoadoutName(e.target.value);
            }
          }}
          fullWidth
          onBlur={handleBlur}
          onFocus={handleFocus}
          sx={{ backgroundColor: "white", width: "100%" }}
        />

        <Box padding={1} display="flex" justifyContent="space-evenly">
          {LoadoutOverwriteButton(
            loadoutSaveKey,
            loadoutMetadataSaveKey,
            textFieldInputLoadoutName,
            totalState,
            setLoadoutMetadata,
            setBlur,
            LANGUAGE_TEXTS
          )}
          {LoadoutLoadButton(
            loadoutSaveKey,
            simulationName,
            setTotalState,
            numberOfEquipmentSets,
            setBlur,
            LANGUAGE_TEXTS
          )}
        </Box>
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
  setBlur: Function,
  LANGUAGE_TEXTS: TextDictionary
) {
  const [dialogOpen, setDialogOpen] = useState(false);
  const [buttonClickConfirmed, setButtonClickConfirmed] = useState(false);

  const handleDialogClose = () => {
    setDialogOpen(false); // 다이얼로그 닫기
  };

  const handleConfirm = () => {
    setDialogOpen(false);
    setButtonClickConfirmed(true); // 다이얼로그 확인 클릭 시 버튼 클릭 확정
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
    setBlur(true);
  };


  useEffect(() => {
    if (buttonClickConfirmed) {
      // 여기서 실제 버튼의 onClick 이벤트 처리
      setButtonClickConfirmed(false); // 상태 초기화
    }
  }, [buttonClickConfirmed]);

  return (
    <>
      <Button
        sx={{
          backgroundColor: AppConfigurations.primary,
          color: "black",
          borderRadius: 2,
        }}
        onClick={() => {
          setDialogOpen(true);
        }}
      >
        <Typography
          sx={{ fontWeight: "bold", fontSize: AppConfigurations.body2FontSize }}
        >
          {LANGUAGE_TEXTS.LOADOUT_WRITE_TEXT}
        </Typography>
      </Button>
      <Dialog
        open={dialogOpen}
        onClose={handleDialogClose}
        aria-labelledby="confirm-dialog-title"
        aria-describedby="confirm-dialog-description"
      >
        <DialogContent>
          <DialogContentText id="confirm-dialog-description">
            {LANGUAGE_TEXTS.OVERWRITE_CONFIRM_TEXT}
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleConfirm} color="primary" autoFocus>
            {LANGUAGE_TEXTS.CONFIRM_TEXT}
          </Button>
          <Button onClick={handleDialogClose} color="primary">
            {LANGUAGE_TEXTS.CANCEL_TEXT}
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
}

function LoadoutLoadButton(
  loadoutSaveKey: string,
  simulationName: string,
  setTotalState: Function,
  numberOfEquipmentSets: number,
  setBlur: Function,
  LANGUAGE_TEXTS: TextDictionary
) {
  const [dialogOpen, setDialogOpen] = useState(false);
  const [buttonClickConfirmed, setButtonClickConfirmed] = useState(false);

  const handleDialogClose = () => {
    setDialogOpen(false); // 다이얼로그 닫기
  };

  const handleConfirm = () => {
    setDialogOpen(false);
    setButtonClickConfirmed(true); // 다이얼로그 확인 클릭 시 버튼 클릭 확정
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
    setBlur(true);
  };


  useEffect(() => {
    if (buttonClickConfirmed) {
      setButtonClickConfirmed(false); // 상태 초기화
    }
  }, [buttonClickConfirmed]);

  return (
    <>
      <Button
        sx={{
          backgroundColor: AppConfigurations.primary,
          color: "black",
          borderRadius: 2,
        }}
        onClick={() => {
          setDialogOpen(true);
        }}
      >
        <Typography
          sx={{ fontWeight: "bold", fontSize: AppConfigurations.body2FontSize }}
        >
          {LANGUAGE_TEXTS.LOADOUT_LOAD_TEXT}
        </Typography>
      </Button>
      <Dialog
        open={dialogOpen}
        onClose={handleDialogClose}
        aria-labelledby="confirm-dialog-title"
        aria-describedby="confirm-dialog-description"
      >
        <DialogContent>
          <DialogContentText id="confirm-dialog-description">
            {LANGUAGE_TEXTS.LOADOUT_LOAD_TEXT}
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleConfirm} color="primary" autoFocus>
            {LANGUAGE_TEXTS.CONFIRM_TEXT}
          </Button>
          <Button onClick={handleDialogClose} color="primary">
            {LANGUAGE_TEXTS.CANCEL_TEXT}
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
}
