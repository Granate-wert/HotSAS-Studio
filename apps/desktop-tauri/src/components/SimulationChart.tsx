import * as echarts from "echarts";
import { useEffect, useRef } from "react";
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

  return <div ref={ref} className="chart" />;
}
