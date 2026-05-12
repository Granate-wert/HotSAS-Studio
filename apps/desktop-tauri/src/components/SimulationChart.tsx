import * as echarts from "echarts";
import { useEffect, useRef } from "react";
import { Stack, Text } from "@mantine/core";
import { BarChart3 } from "lucide-react";
import type { GraphSeriesDto, SimulationResultDto } from "../types";

export function SimulationChart({ simulation }: { simulation: SimulationResultDto | null }) {
  const ref = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!ref.current) {
      return;
    }

    const chart = echarts.init(ref.current);
    const series = simulation?.graph_series ?? [];
    chart.setOption({
      backgroundColor: "transparent",
      tooltip: { trigger: "axis" },
      legend: { textStyle: { color: "#c9d2df" } },
      grid: { left: 58, right: 24, top: 36, bottom: 42 },
      xAxis: {
        type: "log",
        name: "Hz",
        axisLabel: { color: "#9aa8ba" },
        nameTextStyle: { color: "#9aa8ba" },
        splitLine: { lineStyle: { color: "#263244" } },
      },
      yAxis: {
        type: "value",
        axisLabel: { color: "#9aa8ba" },
        splitLine: { lineStyle: { color: "#263244" } },
      },
      series: series.map((item: GraphSeriesDto) => ({
        name: `${item.name} (${item.y_unit})`,
        type: "line",
        showSymbol: false,
        data: item.points,
      })),
    });

    const resize = () => chart.resize();
    window.addEventListener("resize", resize);

    return () => {
      window.removeEventListener("resize", resize);
      chart.dispose();
    };
  }, [simulation]);

  const hasData = simulation && simulation.graph_series && simulation.graph_series.length > 0;

  if (!hasData) {
    return (
      <Stack align="center" justify="center" gap="sm" style={{ height: "100%", padding: 24 }}>
        <BarChart3 size={32} color="#56657a" />
        <Text size="sm" c="dimmed" ta="center">
          No simulation results yet.
        </Text>
        <Text size="xs" c="dimmed" ta="center">
          Run a simulation from the Simulation tab or Simulation Dashboard to see graphs here.
        </Text>
      </Stack>
    );
  }

  return <div ref={ref} className="chart" />;
}
