import { styled, Button } from "@mui/material";
import { useNavigate } from "react-router-dom";
import {
  mapJobAbbrevToJobDefaultStat,
  playerStatToPlayerPower,
} from "../../const/StatValue";
import { PartyInfo } from "../../types/PartyStates";
import {
  SINGLE_INPUT_SAVE_NAME,
  STAT_WEIGHTS_RESPONSE_SAVE_NAME,
  STAT_WEIGHTS_URL,
} from "../../App";
import { useState } from "react";
import { requestButtonStyle } from "./Style";
import {
  EquipmentInput,
  SingleEquipmentInputSaveState,
} from "../../types/EquipmentInput";
import { AUTO_ATTACK_DELAYS } from "../../types/ffxivdatabase/Job";
import { getStatNames } from "../../types/ffxivdatabase/Stats";
import {
  StatWeightsResponse,
  StatWeightsResponseTable,
} from "../../types/StatWeightsResponse";
import { calculatePowerByStat } from "../../types/ffxivdatabase/ItemSet";
import { sendRequestAsync } from "./QuickSimRequestButton";

const REQUEST_URL = "http://localhost:13406/api/v1/statweights";
const WEAPON_DAMAGE_INCREASE = 10;
const MAIN_STAT_INCREASE = 100;
const SUB_STAT_INCREASE = 500;

export function StatWeightsRequestButton(totalState: EquipmentInput) {
  let RequestButton = styled(Button)`
    ${requestButtonStyle}
  `;

  let stats = [""].concat(
    getStatNames(totalState.equipmentDatas[0].mainPlayerJobAbbrev)
  );
  let totalRequestCount = stats.length;

  let [buttonText, setButtonText] = useState("Simulate");
  let [requestCount, setRequestCount] = useState(0);
  const loadingButtonText = (requestCount: number) => {
    return `Simulating... ${requestCount}/${totalRequestCount}`;
  };

  let navigate = useNavigate();
  let count = 0;

  const handleClick = async () => {
    setButtonText(loadingButtonText(requestCount));
    let inputJson = JSON.stringify(totalState);
    localStorage.setItem(SINGLE_INPUT_SAVE_NAME, inputJson);

    let statWeightsResponseTable: StatWeightsResponseTable = {
      statAugmentedSimulationData: [],
    };

    let responsePromises = [];
    let responses: Array<Response> = [];

    let requests = stats.map((stat) => {
      return JSON.stringify(
        createAugmentedRequest(totalState.equipmentDatas[0], stat)
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
    const formattedResponses: Array<Promise<StatWeightsResponse>> =
      responses.map(async (response) => {
        const data = await response.json();
        return data;
      });

    const finalResponses = await Promise.all(formattedResponses);
    statWeightsResponseTable.statAugmentedSimulationData = finalResponses;

    let responseString = JSON.stringify(statWeightsResponseTable);

    localStorage.setItem(STAT_WEIGHTS_RESPONSE_SAVE_NAME, responseString);

    navigate(`/${STAT_WEIGHTS_URL}`);
  };
  return (
    <RequestButton variant="contained" onClick={handleClick}>
      {buttonText}
    </RequestButton>
  );
}

function createAugmentedRequest(
  totalState: SingleEquipmentInputSaveState,
  augmentStatName: string
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

  if (augmentStatName !== "") {
    if (augmentStatName === "WD") {
      power.weaponDamage += WEAPON_DAMAGE_INCREASE;
    }
    if (
      augmentStatName === "STR" ||
      augmentStatName === "DEX" ||
      augmentStatName === "INT" ||
      augmentStatName === "MND"
    ) {
      power.mainStat += MAIN_STAT_INCREASE;
    }
    if (augmentStatName === "CRT") {
      power.criticalStrike += SUB_STAT_INCREASE;
    }
    if (augmentStatName === "DH") {
      power.directHit += SUB_STAT_INCREASE;
    }
    if (augmentStatName === "DET") {
      power.determination += SUB_STAT_INCREASE;
    }
    if (augmentStatName === "SKS") {
      power.skillSpeed += SUB_STAT_INCREASE;
    }
    if (augmentStatName === "SPS") {
      power.spellSpeed += SUB_STAT_INCREASE;
    }
    if (augmentStatName === "TEN") {
      power.tenacity += SUB_STAT_INCREASE;
    }

    calculatePowerByStat(power, jobAbbrev);
  }

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
  };
}
