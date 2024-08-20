import { DamageChartData } from "./GraphData";
import {
  SkillBarBoxStyle,
  SkillBarStyle,
  SkillBoxStyle,
  SkillCountBoxStyle,
  SkillIconBoxStyle,
  SkillNameStyle,
  SkillPercentageBoxStyle,
  TotalDamageBoxStyle,
  SkillTitleBoxStyle,
  imageSize,
  SkillPercentageTitleBoxStyle,
} from "./Style";
import { styled, Box, Typography } from "@mui/material";

const SkillBox = styled(Box)`
  ${SkillBoxStyle}
`;

const SkillIconBox = styled(Box)`
  ${SkillIconBoxStyle}
`;

const SkillNameBox = styled(Box)`
  ${SkillNameStyle}
`;

const SkillPercentageBox = styled(Box)`
  ${SkillPercentageBoxStyle}
`;

const TotalDamageBox = styled(Box)`
  ${TotalDamageBoxStyle}
`;

const SkillCountBox = styled(Box)`
  ${SkillCountBoxStyle}
`;

const SkillBarBox = styled(Box)`
  ${SkillBarBoxStyle}
`;

export function SkillDamageProfile(
  data: DamageChartData,
  totalDamage: number,
  highestSkillDamage: number,
  combatTimeMilliseconds: number
) {
  let rankingPortion = (data.pdps / highestSkillDamage) * 100;
  let damageProportion = (data.pdps / totalDamage) * 100;

  const SkillBar = styled(Box)`
    ${SkillBarStyle(rankingPortion)}
  `;

  return (
    <SkillBox>
      <SkillIconBox>
        <img
          src={data.icon.valueOf()}
          width={imageSize}
          height={imageSize}
          alt="icon"
        />
      </SkillIconBox>
      <SkillNameBox>
        <Typography variant="body2" fontSize={10}>
          {data.name}
        </Typography>
      </SkillNameBox>
      <SkillBarBox>
        <SkillBar />
      </SkillBarBox>
      <SkillPercentageBox>
        <Typography variant="body1">
          {Math.round(damageProportion * 10) / 10}%
        </Typography>
      </SkillPercentageBox>
      <TotalDamageBox>
        <Typography variant="body1">
          {Math.round((data.pdps * combatTimeMilliseconds) / 1000)}
        </Typography>
      </TotalDamageBox>
      <SkillCountBox>
        <Typography variant="body1">{data.castCount}</Typography>
      </SkillCountBox>
    </SkillBox>
  );
}

const SkillTitleBar = styled(Box)`
  ${SkillTitleBoxStyle}
`;

const SkillPercentTitleBox = styled(Box)`
  ${SkillPercentageTitleBoxStyle}
`;

const fontSize = 10;

export const DamageChartTitle = (
  <SkillBox>
    <SkillTitleBar>
      <Typography variant="body1" fontSize={fontSize}>
        Skill
      </Typography>
    </SkillTitleBar>
    <SkillBarBox />
    <SkillPercentTitleBox>
      <Typography variant="body1" fontSize={fontSize}>
        Dmg%
      </Typography>
    </SkillPercentTitleBox>
    <TotalDamageBox>
      <Typography variant="body1" fontSize={fontSize}>
        Total Dmg
      </Typography>
    </TotalDamageBox>
    <SkillCountBox>
      <Typography variant="body1" fontSize={fontSize}>
        Swing
      </Typography>
    </SkillCountBox>
  </SkillBox>
);
