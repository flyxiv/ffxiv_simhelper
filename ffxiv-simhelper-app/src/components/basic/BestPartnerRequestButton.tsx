import { styled, Button, Box } from "@mui/material";
import { useNavigate } from "react-router-dom";
import {
  calculateIlvlAdjustment,
  mapJobAbbrevToJobBisEquipments,
} from "../../const/StatValue";
import { PartyInfo } from "../../types/PartyStates";
import {
  BEST_PARTNER_INPUT_SAVE_NAME,
  BEST_PARTNER_RESPONSE_SAVE_NAME,
  BEST_PARTNER_RESULT_URL,
} from "../../App";
import { useState } from "react";
import { requestButtonStyle } from "./Style";
import {
  EquipmentInput,
  SingleEquipmentInputSaveState,
  USE_POT_VAL,
} from "../../types/EquipmentInput";
import {
  ALL_PLAYER_JOBS,
  AUTO_ATTACK_DELAYS,
} from "../../types/ffxivdatabase/Job";
import { sendRequestAsync } from "./DpsAnalysisRequestButton";
import { jobAbbrevToRole, JobRole } from "../../page/BestPartner";
import {
  BestPartnerResponse,
  BestPartnerResponseTable,
} from "../../types/BestPartnerResponse";
import { StopButton } from "./StopButton";
import { AppConfigurations } from "../../Themes";
import { defaultPlayerPower } from "../../types/ffxivdatabase/PlayerPower";
import { calculatePlayerPowerFromInputs } from "../../types/ffxivdatabase/ItemSet";
import {
  BRD_EN_NAME,
  DNC_EN_NAME,
  MIDLANDER_HYUR_EN_NAME,
  MNK_EN_NAME,
} from "../../const/languageTexts";

const REQUEST_URL = `${AppConfigurations.requestServer}/api/v1/bestpartner`;
export const BEST_PARTNER_ITERATION_COUNT = AppConfigurations.isApp ? 2000 : 1;

// classes like MNK, DNC, BRD eat more buffs when more players are in the party. We can't simulate full party because it's too slow, so add a little boost to these classes.
const PARTY_MEMBER_RELATED_CLASS_MULTIPLIER = 11;

interface PartnerKey {
  jobAbbrev: string;
  role: JobRole;
}

export function BestPartnerRequestButton(totalState: EquipmentInput) {
  let [isRunning, setIsRunning] = useState(false);
  let RequestButton = styled(Button)`
    ${requestButtonStyle}
  `;

  let allPossiblePartners = createAllPossiblePartnerList(
    totalState.equipmentDatas[0].mainPlayerJobAbbrev
  );
  let totalRequestCount = allPossiblePartners.length;

  let [buttonText, setButtonText] = useState("Simulate");
  let [requestCount, setRequestCount] = useState(0);

  const loadingButtonText = (requestCount: number) => {
    return `Simulating... ${requestCount}/${totalRequestCount}`;
  };

  let navigate = useNavigate();
  let count = 0;

  const handleClick = async () => {
    setIsRunning(true);
    setButtonText(loadingButtonText(requestCount));
    let inputJson = JSON.stringify(totalState);
    localStorage.setItem(BEST_PARTNER_INPUT_SAVE_NAME, inputJson);

    let bestPartnerResponseTable: BestPartnerResponseTable = {
      combatTimeMillisecond: totalState.equipmentDatas[0].combatTimeMillisecond,
      mainPlayerPower: totalState.equipmentDatas[0].power,
      mainPlayerJobAbbrev: totalState.equipmentDatas[0].mainPlayerJobAbbrev,
      partnerSimulationData: [],
    };

    let responsePromises = [];
    let responses: Array<Response> = [];

    let requests = allPossiblePartners.map((partnerKey) => {
      return JSON.stringify(
        createBestPartnerRequest(
          totalState.equipmentDatas[0],
          partnerKey.jobAbbrev
        )
      );
    });

    const incrementState = (count: number) => {
      setRequestCount(count);
      setButtonText(loadingButtonText(count));
    };

    for (let i = 0; i < requests.length; i++) {
      responsePromises.push(
        sendRequestAsync(requests[i], REQUEST_URL)
          .then((response) => {
            responses.push(response);
            count = count + 1;
            incrementState(count);
          })
          .catch((error) => {
            console.error("Error: ", error.message);
          })
      );
    }

    await Promise.all(responsePromises);
    const formattedResponses: Array<Promise<BestPartnerResponse>> =
      responses.map(async (response) => {
        const data = await response.json();
        return data;
      });

    const finalResponses = await Promise.all(formattedResponses);
    bestPartnerResponseTable.partnerSimulationData = finalResponses;

    let responseString = JSON.stringify(bestPartnerResponseTable);

    localStorage.setItem(BEST_PARTNER_RESPONSE_SAVE_NAME, responseString);

    navigate(`/${BEST_PARTNER_RESULT_URL}`);
  };
  return (
    <Box display="flex" alignItems="center">
      <RequestButton
        variant="contained"
        onClick={handleClick}
        disabled={isRunning}
        sx={{
          "&:disabled": {
            backgroundColor: AppConfigurations.primary,
            color: "black",
          },
        }}
      >
        {buttonText}
      </RequestButton>
      {isRunning ? StopButton() : <Box />}
    </Box>
  );
}

function createBestPartnerRequest(
  totalState: SingleEquipmentInputSaveState,
  partnerJobAbbrev: string
) {
  let jobAbbrev = totalState.mainPlayerJobAbbrev;
  let partner1Id = totalState.mainPlayerPartner1Id;
  let partner2Id = totalState.mainPlayerPartner2Id;

  let autoAttackDelays = AUTO_ATTACK_DELAYS.get(totalState.mainPlayerJobAbbrev);
  if (autoAttackDelays === undefined) {
    autoAttackDelays = 0;
  }
  let power = totalState.power;
  power.autoAttackDelays = autoAttackDelays;

  let partyInfo: PartyInfo[] = [
    {
      playerId: 0,
      jobAbbrev: jobAbbrev,
      partner1Id: partner1Id,
      partner2Id: partner2Id,
      power: power,
    },
  ];

  let partnerBisEquipments = mapJobAbbrevToJobBisEquipments(partnerJobAbbrev);

  if (partnerBisEquipments !== undefined) {
    let partnerTotalState = {
      mainPlayerJobAbbrev: partnerJobAbbrev,
      race: MIDLANDER_HYUR_EN_NAME,
      foodId: partnerBisEquipments.foodId,
      mainPlayerPartner1Id: null,
      mainPlayerPartner2Id: null,
      itemSet: partnerBisEquipments.itemSet,
      gearSetMaterias: partnerBisEquipments.gearSetMaterias,
      combatTimeMillisecond: 0,
      partyMemberJobAbbrevs: [],
      partyMemberIds: [],
      partyMemberIlvl: 0,
      usePot: 1,
      power: defaultPlayerPower(),
      compositionBuffPercent: 0,
    };
    let partnerPower = calculatePlayerPowerFromInputs(partnerTotalState);

    if (
      partnerJobAbbrev === BRD_EN_NAME ||
      partnerJobAbbrev === DNC_EN_NAME ||
      partnerJobAbbrev === MNK_EN_NAME
    ) {
      partnerPower.mainStatMultiplier =
        Math.floor(
          ((partnerPower.mainStatMultiplier *
            PARTY_MEMBER_RELATED_CLASS_MULTIPLIER) /
            10) *
          1000
        ) / 1000;
    }

    let partnerAutoAttackDelays = AUTO_ATTACK_DELAYS.get(jobAbbrev);
    if (partnerAutoAttackDelays === undefined) {
      partnerAutoAttackDelays = 0;
    }
    partnerPower.autoAttackDelays = partnerAutoAttackDelays;

    partyInfo.push({
      playerId: 1,
      partner1Id: null,
      partner2Id: null,
      jobAbbrev: partnerJobAbbrev,
      power: partnerPower,
    });
  }

  const request = {
    mainPlayerId: 0,
    combatTimeMillisecond: totalState.combatTimeMillisecond,
    party: partyInfo,
    partyIlvlAdjustment: calculateIlvlAdjustment(totalState.partyMemberIlvl),
    usePot: totalState.usePot === USE_POT_VAL,
  };

  return request;
}

function createAllPossiblePartnerList(mainPlayerJobAbbrev: string) {
  let allPartnerList = [];

  for (let jobAbbrev of ALL_PLAYER_JOBS) {
    if (jobAbbrev === mainPlayerJobAbbrev) {
      continue;
    }

    allPartnerList.push(convertToPartnerKey(jobAbbrev));
  }

  return allPartnerList;
}

function convertToPartnerKey(jobAbbrev: string): PartnerKey {
  return { jobAbbrev: jobAbbrev, role: jobAbbrevToRole(jobAbbrev) };
}
