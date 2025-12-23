import React from 'react';

interface ToggleRowProps {
  variant: 'windows' | 'macos' | 'linux';
  label: string;
  description: string;
  enabled: boolean;
  onChange: (enabled: boolean) => void;
}

export function ToggleRow({ variant, label, description, enabled, onChange }: ToggleRowProps) {
  const switchStyles = {
    windows: enabled 
      ? 'bg-blue-500 border border-blue-500/20' 
      : 'bg-slate-200 border border-slate-200/20',
    macos: enabled 
      ? 'bg-blue-500' 
      : 'bg-slate-200',
    linux: enabled 
      ? 'bg-blue-500 border border-blue-500/20' 
      : 'bg-slate-200 border border-slate-200/20',
  };

  const hoverStyles = {
    windows: 'hover:bg-slate-50/40 rounded-lg -mx-2 px-2 transition-colors duration-150',
    macos: 'hover:bg-slate-50/30 rounded-[10px] -mx-2 px-2 transition-colors duration-150',
    linux: 'hover:bg-slate-50/50 rounded-lg -mx-2 px-2 transition-colors duration-150',
  };

  return (
    <div className={`flex items-start justify-between gap-4 py-2 ${hoverStyles[variant]}`}>
      <div className="flex-1 min-w-0 pt-0.5">
        <div className="text-[13px] text-slate-700 font-medium leading-tight tracking-[-0.005em]">
          {label}
        </div>
        <p className="mt-0.5 text-[11px] text-slate-500 leading-[1.5] tracking-normal">
          {description}
        </p>
      </div>
      
      <button
        role="switch"
        aria-checked={enabled}
        aria-label={label}
        onClick={() => onChange(!enabled)}
        className={`
          relative inline-flex items-center flex-shrink-0
          ${variant === 'macos' ? 'w-[42px] h-[24px]' : 'w-10 h-[22px]'}
          rounded-full
          transition-all duration-200 ease-out
          focus:outline-none focus:ring-2 focus:ring-offset-0
          ${variant === 'windows' ? 'focus:ring-slate-300/60' : ''}
          ${variant === 'macos' ? 'focus:ring-slate-300/50' : ''}
          ${variant === 'linux' ? 'focus:ring-slate-300/70' : ''}
          ${switchStyles[variant]}
        `}
      >
        <span
          className={`
            ${variant === 'macos' ? 'w-5 h-5' : 'w-4 h-4'}
            rounded-full bg-white shadow-sm
            transform transition-transform duration-200 ease-out
            ${enabled ? (variant === 'macos' ? 'translate-x-[18px]' : 'translate-x-[18px]') : 'translate-x-0.5'}
          `}
        />
      </button>
    </div>
  );
}