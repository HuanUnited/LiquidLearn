import React, { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { useAppStore } from "../../stores/appStore";
import { useUIStore } from "../../stores/uiStore";
import { TIMER_PHASES } from "@/stores/timerStore";
import { ScrollArea } from "@/components/ui/scroll-area";

// Sample guidelines HTML content by phase
const GUIDELINES_BY_PHASE: Record<number, string> = {
  1: `
    <h2>Phase 1.1: Foundation & Concepts</h2>
    <p>Focus on understanding the core concepts and principles. Don't worry about speed.</p>
    <ul>
      <li>Read the theory carefully</li>
      <li>Write down key definitions</li>
      <li>Ask clarifying questions</li>
    </ul>
  `,
  2: `
    <h2>Phase 1.2: Shallow Practice</h2>
    <p>Apply concepts to simple problems. Build confidence before complexity.</p>
    <ul>
      <li>Work through guided examples</li>
      <li>Practice basic problem types</li>
      <li>Review mistakes immediately</li>
    </ul>
  `,
  3: `
    <h2>Phase 2.1: Deep Practice</h2>
    <p>Tackle harder problems. Focus on understanding your mistakes.</p>
    <ul>
      <li>Attempt problems without solutions</li>
      <li>Log all errors and patterns</li>
      <li>Review error analysis</li>
    </ul>
  `,
  4: `
    <h2>Phase 2.2: Advanced Problem Solving</h2>
    <p>Combine concepts. Work on synthesis and application.</p>
    <ul>
      <li>Multi-step problems</li>
      <li>Real-world scenarios</li>
      <li>Develop intuition</li>
    </ul>
  `,
  5: `
    <h2>Phase 3: Fluency & Speed</h2>
    <p>Build speed and automaticity. Practice until it becomes natural.</p>
    <ul>
      <li>Timed practice sessions</li>
      <li>Focus on efficiency</li>
      <li>Mental shortcuts</li>
    </ul>
  `,
  6: `
    <h2>Phase 4: Teaching & Mastery</h2>
    <p>Teach others. Identify knowledge gaps. Achieve true mastery.</p>
    <ul>
      <li>Explain concepts to others</li>
      <li>Create teaching materials</li>
      <li>Review comprehensive summaries</li>
    </ul>
  `,
};

export const GuidelinesModal: React.FC = () => {
  const { modals, closeModal } = useUIStore();
  const { currentTimerPhase } = useAppStore();

  const currentPhase = TIMER_PHASES.find((p) => p.number === currentTimerPhase);
  const guidelinesHtml = GUIDELINES_BY_PHASE[currentTimerPhase] || "";

  return (
    <Dialog open={modals.guidelines} onOpenChange={() => closeModal("guidelines")}>
      <DialogContent className="bg-slate-900 border-slate-700 max-w-2xl max-h-[80vh]">
        <DialogHeader>
          <DialogTitle className="text-slate-100">
            Phase {currentPhase?.label} Guidelines
          </DialogTitle>
        </DialogHeader>

        <ScrollArea className="h-full pr-4">
          <div
            className="text-slate-300 space-y-4 text-sm prose prose-invert max-w-none"
            dangerouslySetInnerHTML={{ __html: guidelinesHtml }}
          />
        </ScrollArea>
      </DialogContent>
    </Dialog>
  );
};
