import { styled, Button, Box } from "@mui/material";
import { useNavigate } from "react-router-dom";
import {
  calculateIlvlAdjustment,
  mapJobAbbrevToJobBisEquipments,
} from "../../const/StatValue";
import { EMPTY_PARTY_MEMBER, PartyInfo } from "../../types/PartyStates";
import {
  SINGLE_INPUT_SAVE_NAME,
  STAT_WEIGHTS_RESPONSE_SAVE_NAME,
  BEST_STATS_RESULT_URL,
} from "../../App";
import { useState } from "react";
import { requestButtonStyle } from "./Style";
import {
  EquipmentInput,
  SingleEquipmentInputSaveState,
  USE_POT_VAL,
} from "../../types/EquipmentInput";
import { AUTO_ATTACK_DELAYS } from "../../types/ffxivdatabase/Job";
import { getStatWeightNames } from "../../types/ffxivdatabase/Stats";
import {
  BestStatsResponse,
  BestStatsResponseTable,
} from "../../types/BestStats";
import { calculatePlayerPowerFromInputs, calculatePowerByStat } from "../../types/ffxivdatabase/ItemSet";
import { sendRequestAsync } from "./DpsAnalysisRequestButton";
import { defaultPlayerPower, getStatNeededByStatNameLadderAmount } from "../../types/ffxivdatabase/PlayerPower";
import { StopButton } from "./StopButton";
import { AppConfigurations } from "../../Themes";
import { CRIT_STAT_EN_NAME, DET_STAT_EN_NAME, DEX_STAT_EN_NAME, DH_STAT_EN_NAME, INT_STAT_EN_NAME, MIDLANDER_HYUR_EN_NAME, MIND_STAT_EN_NAME, SKS_STAT_EN_NAME, SMN_EN_NAME, SPS_STAT_EN_NAME, STR_STAT_EN_NAME, TEN_STAT_EN_NAME, WD_STAT_EN_NAME } from "../../const/languageTexts";

const REQUEST_URL = `${AppConfigurations.requestServer}/api/v1/beststats`;
const WEAPON_DAMAGE_INCREASE = 10;
const MAIN_STAT_INCREASE = 100;
const CRIT_INCREASE_AMOUNT = 20;
const DH_INCREASE_AMOUNT = 80;
const DET_INCREASE_AMOUNT = 20;
const SKS_INCREASE_AMOUNT = 5;
const SPS_INCREASE_AMOUNT = 5;
const TEN_INCREASE_AMOUNT = 20;

export const STAT_WEIGHTS_REQUEST_COUNT = AppConfigurations.isApp ? 1000 : 1;

export function BestStatsRequestButton(totalState: EquipmentInput, gcdName: string) {
  let [isRunning, setIsRunning] = useState(false);

  let RequestButton = styled(Button)`
    ${requestButtonStyle}
  `;

  let stats = [""].concat(
    getStatWeightNames(totalState.equipmentDatas[0].mainPlayerJobAbbrev, gcdName)
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
    setIsRunning(true);
    setButtonText(loadingButtonText(requestCount));
    let inputJson = JSON.stringify(totalState);
    localStorage.setItem(SINGLE_INPUT_SAVE_NAME, inputJson);

    let statWeightsResponseTable: BestStatsResponseTable = {
      combatTimeMillisecond: totalState.equipmentDatas[0].combatTimeMillisecond,
      mainPlayerPower: totalState.equipmentDatas[0].power,
      mainPlayerJobAbbrev: totalState.equipmentDatas[0].mainPlayerJobAbbrev,
      statAugmentedSimulationData: [],
      partyMemberJobAbbrevs: totalState.equipmentDatas[0].partyMemberJobAbbrevs,
    };

    let responsePromises = [];
    let responses: Array<Response> = [];

    let requests = stats.map((stat) => {
      let augmentedRequest = createAugmentedRequest(totalState.equipmentDatas[0], stat, gcdName);
      return JSON.stringify(
        augmentedRequest
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
    const formattedResponses: Array<Promise<BestStatsResponse>> =
      responses.map(async (response) => {
        const data = await response.json();
        return data;
      });

    const finalResponses = await Promise.all(formattedResponses);
    finalResponses.filter((response) => response.statName === undefined);

    statWeightsResponseTable.statAugmentedSimulationData = finalResponses;

    let responseString = JSON.stringify(statWeightsResponseTable);

    localStorage.setItem(STAT_WEIGHTS_RESPONSE_SAVE_NAME, responseString);

    navigate(`/${BEST_STATS_RESULT_URL}`);
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

function createAugmentedRequest(
  totalState: SingleEquipmentInputSaveState,
  augmentStatName: string,
  gcdName: string
) {
  let jobAbbrev = totalState.mainPlayerJobAbbrev;
  let partner1Id = totalState.mainPlayerPartner1Id;
  let partner2Id = totalState.mainPlayerPartner2Id;

  let autoAttackDelays = AUTO_ATTACK_DELAYS.get(totalState.mainPlayerJobAbbrev);
  if (autoAttackDelays === undefined) {
    autoAttackDelays = 0;
  }
  let power = { ...totalState.power };
  power.autoAttackDelays = autoAttackDelays;

  let augmentAmount = 0;


  if (augmentStatName !== "") {
    if (augmentStatName === WD_STAT_EN_NAME) {
      augmentAmount = WEAPON_DAMAGE_INCREASE;
      power.weaponDamage += WEAPON_DAMAGE_INCREASE;
    }
    if (
      augmentStatName === STR_STAT_EN_NAME ||
      augmentStatName === DEX_STAT_EN_NAME ||
      augmentStatName === INT_STAT_EN_NAME ||
      augmentStatName === MIND_STAT_EN_NAME
    ) {
      augmentAmount = MAIN_STAT_INCREASE;
      power.mainStat += MAIN_STAT_INCREASE;
    }

    if (augmentStatName === CRIT_STAT_EN_NAME) {
      augmentAmount = getStatNeededByStatNameLadderAmount(
        power,
        augmentStatName,
        jobAbbrev,
        CRIT_INCREASE_AMOUNT,
        gcdName
      );
      power.criticalStrike += augmentAmount;
    }
    if (augmentStatName === DH_STAT_EN_NAME) {
      augmentAmount = getStatNeededByStatNameLadderAmount(
        power,
        augmentStatName,
        jobAbbrev,
        DH_INCREASE_AMOUNT,
        gcdName
      );
      power.directHit += augmentAmount;
    }
    if (augmentStatName === DET_STAT_EN_NAME) {
      augmentAmount = getStatNeededByStatNameLadderAmount(
        power,
        augmentStatName,
        jobAbbrev,
        DET_INCREASE_AMOUNT,
        gcdName
      );
      power.determination += augmentAmount;
    }
    if (augmentStatName === SKS_STAT_EN_NAME) {
      let augmentAmountSpeed = getStatNeededByStatNameLadderAmount(
        power,
        augmentStatName,
        jobAbbrev,
        SKS_INCREASE_AMOUNT,
        gcdName
      );
      augmentAmount = jobAbbrev === SMN_EN_NAME ? augmentAmountSpeed : augmentAmountSpeed + 50;
      power.skillSpeed += augmentAmount;
    }
    if (augmentStatName === SPS_STAT_EN_NAME) {
      let augmentAmountSpeed = getStatNeededByStatNameLadderAmount(
        power,
        augmentStatName,
        jobAbbrev,
        SPS_INCREASE_AMOUNT,
        gcdName
      );
      augmentAmount = jobAbbrev === SMN_EN_NAME ? augmentAmountSpeed : augmentAmountSpeed + 50;

      power.spellSpeed += augmentAmount;
    }
    if (augmentStatName === TEN_STAT_EN_NAME) {
      augmentAmount = getStatNeededByStatNameLadderAmount(
        power,
        augmentStatName,
        jobAbbrev,
        TEN_INCREASE_AMOUNT,
        gcdName
      );
      power.tenacity += augmentAmount;
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

  for (let i = 0; i < totalState.partyMemberJobAbbrevs.length; i++) {
    if (totalState.partyMemberJobAbbrevs[i] === EMPTY_PARTY_MEMBER) {
      if (partyInfo[0].partner1Id !== null && partyInfo[0].partner1Id > i) {
        partyInfo[0].partner1Id -= 1;
      }
      if (partyInfo[0].partner2Id !== null && partyInfo[0].partner2Id > i) {
        partyInfo[0].partner2Id -= 1;
      }
    }
  }

  let playerCount = 0;
  for (let i = 0; i < totalState.partyMemberJobAbbrevs.length; i++) {
    let jobAbbrev = totalState.partyMemberJobAbbrevs[i];
    let bisEquipments = mapJobAbbrevToJobBisEquipments(jobAbbrev);

    if (bisEquipments === undefined) {
      continue;
    }

    let playerTotalState = {
      mainPlayerJobAbbrev: jobAbbrev,
      race: MIDLANDER_HYUR_EN_NAME,
      foodId: bisEquipments.foodId,
      mainPlayerPartner1Id: null,
      mainPlayerPartner2Id: null,
      itemSet: bisEquipments.itemSet,
      gearSetMaterias: bisEquipments.gearSetMaterias,
      combatTimeMillisecond: 0,
      partyMemberJobAbbrevs: [],
      partyMemberIds: [],
      partyMemberIlvl: 0,
      usePot: 1,
      power: defaultPlayerPower(),
      compositionBuffPercent: 0,
    }

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
      power: bisPower,
    });

    playerCount++;
  }

  return {
    mainPlayerId: 0,
    combatTimeMillisecond: totalState.combatTimeMillisecond,
    party: partyInfo,
    statName: augmentStatName,
    augmentAmount: augmentAmount,
    usePot: totalState.usePot === USE_POT_VAL,
    partyIlvlAdjustment: calculateIlvlAdjustment(totalState.partyMemberIlvl),
  };
}
