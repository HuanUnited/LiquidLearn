import React from "react";
import { BaseCard } from "./BaseCard";
import { useQuery } from "@tanstack/react-query";
import { subjectAPI, topicAPI } from "@/lib/tauri-api";
import { Crown, Zap } from "lucide-react";
import type { Subject, Topic } from "@/types";

export const MasteryCard: React.FC = () => {
  const { data: subjects } = useQuery({
    queryKey: ["subjects"],
    queryFn: () => subjectAPI.list(),
  });

  const masteredSubjects = subjects?.filter((s: Subject) => Math.random() > 0.5) || [];
  const masteredTopics = subjects
    ?.flatMap((s: Subject) =>
      s.id ? [s.id] : []
    )
    .slice(0, 3) || [];

  return (
    <BaseCard id="mastery" title="Mastery Progress" className="col-span-1">
      <div className="space-y-4">
        {/* Mastered Subjects */}
        <div>
          <div className="flex items-center gap-2 mb-2">
            <Crown size={16} className="text-orange-400" />
            <h4 className="text-xs font-semibold text-slate-300">Mastered Subjects</h4>
          </div>
          <div className="space-y-1">
            {masteredSubjects.length > 0 ? (
              masteredSubjects.map((subject: Subject) => (
                <div
                  key={subject.id}
                  className="px-3 py-2 bg-gradient-to-r from-orange-500/20 to-red-500/20 border border-orange-500/30 rounded text-sm text-orange-300 flex items-center gap-2"
                >
                  <div className="w-2 h-2 rounded-full bg-orange-400" />
                  {subject.name}
                </div>
              ))
            ) : (
              <p className="text-xs text-slate-500">No mastered subjects yet</p>
            )}
          </div>
        </div>

        {/* Mastered Topics */}
        <div>
          <div className="flex items-center gap-2 mb-2">
            <Zap size={16} className="text-yellow-400" />
            <h4 className="text-xs font-semibold text-slate-300">Mastered Topics</h4>
          </div>
          <div className="space-y-1">
            {masteredTopics.length > 0 ? (
              masteredTopics.map((id: string, idx: number) => (
                <div
                  key={idx}
                  className="px-3 py-2 bg-yellow-500/20 border border-yellow-500/30 rounded text-sm text-yellow-300 flex items-center gap-2"
                >
                  <div className="w-2 h-2 rounded-full bg-yellow-400" />
                  Topic {idx + 1}
                </div>
              ))
            ) : (
              <p className="text-xs text-slate-500">No mastered topics yet</p>
            )}
          </div>
        </div>
      </div>
    </BaseCard>
  );
};
