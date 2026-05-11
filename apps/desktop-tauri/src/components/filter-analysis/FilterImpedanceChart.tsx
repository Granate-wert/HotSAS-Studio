import * as echarts from "echarts";
import { useEffect, useRef } from "react";
import type { FilterSweepPoint } from "../../types";

interface Props {
  points: FilterSweepPoint[];
}

export function FilterImpedanceChart({ points }: Props) {
  const ref = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!ref.current) return;

    const chart = echarts.init(ref.current);

    const hasData = points.length > 0;
    const zinData = hasData
      ? points
          .filter((p) => p.zin_magnitude_ohm !== null)
          .map((p) => [p.frequency_hz, p.zin_magnitude_ohm])
      : [];
    const zoutData = hasData
      ? points
          .filter((p) => p.zout_magnitude_ohm !== null)
          .map((p) => [p.frequency_hz, p.zout_magnitude_ohm])
      : [];

    const series: echarts.EChartsOption["series"] = [];
    if (zinData.length > 0) {
      series.push({
        name: "Zin",
        type: "line",
        showSymbol: false,
        data: zinData,
      });
    }
    if (zoutData.length > 0) {
      series.push({
        name: "Zout",
        type: "line",
        showSymbol: false,
        data: zoutData,
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
        name: "Impedance (Ω)",
        axisLabel: { color: "#9aa8ba" },
        nameTextStyle: { color: "#9aa8ba" },
        splitLine: { lineStyle: { color: "#263244" } },
      },
      series,
    });

    const resize = () => chart.resize();
    window.addEventListener("resize", resize);

    return () => {
      window.removeEventListener("resize", resize);
      chart.dispose();
    };
  }, [points]);

  const hasImpedance = points.some(
    (p) => p.zin_magnitude_ohm !== null || p.zout_magnitude_ohm !== null,
  );

  if (!hasImpedance) {
    return null;
  }

  return <div ref={ref} className="chart" style={{ height: 300 }} />;
}
