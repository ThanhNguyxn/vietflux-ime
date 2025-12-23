import React from 'react';

interface SegmentedControlProps {
  variant: 'windows' | 'macos' | 'linux';
  options: string[];
  value: string;
  onChange: (value: string) => void;
}

export function SegmentedControl({ variant, options, value, onChange }: SegmentedControlProps) {
  const containerStyles = {
    windows: 'flex items-center gap-0 bg-slate-100/60 p-0.5 rounded-lg',
    macos: 'flex items-center gap-0 bg-slate-100/40 p-0.5 rounded-[14px]',
    linux: 'flex items-center gap-0 bg-slate-100/70 p-0.5 rounded-lg',
  };

  const buttonStyles = {
    windows: (isSelected: boolean) => `
      px-3 py-1.5 text-[13px] rounded-md transition-all duration-150 font-medium leading-tight tracking-[-0.005em]
      focus:outline-none focus:ring-2 focus:ring-slate-300/60 focus:ring-offset-0
      ${isSelected 
        ? 'bg-white text-slate-700 shadow-sm' 
        : 'text-slate-500 hover:text-slate-700'
      }
    `,
    macos: (isSelected: boolean) => `
      px-3 py-1.5 text-[13px] rounded-[11px] transition-all duration-150 leading-tight tracking-[-0.005em]
      focus:outline-none focus:ring-2 focus:ring-slate-300/50 focus:ring-offset-0
      ${isSelected 
        ? 'bg-white text-slate-700 shadow-sm' 
        : 'text-slate-500 hover:text-slate-700'
      }
    `,
    linux: (isSelected: boolean) => `
      px-3 py-1.5 text-[13px] rounded-md transition-all duration-150 font-medium leading-tight tracking-[-0.005em]
      focus:outline-none focus:ring-2 focus:ring-slate-300/70 focus:ring-offset-0
      ${isSelected 
        ? 'bg-white text-slate-700 shadow-sm' 
        : 'text-slate-500 hover:text-slate-700'
      }
    `,
  };

  return (
    <div className={`inline-flex w-full ${containerStyles[variant]}`} role="group">
      {options.map((option) => (
        <button
          key={option}
          onClick={() => onChange(option)}
          className={`flex-1 ${buttonStyles[variant](value === option)}`}
        >
          {option}
        </button>
      ))}
    </div>
  );
}