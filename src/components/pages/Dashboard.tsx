import React, { useEffect, useState } from "react";
import { Responsive, WidthProvider, Layout } from "react-grid-layout";
import "react-grid-layout/css/styles.css";
import "react-resizable/css/styles.css";
import { useUIStore } from "../../stores/uiStore";
import { usePersistentLayout } from "@/hooks";
import { useInitErrorTypes } from "@/hooks";

// Import all cards
import { SubjectListCard } from "@/components/cards/SubjectListCard";
import { ProblemListCard } from "@/components/cards/ProblemListCard";
import { TimerCard } from "@/components/cards/TimerCard";
import { MasteryCard } from "@/components/cards/MasteryCard";
import { HeatmapCard } from "@/components/cards/HeatMapCard";
import { ErrorLogsCard } from "@/components/cards/ErrorLogsCard";

// Import modals
import { AttemptModal } from "@/components/modals/AttemptModal";
import { GuidelinesModal } from "@/components/modals/GuidelinesModal";

import { Menu } from "lucide-react";

const GridLayout = WidthProvider(Responsive);

const defaultLayout = [
  { i: "subjects", x: 0, y: 0, w: 2, h: 3, static: false },
  { i: "problems", x: 2, y: 0, w: 4, h: 2, static: false },
  { i: "timer", x: 6, y: 0, w: 2, h: 2, static: false },
  { i: "mastery", x: 6, y: 2, w: 2, h: 1, static: false },
  { i: "heatmap", x: 2, y: 2, w: 4, h: 1, static: false },
  { i: "errors", x: 0, y: 3, w: 2, h: 1, static: false },
];

export const Dashboard: React.FC = () => {
  const { layout, setLayout, darkMode, setDarkMode } = useUIStore();
  const { getLayout, setLayout: persistLayout } = usePersistentLayout();
  const { mutate: initErrors } = useInitErrorTypes();

  const [currentLayout, setCurrentLayout] = useState(
    layout.length > 0 ? layout : getLayout() || defaultLayout
  );

  useEffect(() => {
    initErrors();
  }, [initErrors]);

  const handleLayoutChange = (newLayout: Layout[]) => {
    setCurrentLayout(newLayout);
    setLayout(newLayout);
    persistLayout(newLayout);
  };

  return (
    <div
      className={`min-h-screen ${darkMode
        ? "bg-slate-950 text-slate-100"
        : "bg-white text-slate-900"
        } transition-colors duration-300`}
    >
      {/* Header */}
      <header className={`border-b ${darkMode ? "border-slate-800" : "border-slate-200"} p-4`}>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Menu size={20} />
            <h1 className="text-2xl font-bold">LiquidLearn</h1>
          </div>
          <button
            onClick={() => setDarkMode(!darkMode)}
            className={`px-4 py-2 rounded border transition-colors ${darkMode
              ? "bg-slate-800 border-slate-700 hover:bg-slate-700"
              : "bg-slate-100 border-slate-300 hover:bg-slate-200"
              }`}
          >
            {darkMode ? "☀️" : "🌙"}
          </button>
        </div>
      </header>

      {/* Grid Container */}
      <div className="p-6 overflow-hidden">
        <GridLayout
          className="layout"
          layouts={currentLayout}
          onLayoutChange={handleLayoutChange}
          cols={{ lg: 8, md: 6, sm: 4, xs: 2, xxs: 1 }}
          rowHeight={80}
          width={1200}
          isDraggable={true}
          isResizable={true}
          containerPadding={[0, 0]}
          margin={[16, 16]}
          compactType="vertical"
          preventCollision={false}
          useCSSTransforms={true}
        >
          <div key="subjects">
            <SubjectListCard />
          </div>
          <div key="problems">
            <ProblemListCard />
          </div>
          <div key="timer">
            <TimerCard />
          </div>
          <div key="mastery">
            <MasteryCard />
          </div>
          <div key="heatmap">
            <HeatmapCard />
          </div>
          <div key="errors">
            <ErrorLogsCard />
          </div>
        </GridLayout>
      </div>

      {/* Modals */}
      <AttemptModal />
      <GuidelinesModal />
    </div>
  );
};
