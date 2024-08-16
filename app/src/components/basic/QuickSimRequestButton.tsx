import { styled, Button } from "@mui/material";
import { useNavigate } from "react-router-dom";
import { EquipmentSimCharacterStates } from "src/types/CharacterStates";
import {
  mapJobAbbrevToJobDefaultStat,
  playerStatToPlayerPower,
} from "src/const/StatValue";
import { PartyInfo } from "src/types/PartyStates";
import { QuickSimInputSaveName, QuickSimResponseSaveName } from "src/App";
import { useState } from "react";
import { QuickSimResponse } from "src/types/QuickSimResponse";
import { requestButtonStyle } from "./Style";
import {
  CharacterEquipmentsData,
  PlayerPower,
} from "src/types/ffxivdatabase/PlayerPower";
import { QuickSimInputSaveState } from "src/types/QuickSimInput";
import { AUTO_ATTACK_DELAYS } from "src/types/ffxivdatabase/Job";

const totalRequestCount = 1;

export function QuickSimRequestButton(
  partyMemberJobAbbrevs: string[],
  combatTimeSeconds: number,
  characterState: EquipmentSimCharacterStates,
  data: CharacterEquipmentsData
) {
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
    setButtonText(loadingButtonText(requestCount));
    let input = createQuickSimInputSaveState(
      partyMemberJobAbbrevs,
      characterState,
      combatTimeSeconds,
      data
    );
    let inputJson = JSON.stringify(input);
    localStorage.setItem(QuickSimInputSaveName, inputJson);

    let request = createQuickSimRequest(
      partyMemberJobAbbrevs,
      combatTimeSeconds,
      characterState,
      data.power
    );

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
        sendRequestAsync(body)
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

    let totalDps = 0;
    let maxRdps = 0;
    let totalRdps = 0;
    let totalEdps = 0;

    damageSummaries.forEach((summary) => {
      totalDps += summary.pdps;
      totalRdps += summary.rdps;
      totalEdps += summary.edps;
      maxRdps = Math.max(maxRdps, summary.rdps);
    });

    let averageDps = totalDps / totalRequestCount;
    let averageRdps = totalRdps / totalRequestCount;
    let averageEdps = totalEdps / totalRequestCount;

    response.simulationData[mainPlayerId].simulationSummary.pdps = averageDps;
    response.simulationData[mainPlayerId].simulationSummary.rdps = averageRdps;
    response.simulationData[mainPlayerId].simulationSummary.edps = averageEdps;
    response.simulationData[mainPlayerId].simulationSummary.maxRdps = maxRdps;

    const responseString = JSON.stringify(response);
    localStorage.setItem(QuickSimResponseSaveName, responseString);

    navigate("/simulationresult");
  };
  return (
    <RequestButton variant="contained" onClick={handleClick}>
      {buttonText}
    </RequestButton>
  );
}

function createQuickSimInputSaveState(
  partyMemberJobAbbrevs: string[],
  mainCharacterState: EquipmentSimCharacterStates,
  combatTimeSeconds: number,
  data: CharacterEquipmentsData
): QuickSimInputSaveState {
  let partyMemberIds: number[] = [];
  partyMemberJobAbbrevs.forEach((jobAbbrev, index) => {
    if (jobAbbrev !== "Empty") {
      partyMemberIds.push(index + 1);
    }
  });

  return {
    partyMemberJobAbbrevs: partyMemberJobAbbrevs,
    itemSet: data.itemSet,
    gearSetMaterias: data.gearSetMaterias,
    mainPlayerJob: mainCharacterState.jobAbbrev,
    race: data.race,
    mainPlayerPartner1Id: mainCharacterState.partner1Id,
    mainPlayerPartner2Id: mainCharacterState.partner2Id,
    combatTimeMillisecond: combatTimeSeconds * 1000,
    partyMemberIds: partyMemberIds,
    foodId: data.foodId,
  };
}

function createQuickSimRequest(
  partyState: string[],
  combatTimeSeconds: number,
  characterState: EquipmentSimCharacterStates,
  power: PlayerPower
) {
  let autoAttackDelays = AUTO_ATTACK_DELAYS.get(characterState.jobAbbrev);
  if (autoAttackDelays === undefined) {
    autoAttackDelays = 0;
  }
  power.autoAttackDelays = autoAttackDelays;

  let partyInfo: PartyInfo[] = [
    {
      playerId: 0,
      jobAbbrev: characterState.jobAbbrev,
      partner1Id: characterState.partner1Id,
      partner2Id: characterState.partner2Id,
      power: power,
    },
  ];

  let playerCount = 0;
  for (let i = 0; i < partyState.length; i++) {
    let jobAbbrev = partyState[i];
    let defaultStat = mapJobAbbrevToJobDefaultStat(jobAbbrev);

    if (defaultStat === undefined) {
      continue;
    }

    let power = playerStatToPlayerPower(defaultStat, jobAbbrev);
    let autoAttackDelays = AUTO_ATTACK_DELAYS.get(jobAbbrev);
    if (autoAttackDelays === undefined) {
      return (autoAttackDelays = 0);
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
    combatTimeMillisecond: combatTimeSeconds * 1000,
    party: partyInfo,
  };
}

function sendRequestAsync(requestBody: string): Promise<Response> {
  return new Promise(async (resolve, reject) => {
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => {
        controller.abort();
        reject(new Error("Request timeout"));
      }, 300000);

      const response = await fetch("http://localhost:13406/api/v1/simulate", {
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
