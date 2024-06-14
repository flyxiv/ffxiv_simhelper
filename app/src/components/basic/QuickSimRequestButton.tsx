import { Button } from "@mui/material";
import "./QuickSimRequestButton.css";
import { useNavigate } from "react-router-dom";
import { CharacterStates } from "src/types/CharacterStates";
import { MapJobAbbrevToJobDefaultStat } from "src/const/StatValue";
import { PartyInfo } from "src/types/QuickSimRequest";

export function QuickSimRequestButton(
  partyState: string[],
  combatTimeSeconds: number,
  characterState: CharacterStates
) {
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

    console.log(request);

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
    <Button variant="contained" className="RequestButton" onClick={handleClick}>
      Begin Simulation
    </Button>
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

  let i = 0;
  console.log(partyState);

  for (i = 0; i < partyState.length; i++) {
    let defaultStat = MapJobAbbrevToJobDefaultStat(partyState[i]);

    if (defaultStat == undefined) {
      continue;
    }

    partyInfo.push({
      playerId: i + 1,
      job: partyState[i],
      stats: defaultStat,
    });
  }

  return {
    mainPlayerId: 0,
    combatTimeMillisecond: combatTimeSeconds * 1000,
    party: partyInfo,
  };
}
