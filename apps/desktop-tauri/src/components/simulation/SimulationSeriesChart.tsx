import * as echarts from "echarts";
import { useEffect, useRef } from "react";
import type { SimulationSeriesDto } from "../../types";

interface Props {
  series: SimulationSeriesDto[];
}

export function SimulationSeriesChart({ series }: Props) {
  const ref = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!ref.current) {
      return;
    }

    const chart = echarts.init(ref.current);
    chart.setOption({
      backgroundColor: "transparent",
      tooltip: { trigger: "axis" },
      legend: { textStyle: { color: "#c9d2df" } },
      grid: { left: 58, right: 24, top: 36, bottom: 42 },
      xAxis: {
        type: series.length > 0 && series[0].x_unit === "Hz" ? "log" : "value",
        name: series.length > 0 ? (series[0].x_unit ?? "") : "",
        axisLabel: { color: "#9aa8ba" },
        nameTextStyle: { color: "#9aa8ba" },
        splitLine: { lineStyle: { color: "#263244" } },
      },
      yAxis: {
        type: "value",
        name: series.length > 0 ? (series[0].y_unit ?? "") : "",
        axisLabel: { color: "#9aa8ba" },
        nameTextStyle: { color: "#9aa8ba" },
        splitLine: { lineStyle: { color: "#263244" } },
      },
      series: series.map((item) => ({
        name: item.label,
        type: "line",
        showSymbol: false,
        data: item.points.map((p) => [p.x, p.y]),
      })),
    });

    const resize = () => chart.resize();
    window.addEventListener("resize", resize);

    return () => {
      window.removeEventListener("resize", resize);
      chart.dispose();
    };
  }, [series]);

  return <div ref={ref} className="chart" style={{ height: 300 }} />;
}
