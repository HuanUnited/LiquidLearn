import React from "react";
import { useQuery } from "@tanstack/react-query";
import { BaseCard } from "./BaseCard";
import { subjectAPI, topicAPI } from "@/lib/tauri-api";
import { useAppStore } from "../../stores/appStore";
import { ChevronRight, Plus } from "lucide-react";
import type { Subject, Topic } from "@/types";

interface SubjectWithTopics extends Subject {
  topics?: Topic[];
}

export const SubjectListCard: React.FC = () => {
  const { selectedSubject, setSelectedSubject, setSelectedTopic } =
    useAppStore();

  const { data: subjects, isLoading } = useQuery({
    queryKey: ["subjects"],
    queryFn: () => subjectAPI.list(),
  });

  const { data: topics } = useQuery({
    queryKey: ["topics", selectedSubject?.id],
    queryFn: () =>
      selectedSubject ? topicAPI.listBySubject(selectedSubject.id) : [],
    enabled: !!selectedSubject,
  });

  return (
    <BaseCard id="subjects" title="Subjects & Topics" className="col-span-1 row-span-2">
      <div className="space-y-3">
        {/* Subjects */}
        <div>
          <h4 className="text-xs font-semibold text-slate-300 mb-2 uppercase opacity-70">
            Subjects
          </h4>
          <div className="space-y-1">
            {subjects?.map((subject: Subject) => (
              <button
                key={subject.id}
                onClick={() => {
                  setSelectedSubject(subject);
                  setSelectedTopic(null);
                }}
                className={`w-full text-left px-3 py-2 rounded text-sm transition-colors ${selectedSubject?.id === subject.id
                  ? "bg-blue-500/20 border border-blue-500/50 text-blue-300"
                  : "hover:bg-slate-700/50 text-slate-300"
                  }`}
              >
                <div className="flex items-center justify-between">
                  <span>{subject.name}</span>
                  {selectedSubject?.id === subject.id && (
                    <ChevronRight size={14} />
                  )}
                </div>
              </button>
            ))}
          </div>
        </div>

        {/* Topics */}
        {selectedSubject && topics && topics.length > 0 && (
          <div>
            <h4 className="text-xs font-semibold text-slate-300 mb-2 uppercase opacity-70">
              Topics
            </h4>
            <div className="space-y-1 ml-2">
              {topics.map((topic: Topic) => (
                <button
                  key={topic.id}
                  onClick={() => setSelectedTopic(topic)}
                  className="w-full text-left px-3 py-2 rounded text-sm transition-colors hover:bg-slate-700/50 text-slate-300"
                >
                  {topic.name}
                </button>
              ))}
            </div>
          </div>
        )}
      </div>
    </BaseCard>
  );
};
