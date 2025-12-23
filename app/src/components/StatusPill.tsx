import React from 'react';

interface StatusPillProps {
  variant: 'windows' | 'macos' | 'linux';
  isEnabled: boolean;
}

export function StatusPill({ variant, isEnabled }: StatusPillProps) {
  const baseStyles = 'px-1.5 py-[3px] text-[10px] uppercase tracking-wide transition-colors';
  
  const variantStyles = {
    windows: isEnabled 
      ? 'bg-emerald-500/12 text-emerald-700 border border-emerald-500/25 rounded-full'
      : 'bg-slate-400/12 text-slate-600 border border-slate-400/25 rounded-full',
    macos: isEnabled
      ? 'bg-emerald-500/12 text-emerald-700 rounded-full'
      : 'bg-slate-400/12 text-slate-600 rounded-full',
    linux: isEnabled
      ? 'bg-emerald-500/15 text-emerald-800 border border-emerald-600/30 rounded-full'
      : 'bg-slate-400/15 text-slate-700 border border-slate-500/30 rounded-full',
  };

  return (
    <span className={`${baseStyles} ${variantStyles[variant]}`}>
      {isEnabled ? 'ON' : 'OFF'}
    </span>
  );
}