import React from "react";
import { useQuery } from "@tanstack/react-query";
import { BaseCard } from "./BaseCard";
import { problemAPI } from "@/lib/tauri-api";
import { useAppStore } from "../../stores/appStore";
import { useUIStore } from "../../stores/uiStore";
import { AlertCircle, CheckCircle2 } from "lucide-react";
import type { Problem } from "@/types";

export const ProblemListCard: React.FC = () => {
  const { selectedTopic } = useAppStore();
  const { openModal } = useUIStore();

  const { data: problems, isLoading } = useQuery({
    queryKey: ["problems", selectedTopic?.id],
    queryFn: () =>
      selectedTopic
        ? problemAPI.listByTopic(selectedTopic.id)
        : Promise.resolve([]),
    enabled: !!selectedTopic,
  });

  if (!selectedTopic) {
    return (
      <BaseCard id="problems" title="Problems" className="col-span-2">
        <div className="text-slate-400 text-sm">Select a topic to view problems</div>
      </BaseCard>
    );
  }

  return (
    <BaseCard
      id="problems"
      title={`Problems - ${selectedTopic.name}`}
      isLoading={isLoading}
      className="col-span-2"
    >
      <div className="space-y-2">
        {problems?.map((problem: Problem) => (
          <div
            key={problem.id}
            className="p-3 bg-slate-800/50 rounded border border-slate-700/50 hover:border-blue-500/50 cursor-pointer transition-colors group relative"
            onClick={() => openModal("attempt")}
          >
            <div className="flex items-start justify-between gap-2">
              <div className="flex-1">
                <p className="text-sm font-medium text-slate-100">
                  {problem.title}
                </p>
                <p className="text-xs text-slate-400 mt-1">
                  Difficulty: {problem.difficulty}/5
                </p>
              </div>

              {/* Error badge */}
              {problem.total_unresolved_errors > 0 && (
                <div className="flex-shrink-0 relative">
                  <div className="flex items-center justify-center w-6 h-6 rounded-full bg-red-500/80 text-white text-xs font-bold">
                    {problem.total_unresolved_errors}
                  </div>
                </div>
              )}

              {/* Status icon */}
              {problem.is_solved ? (
                <CheckCircle2 size={18} className="text-emerald-500 flex-shrink-0" />
              ) : (
                <AlertCircle size={18} className="text-amber-500 flex-shrink-0" />
              )}
            </div>
          </div>
        ))}
      </div>
    </BaseCard>
  );
};
