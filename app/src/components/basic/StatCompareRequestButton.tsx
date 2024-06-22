import { styled, Button } from "@mui/material";
import { useNavigate } from "react-router-dom";
import { CharacterStates } from "src/types/CharacterStates";
import { MapJobAbbrevToJobDefaultStat } from "src/const/StatValue";
import { PartyInfo } from "src/types/PartyStates";
import { ColorConfigurations } from "src/Themes";
import {
  StatCompareRequestSaveName,
  StatCompareResponseSaveName,
} from "src/App";
import { StatCompareRequest } from "src/types/StatCompareRequest";

export function StatCompareRequestButton(
  partyState: string[],
  combatTimeSeconds: number,
  characterState1: CharacterStates,
  characterState2: CharacterStates
) {
  let RequestButton = styled(Button)`
    font-size: 0.8rem;
    margin: 1rem;
    height: 8vh;
    background-color: ${ColorConfigurations.backgroundButton};
    color: black;
  `;

  let navigate = useNavigate();

  const handleClick = async () => {
    let request = createStatCompareRequest(
      partyState,
      combatTimeSeconds,
      characterState1,
      characterState2
    );

    if (request instanceof Error) {
      console.error("Error: ", request.message);
      return;
    }

    let body = JSON.stringify(request);
    console.log(body);

    localStorage.setItem(StatCompareRequestSaveName, body);

    try {
      const response = await fetch(
        "http://localhost:13406/api/v1/statcompare",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: body,
        }
      );

      if (response.ok) {
        console.log("POST 요청이 성공했습니다.");
        // JavaScript 객체를 key-value dictionary로 변환
        const responseString = JSON.stringify(await response.json());
        localStorage.setItem(StatCompareResponseSaveName, responseString);
        navigate("/statcompareresult");
      } else {
        console.error("POST 요청이 실패했습니다.");
      }
    } catch (error) {
      console.error("오류 발생: ", error);
    }
  };
  return (
    <RequestButton variant="contained" onClick={handleClick}>
      Simulate
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
  let i = 0;

  for (i = 0; i < partyState.length; i++) {
    let defaultStat = MapJobAbbrevToJobDefaultStat(partyState[i]);

    if (defaultStat === undefined) {
      continue;
    }

    partyInfo.push({
      playerId: playerCount + 1,
      job: partyState[i],
      stats: defaultStat,
    });

    playerCount++;
  }

  return {
    mainPlayerId: 0,
    mainPlayerJob: characterState1.jobName,
    combatTimeMillisecond: combatTimeSeconds * 1000,
    mainPlayerStat1: characterState1.stats,
    mainPlayerStat2: characterState2.stats,
    party: partyInfo,
  };
}
