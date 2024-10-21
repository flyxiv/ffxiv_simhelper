import { SkillLog } from "../../types/CombatSimulationResult";
import { Box, ListItem, Typography, styled } from "@mui/material";
import { skillIdToIcon } from "../icon/abilityicon/SkillIconFactory";
import {
  SkillEntityBoxStyle,
  SkillIconBoxStyle,
  SkillLogCombatTimeBoxStyle,
  SkillLogRowStyle,
  StatusIconBoxStyle,
  statusBoxWidth,
} from "./Styles";
import { iconPathToName } from "../Util";
import { AppConfigurations } from "../../Themes";
import { StatusIdToIcon } from "..//icon/abilityicon/StatusIconFactory";

const SkillLogRowBox = styled(Box)`
  ${SkillLogRowStyle}
`;

const SkillEntityBox = styled(Box)`
  ${SkillEntityBoxStyle}
`;

const SkillLogCombatTimeBox = styled(Box)`
  ${SkillLogCombatTimeBoxStyle}
`;

const SkillIconBox = styled(Box)`
  ${SkillIconBoxStyle}
`;

// ex) 1700 to 0:01.700
function combatTimeMillisecondToTimeFormat(combatTimeMillisecond: number) {
  let combatTimeMillisecondAbs = Math.abs(combatTimeMillisecond);
  let minutes = Math.floor(combatTimeMillisecondAbs / 60000);
  let seconds = Math.floor((combatTimeMillisecondAbs % 60000) / 1000);
  let milliseconds = combatTimeMillisecondAbs % 1000;
  let sign = combatTimeMillisecond < 0 ? "-" : "";

  let minutesString = minutes.toString().padStart(1, "0");
  let secondsString = seconds.toString().padStart(2, "0");
  let millisecondsString = milliseconds.toString().padStart(3, "0");

  return `${sign}${minutesString}:${secondsString}.${millisecondsString}`;
}

const SkillEntity = (
  combatTimeMillisecond: number,
  skillId: number,
  _: number | null,
  buffs: number[],
  debuffs: number[]
) => {
  let skillIcon = skillIdToIcon(skillId);
  let skillName = iconPathToName(skillIcon);

  const StatusBox = styled(Box)`
    ${StatusIconBoxStyle(statusBoxWidth)}
  `;

  return (
    <SkillEntityBox>
      <SkillLogCombatTimeBox>
        <Typography variant="body2" color="white" sx={{ fontSize: AppConfigurations.body2FontSize }}>
          {combatTimeMillisecondToTimeFormat(combatTimeMillisecond)}
        </Typography>
      </SkillLogCombatTimeBox>

      <SkillIconBox>
        <img src={skillIcon} alt={skillIcon} height={30} width={30} />

        <Box marginLeft={2}>
          <Typography variant="body1" fontSize={AppConfigurations.body2FontSize}>
            {skillName}
          </Typography>
        </Box>
      </SkillIconBox>

      <StatusBox>
        {buffs.map((buffId) => {
          let iconPath = StatusIdToIcon(buffId);

          if (iconPath.includes("unknown")) {
            return <></>;
          } else {
            return (
              <>
                <img
                  src={iconPath}
                  alt={buffId.toString()}
                  height={30}
                  width={30}
                />
              </>
            );
          }
        })}
        {debuffs.map((debuffId) => {
          let iconPath = StatusIdToIcon(debuffId);

          if (iconPath.includes("unknown")) {
            return <></>;
          } else {
            return (
              <img
                src={iconPath}
                alt={debuffId.toString()}
                height={30}
                width={30}
              />
            );
          }
        })}
      </StatusBox>
    </SkillEntityBox>
  );
};

export const SkillLogTable = (skilllog: SkillLog[]) => {
  return skilllog.map((skillEntity) => (
    <ListItem>
      <SkillLogRowBox>
        {SkillEntity(
          skillEntity.time,
          skillEntity.skillId,
          skillEntity.target,
          skillEntity.buffs,
          skillEntity.debuffs
        )}
      </SkillLogRowBox>
    </ListItem>
  ));
};
