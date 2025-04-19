import React from "react";
import "./Bars.css";

export type MetricKey = "cpu" | "mem" | "rx" | "tx";

export interface BarData {
  key: MetricKey;
  label: string;
  percent: number; // 0‑100
}

interface Props {
  data: BarData[];
  onSelect: (key: MetricKey) => void;
}

export default function Bars({ data, onSelect }: Props) {
  return (
    <div className="bars-row">
      {data.map(b => (
        <div key={b.key} className="bar-wrapper" onClick={() => onSelect(b.key)}>
          <span className="bar-name">{b.label}</span>
          <div className="bar">
            <div className="bar-fill" style={{ height: `${b.percent}%` }} />
          </div>
          <span className="bar-percent">{b.percent.toFixed(0)}%</span>
        </div>
      ))}
    </div>
  );
}
