export const TIMER_PHASES = [
  { number: 1, label: "1.1", duration: 25 * 60, title: "Foundation & Concepts" },
  { number: 2, label: "1.2", duration: 25 * 60, title: "Shallow Practice" },
  { number: 3, label: "2.1", duration: 25 * 60, title: "Deep Practice" },
  { number: 4, label: "2.2", duration: 25 * 60, title: "Advanced Problem Solving" },
  { number: 5, label: "3", duration: 25 * 60, title: "Fluency & Speed" },
  { number: 6, label: "4", duration: 25 * 60, title: "Teaching & Mastery" },
];

export const ERROR_TYPES = [
  { id: 1, name: "Conceptual Error", multiplier: 1.5 },
  { id: 2, name: "Terminology Error", multiplier: 1.5 },
  { id: 3, name: "Logical Gap", multiplier: 1.5 },
  { id: 4, name: "Performance Error", multiplier: 1.5 },
  { id: 5, name: "Off-by-One Error", multiplier: 1.0 },
  { id: 6, name: "Edge Case Error", multiplier: 1.0 },
  { id: 7, name: "Careless Error", multiplier: 1.0 },
  { id: 8, name: "Implementation Error", multiplier: 1.0 },
  { id: 9, name: "Unoptimized", multiplier: 0.7 },
  { id: 10, name: "Over Time Limit", multiplier: 0.7 },
];

export const GRID_BREAKPOINTS = {
  lg: 1200,
  md: 996,
  sm: 768,
  xs: 480,
  xxs: 0,
};

export const CARD_VARIANTS = {
  default: "bg-slate-900/50 border-slate-700",
  success: "bg-emerald-950/30 border-emerald-700/50",
  warning: "bg-amber-950/30 border-amber-700/50",
  error: "bg-red-950/30 border-red-700/50",
};
