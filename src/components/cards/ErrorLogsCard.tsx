import React from "react";
import { BaseCard } from "./BaseCard";
import { useQuery } from "@tanstack/react-query";
import { errorAPI } from "@/lib/tauri-api";
import { AlertTriangle, CheckCircle2 } from "lucide-react";
import type { AttemptError } from "@/types";

export const ErrorLogsCard: React.FC = () => {
  const { data: errorTypes } = useQuery({
    queryKey: ["error-types"],
    queryFn: () => errorAPI.getTypes(),
  });

  // Mock unresolved errors
  const mockErrors: AttemptError[] = [
    {
      id: "1",
      attempt_id: "a1",
      error_type_id: 1,
      description: "Conceptual gap in integration",
      is_resolved: false,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    },
  ];

  return (
    <BaseCard id="errors" title="Recent Errors" className="col-span-1">
      <div className="space-y-2">
        {mockErrors.map((error) => {
          const errorType = errorTypes?.find((t) => t.id === error.error_type_id);
          return (
            <div
              key={error.id}
              className={`p-3 rounded border ${error.is_resolved
                  ? "bg-emerald-500/10 border-emerald-500/30"
                  : "bg-red-500/10 border-red-500/30"
                }`}
            >
              <div className="flex items-start gap-2">
                {error.is_resolved ? (
                  <CheckCircle2 size={16} className="text-emerald-400 flex-shrink-0 mt-0.5" />
                ) : (
                  <AlertTriangle size={16} className="text-red-400 flex-shrink-0 mt-0.5" />
                )}
                <div className="flex-1 min-w-0">
                  <p className="text-xs font-semibold text-slate-200">
                    {errorType?.name}
                  </p>
                  <p className="text-xs text-slate-400 mt-1 truncate">
                    {error.description}
                  </p>
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </BaseCard>
  );
};
