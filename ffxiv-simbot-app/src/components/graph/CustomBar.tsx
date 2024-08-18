import { jobAbbrevToJobIconPath } from "../icon/jobicon/JobIconFactory";

interface JobImageTickProps {
  payload: { value: string };
  x: number;
  y: number;
}

interface AbilityImageTickProps {
  payload: { value: string };
  x: number;
  y: number;
}

export const JobImageTick = (props: JobImageTickProps) => {
  const jobIcon = jobAbbrevToJobIconPath(props.payload.value);

  return (
    <g transform={`translate(${props.x},${props.y})`}>
      <image href={jobIcon} x="-30" y="-15" width="30" height="30" />
    </g>
  );
};

interface BarLabelProps {
  payload: { value: string };
  x: number;
  y: number;
  width: number;
  height: number;
  value: number;
}

export const renderAbilityDpsLabel = (props: BarLabelProps) => {
  let value = Math.round(props.value);
  return (
    <text
      y={props.y + props.height}
      x={props.x + props.width}
      fill="#666"
      textAnchor="middle"
      dy={-6}
    >{`${value}`}</text>
  );
};

export const AbilityImageTick = (props: AbilityImageTickProps) => {
  const abilityIcon = props.payload.value;

  return (
    <g transform={`translate(${props.x},${props.y})`}>
      <image href={abilityIcon} x="-30" y="-15" width="30" height="30" />
    </g>
  );
};
