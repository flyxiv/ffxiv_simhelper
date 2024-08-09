import { styled, Button } from "@mui/material";
import { useNavigate } from "react-router-dom";
import { CharacterStates } from "src/types/CharacterStates";
import { MapJobAbbrevToJobDefaultStat } from "src/const/StatValue";
import { PartyInfo } from "src/types/PartyStates";
import { QuickSimRequestSaveName, QuickSimResponseSaveName } from "src/App";
import { useState } from "react";
import { QuickSimResponse } from "src/types/QuickSimResponse";
import { requestButtonStyle } from "./Style";

const totalRequestCount = 24;

export function QuickSimRequestButton(
  partyState: string[],
  combatTimeSeconds: number,
  characterState: CharacterStates
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
    let request = createQuickSimRequest(
      partyState,
      combatTimeSeconds,
      characterState
    );

    if (request instanceof Error) {
      console.error("Error: ", request.message);
      return;
    }

    let body = JSON.stringify(request);
    localStorage.setItem(QuickSimRequestSaveName, body);
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

function createQuickSimRequest(
  partyState: string[],
  combatTimeSeconds: number,
  characterState: CharacterStates
) {
  let partyInfo: PartyInfo[] = [
    {
      playerId: 0,
      job: characterState.jobAbbrev,
      partner1Id: characterState.partner1Id,
      partner2Id: characterState.partner2Id,
      stats: {
        weaponDamage: characterState.stats.weaponDamage,
        mainStat: characterState.stats.mainStat,
        criticalStrike: characterState.stats.criticalStrike,
        directHit: characterState.stats.directHit,
        determination: characterState.stats.determination,
        speed: characterState.stats.speed,
        tenacity: characterState.stats.tenacity,
      },
    },
  ];

  let playerCount = 0;
  for (let i = 0; i < partyState.length; i++) {
    let defaultStat = MapJobAbbrevToJobDefaultStat(partyState[i]);

    if (defaultStat === undefined) {
      continue;
    }

    partyInfo.push({
      playerId: playerCount + 1,
      partner1Id: null,
      partner2Id: null,
      job: partyState[i],
      stats: defaultStat,
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
        console.error("POST request failed");
        reject(new Error("Request failed"));
      }
    } catch (error) {
      console.error("Error occurred: ", error);
      reject(error);
    }
  });
}
