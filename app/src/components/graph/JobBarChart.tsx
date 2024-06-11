import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
} from "recharts";
import { JobImageTick } from "./CustomBar";
import { ResponsiveContainer } from "recharts";

export interface TeammateChartData {
  rdps: number;
  jobName: String;
}

export function JobBarChartTeammate(props: { data: TeammateChartData[] }) {
  return (
    <ResponsiveContainer width="100%" height="100%">
      <BarChart data={props.data} layout="vertical">
        <CartesianGrid strokeDasharray="3 3" />
        <XAxis type="number" />
        <YAxis dataKey="jobName" type="category" tick={JobImageTick} />
        <Tooltip />

        <Bar dataKey="rdps" fill="#8884d8" />
      </BarChart>
    </ResponsiveContainer>
  );
}
