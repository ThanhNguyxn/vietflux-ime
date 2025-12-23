import React from 'react';

interface KeycapChipProps {
  variant: 'windows' | 'macos' | 'linux';
  keyLabel: string;
}

export function KeycapChip({ variant, keyLabel }: KeycapChipProps) {
  const chipStyles = {
    windows: 'bg-slate-100/80 border border-slate-200/60 shadow-sm rounded-md px-1.5 py-0.5 text-[10px] text-slate-600 min-w-[20px] text-center leading-none font-medium tracking-wide',
    macos: 'bg-slate-100/60 border border-slate-200/40 shadow-sm rounded-[5px] px-1.5 py-0.5 text-[10px] text-slate-600 min-w-[20px] text-center leading-none font-medium tracking-wide',
    linux: 'bg-slate-100/80 border border-slate-200/60 shadow-sm rounded-md px-1.5 py-0.5 text-[10px] text-slate-600 min-w-[20px] text-center leading-none font-medium tracking-wide',
  };

  return (
    <kbd className={chipStyles[variant]}>
      {keyLabel}
    </kbd>
  );
}