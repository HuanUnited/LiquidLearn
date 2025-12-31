import React from "react";
import { BaseCard } from "./BaseCard";
import { useQuery } from "@tanstack/react-query";
import { fsrsAPI } from "@/lib/tauri-api";

export const HeatmapCard: React.FC = () => {
  const { data: stats } = useQuery({
    queryKey: ["fsrs-stats"],
    queryFn: () => fsrsAPI.getStats(),
    refetchInterval: 5000,
  });

  // Generate last 12 weeks of activity
  const generateHeatmapData = () => {
    const weeks = [];
    const today = new Date();
    for (let i = 83; i >= 0; i--) {
      const date = new Date(today);
      date.setDate(date.getDate() - i);
      weeks.push({
        date,
        count: Math.floor(Math.random() * 5),
      });
    }
    return weeks;
  };

  const heatmapData = generateHeatmapData();

  const getColor = (count: number) => {
    if (count === 0) return "bg-slate-700";
    if (count === 1) return "bg-emerald-900";
    if (count === 2) return "bg-emerald-700";
    if (count === 3) return "bg-emerald-600";
    return "bg-emerald-500";
  };

  return (
    <BaseCard id="heatmap" title="Activity Heatmap" className="col-span-2">
      <div className="flex flex-col gap-4">
        {/* Stats summary */}
        <div className="grid grid-cols-3 gap-2">
          <div className="text-center p-2 bg-slate-800/50 rounded">
            <p className="text-xs text-slate-400">Total Cards</p>
            <p className="text-lg font-bold text-blue-400">{stats?.total_cards || 0}</p>
          </div>
          <div className="text-center p-2 bg-slate-800/50 rounded">
            <p className="text-xs text-slate-400">Due Today</p>
            <p className="text-lg font-bold text-amber-400">{stats?.due_today || 0}</p>
          </div>
          <div className="text-center p-2 bg-slate-800/50 rounded">
            <p className="text-xs text-slate-400">Retention</p>
            <p className="text-lg font-bold text-emerald-400">
              {stats?.retention_rate.toFixed(0)}%
            </p>
          </div>
        </div>

        {/* Heatmap */}
        <div className="overflow-x-auto">
          <div className="flex gap-1">
            {Array.from({ length: 12 }).map((_, week) => (
              <div key={week} className="flex flex-col gap-1">
                {Array.from({ length: 7 }).map((_, day) => {
                  const index = week * 7 + day;
                  const data = heatmapData[index];
                  return (
                    <div
                      key={`${week}-${day}`}
                      className={`w-4 h-4 rounded ${getColor(data?.count || 0)} cursor-pointer hover:ring-1 hover:ring-slate-300 transition-all`}
                      title={data?.date.toLocaleDateString()}
                    />
                  );
                })}
              </div>
            ))}
          </div>
        </div>
      </div>
    </BaseCard>
  );
};
