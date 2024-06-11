import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from "recharts";
import { AbilityImageTick, renderAbilityDpsLabel } from "./CustomBar";

export interface DamageChartData {
  icon: String;
  rdps: number;
  pdps: number;
}

export function DamageProfileBarChart(props: { data: DamageChartData[] }) {
  return (
    <ResponsiveContainer width="100%" height="100%">
      <BarChart data={props.data} barCategoryGap={3} layout="vertical">
        <CartesianGrid strokeDasharray="3 3" />
        <XAxis type="number" />
        <YAxis dataKey="icon" type="category" tick={AbilityImageTick} />
        <Tooltip />

        <Bar dataKey="pdps" fill="#8884d8" label={renderAbilityDpsLabel} />
      </BarChart>
    </ResponsiveContainer>
  );
}
