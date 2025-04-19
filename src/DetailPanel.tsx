import React from "react";
import { MetricKey } from "./Bars";
import "./DetailPanel.css";

interface RealtimeStats {
  cpu_usage: number;
  detailed_cpu_usage: number[];
  memory_usage_percent: number;
  networks: { name: string; received: number; transmitted: number }[];
}

interface CpuMeta {
  cpu: {
    name: string;
    core_count: number;
    frequency: number;
  };
}

interface Props {
  metric: MetricKey | null;
  stats: RealtimeStats | null;
  meta: CpuMeta | null; // CPU 정적 정보는 상위에서 전달
}

const DetailPanel: React.FC<Props> = ({ metric, stats, meta }) => {
  if (!metric || !stats) return null;

  /* CPU */
  if (metric === "cpu" && meta) {
    return (
      <div className="detail">
        <h2>CPU Detail</h2>
        <p>Name: {meta.cpu.name}</p>
        <p>Base Freq: {meta.cpu.frequency} MHz</p>
        <p>Cores: {meta.cpu.core_count}</p>
        <p>Usage: {stats.cpu_usage.toFixed(1)}%</p>
      </div>
    );
  }

  /* Memory */
  if (metric === "mem") {
    return (
      <div className="detail">
        <h2>Memory Detail</h2>
        <p>Usage: {stats.memory_usage_percent.toFixed(1)}%</p>
      </div>
    );
  }

  /* Net RX */
  if (metric === "rx") {
    const rx = stats.networks.reduce((s, n) => s + n.received, 0);
    return (
      <div className="detail">
        <h2>Network RX Detail</h2>
        <p>RX Δ: {rx} bytes/s</p>
      </div>
    );
  }
  /* Net TX */
  if (metric === "tx") {
    const tx = stats.networks.reduce((s, n) => s + n.transmitted, 0);
    return (
      <div className="detail">
        <h2>Network TX Detail</h2>
        <p>TX Δ: {tx} bytes/s</p>
      </div>
    );
  }

  return null;
};

export default DetailPanel;