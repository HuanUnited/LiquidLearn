import React, { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { useUIStore } from "../../stores/uiStore";
import { useSubmitAttempt } from "@/hooks/useProblems";
import { useErrorTypes } from "@/hooks";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Plus, Trash2 } from "lucide-react";
import toast from "react-hot-toast";

export const AttemptModal: React.FC = () => {
  const { modals, closeModal } = useUIStore();
  const { mutate: submitAttempt, isPending } = useSubmitAttempt();
  const { data: errorTypes } = useErrorTypes();

  const [isSolved, setIsSolved] = useState(true);
  const [commentary, setCommentary] = useState("");
  const [selectedErrors, setSelectedErrors] = useState<
    Array<{ error_type_id: number; description: string }>
  >([]);
  const [quality, setQuality] = useState(3);

  const handleSubmit = () => {
    if (!isSolved && selectedErrors.length === 0) {
      toast.error("Please select at least one error");
      return;
    }

    submitAttempt(
      {
        problemId: "mock-problem-id", // Replace with actual
        isSolved,
        commentary,
        errors: selectedErrors,
        quality,
        timeSpent: 300,
      },
      {
        onSuccess: () => {
          closeModal("attempt");
          setIsSolved(true);
          setCommentary("");
          setSelectedErrors([]);
          setQuality(3);
        },
      }
    );
  };

  return (
    <Dialog open={modals.attempt} onOpenChange={() => closeModal("attempt")}>
      <DialogContent className="bg-slate-900 border-slate-700 max-w-md">
        <DialogHeader>
          <DialogTitle className="text-slate-100">Record Attempt</DialogTitle>
        </DialogHeader>

        <div className="space-y-4">
          {/* Solved toggle */}
          <div className="flex items-center gap-3">
            <button
              onClick={() => setIsSolved(!isSolved)}
              className={`px-4 py-2 rounded border transition-colors ${isSolved
                ? "bg-emerald-500/20 border-emerald-500/50 text-emerald-300"
                : "bg-red-500/20 border-red-500/50 text-red-300"
                }`}
            >
              {isSolved ? "✓ Solved" : "✗ Unsolved"}
            </button>
          </div>

          {/* Quality rating (if solved) */}
          {isSolved && (
            <div>
              <label className="text-xs font-semibold text-slate-300 mb-2 block">
                Confidence
              </label>
              <div className="flex gap-2">
                {[1, 2, 3, 4, 5].map((q) => (
                  <button
                    key={q}
                    onClick={() => setQuality(q)}
                    className={`flex-1 py-2 rounded border transition-colors ${quality === q
                      ? "bg-blue-500/20 border-blue-500/50 text-blue-300"
                      : "bg-slate-700/50 border-slate-600/50 text-slate-400"
                      }`}
                  >
                    {q}
                  </button>
                ))}
              </div>
            </div>
          )}

          {/* Errors (if unsolved) */}
          {!isSolved && (
            <div>
              <label className="text-xs font-semibold text-slate-300 mb-2 block">
                Errors
              </label>
              <div className="space-y-2 max-h-48 overflow-y-auto">
                {errorTypes?.map((type) => (
                  <div
                    key={type.id}
                    onClick={() => {
                      const exists = selectedErrors.find(
                        (e) => e.error_type_id === type.id
                      );
                      if (exists) {
                        setSelectedErrors(
                          selectedErrors.filter((e) => e.error_type_id !== type.id)
                        );
                      } else {
                        setSelectedErrors([
                          ...selectedErrors,
                          { error_type_id: type.id, description: "" },
                        ]);
                      }
                    }}
                    className={`p-2 rounded border cursor-pointer transition-colors ${selectedErrors.find((e) => e.error_type_id === type.id)
                      ? "bg-red-500/20 border-red-500/50"
                      : "bg-slate-700/50 border-slate-600/50 hover:border-slate-500"
                      }`}
                  >
                    <p className="text-xs font-semibold text-slate-200">
                      {type.name}
                    </p>
                    <p className="text-xs text-slate-400">{type.description}</p>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Commentary */}
          <div>
            <label className="text-xs font-semibold text-slate-300 mb-2 block">
              Notes
            </label>
            <Textarea
              value={commentary}
              onChange={(e) => setCommentary(e.target.value)}
              placeholder="Add any notes about this attempt..."
              className="bg-slate-800 border-slate-700 text-slate-100 text-sm"
              rows={3}
            />
          </div>
        </div>

        <DialogFooter>
          <Button
            onClick={() => closeModal("attempt")}
            variant="outline"
            className="border-slate-600"
          >
            Cancel
          </Button>
          <Button
            onClick={handleSubmit}
            disabled={isPending}
            className="bg-blue-600 hover:bg-blue-700"
          >
            {isPending ? "Saving..." : "Save Attempt"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
