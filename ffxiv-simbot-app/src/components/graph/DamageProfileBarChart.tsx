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
  imageSize,
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
      <Box display="flex" width="200px">
        <SkillIconBox>
          <img
            src={data.icon.valueOf()}
            width={imageSize}
            height={imageSize}
            alt="icon"
          />
        </SkillIconBox>
        <SkillNameBox>
          <Box>
            <Typography variant="body2" fontSize={14} align="right" >
              {data.name}
            </Typography>
          </Box>
        </SkillNameBox>
      </Box>

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

const SkillPercentBox = styled(Box)`
  ${SkillPercentageBoxStyle}
`;

const fontSize = 14;

export const DamageChartTitle = (
  <SkillBox>
    <Box display="flex" width="200px">
      <SkillIconBox />
      <SkillNameBox>
        <Typography variant="body2" fontSize={fontSize}>
          Skill
        </Typography>
      </SkillNameBox>
    </Box>

    <SkillBarBox />

    <SkillPercentBox>
      <Typography variant="body1" fontSize={fontSize} align="right">
        Dmg%
      </Typography>
    </SkillPercentBox>

    <TotalDamageBox>
      <Typography variant="body1" fontSize={fontSize} align="right">
        Total Dmg
      </Typography>
    </TotalDamageBox>

    <SkillCountBox>
      <Typography variant="body1" fontSize={fontSize} align="right">
        Swing
      </Typography>
    </SkillCountBox>
  </SkillBox>
);
