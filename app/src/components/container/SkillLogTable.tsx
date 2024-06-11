import { SkillLog } from "src/types/QuickSimResponse";
import { Box } from "@mui/material";
import { SkillIdToIconPathFactory } from "../abilityicon/SkillIconFactory";
import "./SkillLogTable.css";

const SkillEntity = (props: {
  combatTime: number;
  skillId: number;
  target: number | null;
}) => {
  let skillIcon = SkillIdToIconPathFactory(props.skillId);

  return (
    <Box className="SkillEntity">
      <div className="CombatTime">{props.combatTime}</div>
      <div className="SkillIconEntity">
        <img src={skillIcon} alt={skillIcon} height={30} width={30} />
      </div>
      <div className="TargetEntity">{props.target}</div>
    </Box>
  );
};

export const SkillLogTable = (props: { skilllog: SkillLog[] }) => {
  return (
    <Box className="SkillLogTable">
      {props.skilllog.map((skillEntity) => (
        <SkillEntity
          combatTime={skillEntity.time}
          skillId={skillEntity.skillId}
          target={skillEntity.target}
        />
      ))}
    </Box>
  );
};
