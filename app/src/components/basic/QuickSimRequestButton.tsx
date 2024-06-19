import { Button, styled } from "@mui/material";
import { useNavigate } from "react-router-dom";
import { CharacterStates } from "src/types/CharacterStates";
import { MapJobAbbrevToJobDefaultStat } from "src/const/StatValue";
import { PartyInfo } from "src/types/QuickSimRequest";
import { ColorConfigurations } from "src/Themes";

export function QuickSimRequestButton(
  partyState: string[],
  combatTimeSeconds: number,
  characterState: CharacterStates
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
    localStorage.setItem("mostRecentRequest", body);

    try {
      const response = await fetch("http://localhost:13406/api/v1/simulate", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: body,
      });

      if (response.ok) {
        console.log("POST 요청이 성공했습니다.");
        // JavaScript 객체를 key-value dictionary로 변환
        const responseString = JSON.stringify(await response.json());
        localStorage.setItem("simulationResult", responseString);
        navigate("/simulationresult");
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

function createQuickSimRequest(
  partyState: string[],
  combatTimeSeconds: number,
  characterState: CharacterStates
) {
  let partyInfo: PartyInfo[] = [
    {
      playerId: 0,
      job: characterState.jobName,
      stats: {
        weaponDamage: characterState.value.weaponDamage,
        mainStat: characterState.value.mainStat,
        criticalStrike: characterState.value.criticalStrike,
        directHit: characterState.value.directHit,
        determination: characterState.value.determination,
        speed: characterState.value.speed,
        tenacity: characterState.value.tenacity,
      },
    },
  ];

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

  console.log(partyInfo);

  return {
    mainPlayerId: 0,
    combatTimeMillisecond: combatTimeSeconds * 1000,
    party: partyInfo,
  };
}
