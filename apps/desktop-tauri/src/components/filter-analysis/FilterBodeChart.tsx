import * as echarts from "echarts";
import { useEffect, useRef } from "react";
import type { FilterSweepPoint } from "../../types";

interface Props {
  points: FilterSweepPoint[];
}

export function FilterBodeChart({ points }: Props) {
  const gainRef = useRef<HTMLDivElement | null>(null);
  const phaseRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!gainRef.current || !phaseRef.current) return;

    const gainChart = echarts.init(gainRef.current);
    const phaseChart = echarts.init(phaseRef.current);

    const hasData = points.length > 0;

    const gainData = hasData
      ? points.filter((p) => p.gain_db !== null).map((p) => [p.frequency_hz, p.gain_db])
      : [];

    const phaseData = hasData
      ? points.filter((p) => p.phase_deg !== null).map((p) => [p.frequency_hz, p.phase_deg])
      : [];

    const commonXAxis = {
      type: "log" as const,
      name: "Frequency (Hz)",
      axisLabel: { color: "#9aa8ba" },
      nameTextStyle: { color: "#9aa8ba" },
      splitLine: { lineStyle: { color: "#263244" } },
    };

    gainChart.setOption({
      backgroundColor: "transparent",
      tooltip: { trigger: "axis" },
      legend: { textStyle: { color: "#c9d2df" } },
      grid: { left: 58, right: 24, top: 36, bottom: 42 },
      xAxis: commonXAxis,
      yAxis: {
        type: "value" as const,
        name: "Gain (dB)",
        axisLabel: { color: "#9aa8ba" },
        nameTextStyle: { color: "#9aa8ba" },
        splitLine: { lineStyle: { color: "#263244" } },
      },
      series: [
        {
          name: "Gain",
          type: "line",
          showSymbol: false,
          data: gainData,
        },
      ],
    });

    phaseChart.setOption({
      backgroundColor: "transparent",
      tooltip: { trigger: "axis" },
      legend: { textStyle: { color: "#c9d2df" } },
      grid: { left: 58, right: 24, top: 36, bottom: 42 },
      xAxis: commonXAxis,
      yAxis: {
        type: "value" as const,
        name: "Phase (deg)",
        axisLabel: { color: "#9aa8ba" },
        nameTextStyle: { color: "#9aa8ba" },
        splitLine: { lineStyle: { color: "#263244" } },
      },
      series: [
        {
          name: "Phase",
          type: "line",
          showSymbol: false,
          data: phaseData,
        },
      ],
    });

    const resize = () => {
      gainChart.resize();
      phaseChart.resize();
    };
    window.addEventListener("resize", resize);

    return () => {
      window.removeEventListener("resize", resize);
      gainChart.dispose();
      phaseChart.dispose();
    };
  }, [points]);

  return (
    <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16 }}>
      <div ref={gainRef} className="chart" style={{ height: 300 }} />
      <div ref={phaseRef} className="chart" style={{ height: 300 }} />
    </div>
  );
}
