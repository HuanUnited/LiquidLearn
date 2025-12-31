import React, { ReactNode } from 'react';
import { ChevronDown } from 'lucide-react';

interface BaseCardProps {
  title: string;
  subtitle?: string;
  icon?: ReactNode;
  children: ReactNode;
  onClick?: () => void;
  isExpandable?: boolean;
  isExpanded?: boolean;
  onToggleExpand?: () => void;
  className?: string;
  headerClassName?: string;
  bodyClassName?: string;
  variant?: 'default' | 'success' | 'warning' | 'danger';
}

export const BaseCard: React.FC<BaseCardProps> = ({
  title,
  subtitle,
  icon,
  children,
  onClick,
  isExpandable = false,
  isExpanded = true,
  onToggleExpand,
  className = '',
  headerClassName = '',
  bodyClassName = '',
  variant = 'default',
}) => {
  const variantClasses = {
    default: 'bg-white dark:bg-slate-800 border-slate-200 dark:border-slate-700',
    success: 'bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-700',
    warning: 'bg-yellow-50 dark:bg-yellow-900/20 border-yellow-200 dark:border-yellow-700',
    danger: 'bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-700',
  };

  return (
    <div
      className={`card-base ${variantClasses[variant]} ${className}`}
      onClick={onClick}
      role={onClick ? 'button' : undefined}
      tabIndex={onClick ? 0 : undefined}
    >
      {/* Header */}
      <div
        className={`card-header flex items-center justify-between cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-700/50 transition-colors ${headerClassName}`}
        onClick={() => isExpandable && onToggleExpand?.()}
      >
        <div className="flex items-center gap-3">
          {icon && <div className="text-xl">{icon}</div>}
          <div>
            <h2>{title}</h2>
            {subtitle && (
              <p className="text-sm text-slate-500 dark:text-slate-400 mt-1">
                {subtitle}
              </p>
            )}
          </div>
        </div>

        {isExpandable && (
          <ChevronDown
            size={20}
            className={`text-slate-600 dark:text-slate-400 transition-transform duration-200 ${isExpanded ? 'rotate-180' : ''
              }`}
          />
        )}
      </div>

      {/* Body */}
      {(!isExpandable || isExpanded) && (
        <div className={`card-body ${bodyClassName}`}>{children}</div>
      )}
    </div>
  );
};
