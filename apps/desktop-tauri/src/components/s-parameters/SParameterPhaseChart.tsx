import * as echarts from "echarts";
import { useEffect, useRef } from "react";
import type { SParameterCurvePoint } from "../../types";

interface Props {
  points: SParameterCurvePoint[];
  showS11: boolean;
  showS21: boolean;
  showS12: boolean;
  showS22: boolean;
}

export function SParameterPhaseChart({ points, showS11, showS21, showS12, showS22 }: Props) {
  const chartRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!chartRef.current) return;
    const chart = echarts.init(chartRef.current);

    const series: echarts.SeriesOption[] = [];
    if (showS11) {
      series.push({
        name: "S11",
        type: "line",
        showSymbol: false,
        data: points
          .filter((p) => p.s11_phase_deg !== null)
          .map((p) => [p.frequency_hz, p.s11_phase_deg]),
      });
    }
    if (showS21) {
      series.push({
        name: "S21",
        type: "line",
        showSymbol: false,
        data: points
          .filter((p) => p.s21_phase_deg !== null)
          .map((p) => [p.frequency_hz, p.s21_phase_deg]),
      });
    }
    if (showS12) {
      series.push({
        name: "S12",
        type: "line",
        showSymbol: false,
        data: points
          .filter((p) => p.s12_phase_deg !== null)
          .map((p) => [p.frequency_hz, p.s12_phase_deg]),
      });
    }
    if (showS22) {
      series.push({
        name: "S22",
        type: "line",
        showSymbol: false,
        data: points
          .filter((p) => p.s22_phase_deg !== null)
          .map((p) => [p.frequency_hz, p.s22_phase_deg]),
      });
    }

    chart.setOption({
      backgroundColor: "transparent",
      tooltip: { trigger: "axis" },
      legend: { textStyle: { color: "#c9d2df" } },
      grid: { left: 58, right: 24, top: 36, bottom: 42 },
      xAxis: {
        type: "log" as const,
        name: "Frequency (Hz)",
        axisLabel: { color: "#9aa8ba" },
        nameTextStyle: { color: "#9aa8ba" },
        splitLine: { lineStyle: { color: "#263244" } },
      },
      yAxis: {
        type: "value" as const,
        name: "Phase (deg)",
        axisLabel: { color: "#9aa8ba" },
        nameTextStyle: { color: "#9aa8ba" },
        splitLine: { lineStyle: { color: "#263244" } },
      },
      series,
    });

    const handleResize = () => chart.resize();
    window.addEventListener("resize", handleResize);
    return () => {
      window.removeEventListener("resize", handleResize);
      chart.dispose();
    };
  }, [points, showS11, showS21, showS12, showS22]);

  return <div ref={chartRef} style={{ width: "100%", height: 320 }} />;
}
