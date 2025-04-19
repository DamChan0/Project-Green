import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Bars, { MetricKey, BarData } from "./Bars";
import DetailPanel from "./DetailPanel";

interface RealtimeStats {
  cpu_usage: number;
  detailed_cpu_usage: number[];
  memory_usage_percent: number;
  networks: { name: string; received: number; transmitted: number }[];
}
interface CpuMeta {
  cpu: { name: string; core_count: number; frequency: number };
}
const MAX_NET = 10 * 1024 * 1024;

export default function Dashboard() {
  const [stats, setStats] = useState<RealtimeStats | null>(null);
  const [meta, setMeta] = useState<CpuMeta | null>(null);
  const [selected, setSelected] = useState<MetricKey | null>(null);

  useEffect(() => {
    const id = setInterval(async () => {
      const data = await invoke<RealtimeStats>("get_realtime_stats");
      setStats(data);
    }, 1000);
    return () => clearInterval(id);
  }, []);

  useEffect(() => {
    if (!meta) invoke<CpuMeta>("get_system_info").then(setMeta);
  }, [meta]);

  const netRx = stats?.networks.reduce((s, n) => s + n.received, 0) ?? 0;
  const netTx = stats?.networks.reduce((s, n) => s + n.transmitted, 0) ?? 0;

  const bars: BarData[] = [
    { key: "cpu", label: "CPU", percent: stats?.cpu_usage ?? 0 },
    { key: "mem", label: "Memory", percent: stats?.memory_usage_percent ?? 0 },
    { key: "rx", label: "Net RX", percent: Math.min((netRx / MAX_NET) * 100, 100) },
    { key: "tx", label: "Net TX", percent: Math.min((netTx / MAX_NET) * 100, 100) },
  ];

  return (
    <div className="dash-root">
      <Bars data={bars} onSelect={setSelected} compact={!!selected} />
      {selected && <DetailPanel metric={selected} stats={stats} meta={meta} />}
    </div>
  );
}