import { styled, Button, Box } from "@mui/material";
import { useNavigate } from "react-router-dom";
import {
  calculateIlvlAdjustment,
  mapJobAbbrevToJobDefaultStat,
  playerStatToPlayerPower,
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

const totalRequestCount = 1000;
const REQUEST_URL = "http://localhost:13406/api/v1/simulate";

export function QuickSimRequestButton(totalState: EquipmentInput) {
  let [isRunning, setIsRunning] = useState(false);
  let RequestButton = styled(Button)`
    ${requestButtonStyle}
  `;

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

    for (let i = 0; i < totalRequestCount; i++) {
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

    let damageSummary =
      aggregateDamageStatisticsFromSampleRuns(damageSummaries);

    response = finalResponses[0];
    response.simulationData[mainPlayerId].simulationSummary = damageSummary;

    const responseString = JSON.stringify(response);
    localStorage.setItem(QUICK_SIM_RESPONSE_SAVE_NAME, responseString);

    navigate(`/${QUICKSIM_RESULT_URL}`);
  };
  return (
    <Box display="flex" alignItems={"center"}>
      <RequestButton variant="contained" onClick={handleClick}>
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
    let defaultStat = mapJobAbbrevToJobDefaultStat(jobAbbrev);

    if (defaultStat === undefined) {
      continue;
    }

    let power = playerStatToPlayerPower(defaultStat, jobAbbrev);
    let autoAttackDelays = AUTO_ATTACK_DELAYS.get(jobAbbrev);
    if (autoAttackDelays === undefined) {
      autoAttackDelays = 0;
    }
    power.autoAttackDelays = autoAttackDelays;

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
