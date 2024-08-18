import { styled, Button } from "@mui/material";
import { useNavigate } from "react-router-dom";
import { CharacterStates } from "src/types/CharacterStates";
import { mapJobAbbrevToJobDefaultStat } from "src/const/StatValue";
import { PartyInfo } from "src/types/PartyStates";
import { useState } from "react";
import {
  StatCompareRequestSaveName,
  StatCompareResponseSaveName,
} from "src/App";
import { StatCompareRequest } from "src/types/StatCompareRequest";
import { StatCompareResponse } from "src/types/StatCompareResponse";
import { SimulationSummary } from "src/types/CombatSimulationResult";
import { requestButtonStyle } from "./Style";

const totalRequestCount = 24;

export function StatCompareRequestButton(
  partyState: string[],
  combatTimeSeconds: number,
  characterState1: CharacterStates,
  characterState2: CharacterStates
) {
  return <></>;
}
/*
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
    let request = createStatCompareRequest(
      partyState,
      combatTimeSeconds,
      characterState1,
      characterState2
    );

    console.log(request);

    if (request instanceof Error) {
      console.error("Error: ", request.message);
      return;
    }

    let body = JSON.stringify(request);
    localStorage.setItem(StatCompareRequestSaveName, body);
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
    const formattedResponses: Array<Promise<StatCompareResponse>> =
      responses.map(async (response) => {
        const data = await response.json();
        return data;
      });

    const finalResponses = await Promise.all(formattedResponses);
    // Use mean/max for the summary and the very first request for the other results.
    let response = finalResponses[0];
    let damageSummaries1 = finalResponses.map(
      (response) => response.simulationGear1
    );
    let damageSummaries2 = finalResponses.map(
      (response) => response.simulationGear2
    );

    let damageSummary1 =
      aggregateDamageStatisticsFromSampleRuns(damageSummaries1);
    let damageSummary2 =
      aggregateDamageStatisticsFromSampleRuns(damageSummaries2);

    response.simulationGear1.pdps = damageSummary1.pdps;
    response.simulationGear1.rdps = damageSummary1.rdps;
    response.simulationGear1.edps = damageSummary1.edps;
    response.simulationGear1.maxRdps = damageSummary1.maxRdps;

    response.simulationGear2.pdps = damageSummary2.pdps;
    response.simulationGear2.rdps = damageSummary2.rdps;
    response.simulationGear2.edps = damageSummary2.edps;
    response.simulationGear2.maxRdps = damageSummary2.maxRdps;

    const responseString = JSON.stringify(response);
    localStorage.setItem(StatCompareResponseSaveName, responseString);

    navigate("/statcompareresult");
  };
  return (
    <RequestButton variant="contained" onClick={handleClick}>
      {buttonText}
    </RequestButton>
  );
}

function createStatCompareRequest(
  partyState: string[],
  combatTimeSeconds: number,
  characterState1: CharacterStates,
  characterState2: CharacterStates
): StatCompareRequest {
  let partyInfo: PartyInfo[] = [];

  let playerCount = 0;

  for (let i = 0; i < partyState.length; i++) {
    let defaultStat = mapJobAbbrevToJobDefaultStat(partyState[i]);

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
    mainPlayerJob: characterState1.jobAbbrev,
    mainPlayerPartner1Id: characterState1.partner1Id,
    mainPlayerPartner2Id: characterState1.partner2Id,
    combatTimeMillisecond: combatTimeSeconds * 1000,
    mainPlayerStat1: characterState1.stats,
    mainPlayerStat2: characterState2.stats,
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
      }, 60000);

      const response = await fetch(
        "http://localhost:13406/api/v1/statcompare",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: requestBody,
          signal: controller.signal,
        }
      );

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

function aggregateDamageStatisticsFromSampleRuns(
  damageSummaries: SimulationSummary[]
) {
  let totalDps: Array<number> = [];
  let maxRdps = 0;
  let totalRdps: Array<number> = [];
  let totalEdps: Array<number> = [];

  damageSummaries.forEach((summary) => {
    totalDps.push(summary.pdps);
    totalRdps.push(summary.rdps);
    totalEdps.push(summary.edps);
    maxRdps = Math.max(maxRdps, summary.rdps);
  });

  let medianIndex = Math.floor(totalRequestCount / 2);
  totalDps.sort((a, b) => a - b);
  totalRdps.sort((a, b) => a - b);
  totalEdps.sort((a, b) => a - b);

  let medianDps = totalDps[medianIndex];
  let medianRdps = totalRdps[medianIndex];
  let medianEdps = totalEdps[medianIndex];

  return {
    pdps: medianDps,
    rdps: medianRdps,
    edps: medianEdps,
    maxRdps: maxRdps,
  };
} */
