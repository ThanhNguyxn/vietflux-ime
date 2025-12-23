import React from 'react';
import { Settings, Keyboard } from 'lucide-react';
import { StatusPill } from './StatusPill';

interface AppHeaderProps {
  variant: 'windows' | 'macos' | 'linux';
  isEnabled: boolean;
}

export function AppHeader({ variant, isEnabled }: AppHeaderProps) {
  const iconSize = 16;
  
  const borderClass = variant === 'windows' 
    ? 'border-b border-slate-200/50 bg-white/70 backdrop-blur-xl' 
    : variant === 'linux'
    ? 'border-b border-slate-200/60 bg-white'
    : 'border-b border-slate-200/30 bg-white/50 backdrop-blur-2xl';

  const iconContainerStyles = {
    windows: 'w-8 h-8 rounded-lg bg-gradient-to-br from-blue-500 to-blue-600 shadow-sm flex items-center justify-center',
    macos: 'w-8 h-8 rounded-[10px] bg-gradient-to-br from-blue-500 to-blue-600 shadow-sm flex items-center justify-center',
    linux: 'w-8 h-8 rounded-lg bg-gradient-to-br from-blue-500 to-blue-600 shadow-sm flex items-center justify-center',
  };

  const iconButtonStyles = {
    windows: 'w-8 h-8 rounded-lg hover:bg-slate-100/50 active:bg-slate-100 transition-colors duration-100 flex items-center justify-center',
    macos: 'w-8 h-8 rounded-[10px] hover:bg-slate-100/40 active:bg-slate-100/70 transition-colors duration-150 flex items-center justify-center',
    linux: 'w-8 h-8 rounded-lg hover:bg-slate-100/60 active:bg-slate-100 transition-colors duration-100 flex items-center justify-center',
  };

  const statusStyles = variant === 'windows' 
    ? 'bg-emerald-50/80 text-emerald-700 border border-emerald-200/40' 
    : variant === 'macos'
    ? 'bg-emerald-50/60 text-emerald-700 border border-emerald-200/20'
    : 'bg-emerald-50/70 text-emerald-700 border border-emerald-200/40';

  return (
    <div className={`px-4 py-3 ${borderClass} flex items-center justify-between flex-shrink-0`}>
      <div className="flex items-center gap-3">
        <div className={iconContainerStyles[variant]}>
          <Keyboard size={iconSize} className="text-white" strokeWidth={2} />
        </div>
        <div className="flex items-baseline gap-2">
          <h1 className="text-[15px] font-semibold text-slate-800 leading-none tracking-[-0.01em]">
            VietFlux
          </h1>
          {isEnabled && (
            <span className={`text-[10px] px-2 py-0.5 rounded-full font-medium leading-none tracking-wide ${statusStyles}`}>
              ACTIVE
            </span>
          )}
        </div>
      </div>
      
      <button 
        className={`${iconButtonStyles[variant]} focus:outline-none focus:ring-2 ${variant === 'windows' ? 'focus:ring-slate-300/60' : variant === 'macos' ? 'focus:ring-slate-300/50' : 'focus:ring-slate-300/70'} focus:ring-offset-0`}
        aria-label="Settings"
      >
        <Settings size={16} className="text-slate-500" strokeWidth={2} />
      </button>
    </div>
  );
}