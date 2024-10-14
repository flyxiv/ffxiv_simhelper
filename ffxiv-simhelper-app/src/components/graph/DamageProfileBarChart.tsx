import { AppConfigurations } from "../../Themes";
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

// One row of Damage Profile column in DpsAnalysis Result
// 1. Skill Icon
// 2. Skill Name
// 3. Skill Damage Bar + Percentage of Total Damage
// 4. Total Damage
// 5. Cast Count
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
      <Box display="flex" width="10%">
        <SkillIconBox>
          <Box
            component={"img"}
            src={data.icon.valueOf()}
            width={imageSize}
            height={imageSize}
            alt="icon"
          />
        </SkillIconBox>
        <SkillNameBox>
          <Box>
            <Typography variant="body2" fontSize={AppConfigurations.body2FontSize} align="right" >
              {data.name}
            </Typography>
          </Box>
        </SkillNameBox>
      </Box>

      <SkillBarBox>
        <SkillBar />
      </SkillBarBox>

      <SkillPercentageBox>
        <Typography variant="body1" fontSize={AppConfigurations.body2FontSize}>
          {Math.round(damageProportion * 10) / 10}%
        </Typography>
      </SkillPercentageBox>

      <TotalDamageBox>
        <Typography variant="body1" fontSize={AppConfigurations.body2FontSize}>
          {Math.round((data.pdps * combatTimeMilliseconds) / 1000)}
        </Typography>
      </TotalDamageBox>
      <SkillCountBox>
        <Typography variant="body1" fontSize={AppConfigurations.body2FontSize}>{data.castCount}</Typography>
      </SkillCountBox>
    </SkillBox>
  );
}

const SkillPercentBox = styled(Box)`
  ${SkillPercentageBoxStyle}
`;

export const DamageChartTitle = (skillTitleText: string, damagePercentageText: string, totalDamageText: string, castText: string) => {

  return (
    <SkillBox>
      <Box display="flex" width="200px">
        <SkillIconBox />
        <SkillNameBox>
          <Typography variant="body2" fontSize={AppConfigurations.body2FontSize}>
            {skillTitleText}
          </Typography>
        </SkillNameBox>
      </Box>

      <SkillBarBox />

      <SkillPercentBox>
        <Typography variant="body1" fontSize={AppConfigurations.body2FontSize} align="right">
          {damagePercentageText}
        </Typography>
      </SkillPercentBox>

      <TotalDamageBox>
        <Typography variant="body1" fontSize={AppConfigurations.body2FontSize} align="right">
          {totalDamageText}
        </Typography>
      </TotalDamageBox>

      <SkillCountBox>
        <Typography variant="body1" fontSize={AppConfigurations.body2FontSize} align="right">
          {castText}
        </Typography>
      </SkillCountBox>
    </SkillBox>
  );
}