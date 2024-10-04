import { styled, Button, Box } from "@mui/material";
import { useNavigate } from "react-router-dom";
import {
  calculateIlvlAdjustment,
  mapJobAbbrevToJobBisEquipments,
} from "../../const/StatValue";
import { PartyInfo } from "../../types/PartyStates";
import {
  QUICKSIM_RESULT_URL,
  SINGLE_INPUT_SAVE_NAME,
  QUICK_SIM_RESPONSE_SAVE_NAME,
} from "../../App";
import { useState } from "react";
import { QuickSimResponse } from "../../types/QuickSimResponse";
import { requestButtonStyle } from "./Style";
import {
  EquipmentInput,
  SingleEquipmentInputSaveState,
  USE_POT_VAL,
} from "../../types/EquipmentInput";
import { AUTO_ATTACK_DELAYS } from "../../types/ffxivdatabase/Job";
import { StopButton } from "./StopButton";
import { aggregateDamageStatisticsFromSampleRuns } from "./GearCompareRequestButton";
import { AppConfigurations } from "../../Themes";
import { defaultPlayerPower } from "../../types/ffxivdatabase/PlayerPower";
import { calculatePlayerPowerFromInputs } from "../../types/ffxivdatabase/ItemSet";
import { MIDLANDER_HYUR_NAME_EN } from "../../const/languageTexts";

const TOTAL_REQUEST_COUNT = 1000;
const TOTAL_ITERATION_COUNT = 2;
export const QUICK_SIM_ITERATION_COUNT =
  TOTAL_REQUEST_COUNT * TOTAL_ITERATION_COUNT;
const REQUEST_URL = "http://localhost:13406/api/v1/simulate";

export function QuickSimRequestButton(totalState: EquipmentInput) {
  let [isRunning, setIsRunning] = useState(false);
  let RequestButton = styled(Button)`
    ${requestButtonStyle}
  `;

  let [buttonText, setButtonText] = useState("Simulate");
  let [requestCount, setRequestCount] = useState(0);
  const loadingButtonText = (requestCount: number) => {
    return `Simulating... ${(
      (requestCount / TOTAL_REQUEST_COUNT) *
      100
    ).toFixed(0)}%`;
  };

  let navigate = useNavigate();
  let count = 0;

  const handleClick = async () => {
    setIsRunning(true);

    setButtonText(loadingButtonText(requestCount));
    let inputJson = JSON.stringify(totalState);
    localStorage.setItem(SINGLE_INPUT_SAVE_NAME, inputJson);

    let request = createQuickSimRequest(totalState.equipmentDatas[0]);

    if (request instanceof Error) {
      console.error("Error: ", request.message);
      return;
    }

    let body = JSON.stringify(request);

    let responsePromises = [];
    let responses: Array<Response> = [];
    const incrementState = (count: number) => {
      setRequestCount(count);
      setButtonText(loadingButtonText(count));
    };

    for (let i = 0; i < TOTAL_REQUEST_COUNT; i++) {
      responsePromises.push(
        sendRequestAsync(body, REQUEST_URL)
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
    const formattedResponses: Array<Promise<QuickSimResponse>> = responses.map(
      async (response) => {
        const data = await response.json();
        return data;
      }
    );

    const finalResponses = await Promise.all(formattedResponses);
    // Use mean/max for the summary and the very first request for the other results.
    let response = finalResponses[0];
    let mainPlayerId = response.mainPlayerId;
    let damageSummaries = finalResponses.map(
      (response) => response.simulationData[mainPlayerId].simulationSummary
    );

    let damageSummary = aggregateDamageStatisticsFromSampleRuns(
      damageSummaries,
      TOTAL_REQUEST_COUNT * TOTAL_ITERATION_COUNT,
      1.0
    );

    response = finalResponses[0];
    response.simulationData[mainPlayerId].simulationSummary = damageSummary;

    const responseString = JSON.stringify(response);
    localStorage.setItem(QUICK_SIM_RESPONSE_SAVE_NAME, responseString);

    navigate(`/${QUICKSIM_RESULT_URL}`);
  };
  return (
    <Box display="flex" alignItems={"center"}>
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

export function createQuickSimRequest(
  totalState: SingleEquipmentInputSaveState
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

  let playerCount = 0;
  for (let i = 0; i < totalState.partyMemberJobAbbrevs.length; i++) {
    let jobAbbrev = totalState.partyMemberJobAbbrevs[i];
    let bisEquipments = mapJobAbbrevToJobBisEquipments(jobAbbrev);

    if (bisEquipments === undefined) {
      continue;
    }

    let playerTotalState = {
      mainPlayerJobAbbrev: jobAbbrev,
      race: MIDLANDER_HYUR_NAME_EN,
      foodId: bisEquipments.foodId,
      mainPlayerPartner1Id: null,
      mainPlayerPartner2Id: null,
      itemSet: bisEquipments.itemSet,
      gearSetMaterias: bisEquipments.gearSetMaterias,
      combatTimeMillisecond: 0,
      partyMemberJobAbbrevs: totalState.partyMemberJobAbbrevs,
      partyMemberIds: totalState.partyMemberIds,
      partyMemberIlvl: totalState.partyMemberIlvl,
      usePot: totalState.usePot,
      power: defaultPlayerPower(),
      compositionBuffPercent: 0,
    };

    let bisPower = calculatePlayerPowerFromInputs(playerTotalState);

    let autoAttackDelays = AUTO_ATTACK_DELAYS.get(jobAbbrev);
    if (autoAttackDelays === undefined) {
      autoAttackDelays = 0;
    }
    bisPower.autoAttackDelays = autoAttackDelays;

    partyInfo.push({
      playerId: playerCount + 1,
      partner1Id: null,
      partner2Id: null,
      jobAbbrev: jobAbbrev,
      power: power,
    });

    playerCount++;
  }

  return {
    mainPlayerId: 0,
    combatTimeMillisecond: totalState.combatTimeMillisecond,
    party: partyInfo,
    partyIlvlAdjustment: calculateIlvlAdjustment(totalState.partyMemberIlvl),
    usePot: totalState.usePot === USE_POT_VAL,
  };
}

export function sendRequestAsync(
  requestBody: string,
  requestUrl: string
): Promise<Response> {
  return new Promise(async (resolve, reject) => {
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => {
        controller.abort();
        reject(new Error("Request timeout"));
      }, 300000);

      const response = await fetch(requestUrl, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: requestBody,
        signal: controller.signal,
      });

      clearTimeout(timeoutId);
      if (response.ok) {
        console.log("POST request successful");
        resolve(response);
      } else {
        // Read the response body for error details
        const errorText = await response.text(); // Or use response.json() if you expect JSON
        console.error("POST request failed", {
          status: response.status,
          body: errorText,
        });
        reject(
          new Error(
            `Request failed with status ${response.status}: ${errorText}`
          )
        );
      }
    } catch (error) {
      console.error("Error occurred: ", error);
    }
  });
}
