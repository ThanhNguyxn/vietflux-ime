import React from 'react';

interface PrimarySwitchProps {
  variant: 'windows' | 'macos' | 'linux';
  enabled: boolean;
  onChange: (enabled: boolean) => void;
}

export function PrimarySwitch({ variant, enabled, onChange }: PrimarySwitchProps) {
  const switchSize = variant === 'macos' ? 'w-[52px] h-[30px]' : 'w-12 h-[26px]';
  const knobSize = variant === 'macos' ? 'w-[26px] h-[26px]' : 'w-[22px] h-[22px]';
  const knobTranslate = variant === 'macos' ? 'translate-x-[22px]' : 'translate-x-[22px]';

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

  return (
    <div className="flex items-center justify-between gap-4">
      <div className="flex-1 min-w-0">
        <div className="text-[13px] font-medium text-slate-700 leading-tight tracking-[-0.005em]">
          Enable VietFlux
        </div>
        <p className="mt-1 text-[11px] text-slate-500 leading-[1.5] tracking-normal">
          Typing Vietnamese system-wide
        </p>
      </div>
      
      <button
        role="switch"
        aria-checked={enabled}
        onClick={() => onChange(!enabled)}
        className={`
          relative inline-flex items-center flex-shrink-0 ${switchSize} 
          ${variant === 'windows' ? 'rounded-full' : variant === 'macos' ? 'rounded-full' : 'rounded-full'}
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
            ${knobSize} rounded-full bg-white shadow-sm
            transform transition-transform duration-200 ease-out
            ${enabled ? knobTranslate : 'translate-x-1'}
          `}
        />
      </button>
    </div>
  );
}