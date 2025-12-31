import React from "react";
import { X } from "lucide-react";
import { cn } from "@/lib/utils";

interface BaseCardProps {
  id: string;
  title: string;
  children: React.ReactNode;
  className?: string;
  onClose?: () => void;
  headerAction?: React.ReactNode;
  variant?: "default" | "success" | "warning" | "error";
  isLoading?: boolean;
}

export const BaseCard: React.FC<BaseCardProps> = ({
  id,
  title,
  children,
  className,
  onClose,
  headerAction,
  variant = "default",
  isLoading = false,
}) => {
  const variantStyles = {
    default: "bg-slate-900/50 border-slate-700",
    success: "bg-emerald-950/30 border-emerald-700/50",
    warning: "bg-amber-950/30 border-amber-700/50",
    error: "bg-red-950/30 border-red-700/50",
  };

  return (
    <div
      className={cn(
        "rounded-lg border backdrop-blur-sm transition-all duration-200",
        "flex flex-col h-full overflow-hidden",
        variantStyles[variant],
        className
      )}
    >
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-slate-700/50">
        <h3 className="text-sm font-semibold text-slate-100">{title}</h3>
        <div className="flex items-center gap-2">
          {headerAction}
          {onClose && (
            <button
              onClick={onClose}
              className="p-1 hover:bg-slate-700/50 rounded transition-colors"
            >
              <X size={16} className="text-slate-400" />
            </button>
          )}
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-auto p-4">
        {isLoading ? (
          <div className="flex items-center justify-center h-full">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500" />
          </div>
        ) : (
          children
        )}
      </div>
    </div>
  );
};
