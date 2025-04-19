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
  compact: boolean;              // 상세 패널이 열렸을 때 true
}

export default function Bars({ data, onSelect, compact }: Props) {
  return (
    <div className={compact ? "bars-row compact" : "bars-row"}>
      {data.map((b) => (
        <div
          key={b.key}
          className="bar-wrapper"
          onClick={() => onSelect(b.key)}
        >
          <span className="bar-name">{b.label}</span>
          <div className={compact ? "bar compact" : "bar"}>
            <div className="bar-fill" style={{ height: `${b.percent}%` }} />
          </div>
          <span className="bar-percent">{b.percent.toFixed(0)}%</span>
        </div>
      ))}
    </div>
  );
}