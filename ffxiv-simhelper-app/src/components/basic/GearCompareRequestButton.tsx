import { styled, Button, Box } from "@mui/material";
import { useNavigate } from "react-router-dom";
import {
  GEAR_COMPARE_RESULT_URL,
  GEAR_COMPARE_RESPONSE_SAVE_NAME,
  GEAR_COMPARE_REQUEST_SAVE_NAME,
} from "../../App";
import { useState } from "react";
import { requestButtonStyle } from "./Style";
import {
  EquipmentInput,
  SingleEquipmentInputSaveState,
} from "../../types/EquipmentInput";
import {
  createQuickSimRequest,
  sendRequestAsync,
} from "./QuickSimRequestButton";
import { GearCompareResponse } from "../..//types/GearCompareResponse";
import { SimulationSummary } from "../../types/CombatSimulationResult";
import { StopButton } from "./StopButton";
import { AppConfigurations } from "../../Themes";

const TOTAL_REQUEST_COUNT = 500;
const TOTAL_ITERATION_COUNT = 8;
const HIGH_PERCENTILE = 0.95;

const REQUEST_URL = "http://localhost:13406/api/v1/gearcompare";

export function GearCompareRequestButton(totalState: EquipmentInput) {
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
    localStorage.setItem(GEAR_COMPARE_REQUEST_SAVE_NAME, inputJson);

    let request = createGearCompareRequest(
      totalState.equipmentDatas[0],
      totalState.equipmentDatas[1]
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
    const formattedResponses: Array<Promise<GearCompareResponse>> =
      responses.map(async (response) => {
        const data = await response.json();
        return data;
      });

    const finalResponses = await Promise.all(formattedResponses);

    // Use mean/max for the summary and the very first request for the other results.
    let response = finalResponses[0];
    let mainPlayerId = response.simulationGear1.mainPlayerId;

    let damageSummaries1 = finalResponses.map(
      (response) =>
        response.simulationGear1.simulationData[mainPlayerId].simulationSummary
    );
    let damageSummaries2 = finalResponses.map(
      (response) =>
        response.simulationGear2.simulationData[mainPlayerId].simulationSummary
    );

    let aggregatedDamageSummary1 = aggregateDamageStatisticsFromSampleRuns(
      damageSummaries1,
      TOTAL_REQUEST_COUNT * TOTAL_ITERATION_COUNT,
      HIGH_PERCENTILE
    );
    let aggregatedDamageSummary2 = aggregateDamageStatisticsFromSampleRuns(
      damageSummaries2,
      TOTAL_REQUEST_COUNT * TOTAL_ITERATION_COUNT,
      HIGH_PERCENTILE
    );

    response.simulationGear1.simulationData[mainPlayerId].simulationSummary =
      aggregatedDamageSummary1;
    response.simulationGear2.simulationData[mainPlayerId].simulationSummary =
      aggregatedDamageSummary2;

    const responseString = JSON.stringify(response);
    localStorage.setItem(GEAR_COMPARE_RESPONSE_SAVE_NAME, responseString);

    navigate(`/${GEAR_COMPARE_RESULT_URL}`);
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

function createGearCompareRequest(
  equipment1: SingleEquipmentInputSaveState,
  equipment2: SingleEquipmentInputSaveState
) {
  return {
    gear1Request: createQuickSimRequest(equipment1),
    gear2Request: createQuickSimRequest(equipment2),
  };
}

export function aggregateDamageStatisticsFromSampleRuns(
  damageSummaries: SimulationSummary[],
  totalRequestCount: number,
  highPercentile: number
) {
  let totalDps: Array<number> = [];
  let highRdps = 0;
  let totalRdps: Array<number> = [];
  let totalEdps: Array<number> = [];

  damageSummaries.forEach((summary) => {
    totalDps = totalDps.concat(summary.pdps);
    totalRdps = totalRdps.concat(summary.rdps);
    totalEdps = totalEdps.concat(summary.edps);
  });

  totalDps.sort((a, b) => a - b);
  totalRdps.sort((a, b) => a - b);
  totalEdps.sort((a, b) => a - b);

  let medianIndex = Math.floor(totalRequestCount / 2);
  let percentileIndex = Math.floor(totalRequestCount * highPercentile) - 1;

  let medianDps = totalDps[medianIndex];
  let medianRdps = totalRdps[medianIndex];
  let medianEdps = totalEdps[medianIndex];
  highRdps = totalRdps[percentileIndex >= 0 ? percentileIndex : 0];

  return {
    pdps: [medianDps],
    rdps: [medianRdps],
    edps: [medianEdps],
    maxRdps: [highRdps],
  };
}
