import React from 'react';
import { KeycapChip } from './KeycapChip';

interface ShortcutsCardProps {
  variant: 'windows' | 'macos' | 'linux';
}

const shortcuts = [
  { keys: ['Ctrl', 'Shift', 'V'], action: 'Toggle VietFlux' },
  { keys: ['Ctrl', ','], action: 'Settings' },
  { keys: ['Ctrl', 'Alt', 'M'], action: 'Cycle method' },
];

export function ShortcutsCard({ variant }: ShortcutsCardProps) {
  const cardStyles = {
    windows: 'bg-slate-50/50 border border-slate-200/40 rounded-xl p-3',
    macos: 'bg-slate-50/30 border border-slate-200/30 rounded-[14px] p-3',
    linux: 'bg-slate-50/60 border border-slate-200/50 rounded-xl p-3',
  };

  const linkStyles = {
    windows: 'text-[11px] text-slate-500 hover:text-slate-700 focus:outline-none focus:ring-2 focus:ring-slate-300/60 focus:ring-offset-0 rounded-md px-1 py-0.5 -mx-1 -my-0.5 transition-colors duration-150 font-medium',
    macos: 'text-[11px] text-slate-500 hover:text-slate-700 focus:outline-none focus:ring-2 focus:ring-slate-300/50 focus:ring-offset-0 rounded-lg px-1 py-0.5 -mx-1 -my-0.5 transition-colors duration-150',
    linux: 'text-[11px] text-slate-500 hover:text-slate-700 focus:outline-none focus:ring-2 focus:ring-slate-300/70 focus:ring-offset-0 rounded-md px-1 py-0.5 -mx-1 -my-0.5 transition-colors duration-150 font-medium',
  };

  return (
    <div className={cardStyles[variant]}>
      <div className="flex items-center justify-between mb-3">
        <h3 className="text-[13px] text-slate-700 font-medium leading-tight tracking-[-0.005em]">Keyboard Shortcuts</h3>
        <button className={linkStyles[variant]}>
          Customize
        </button>
      </div>
      
      <div className="space-y-2">
        <ShortcutRow
          variant={variant}
          label="Toggle Vietnamese"
          keys={['⌃', 'Space']}
        />
        <ShortcutRow
          variant={variant}
          label="Switch input method"
          keys={['⌃', '⇧', 'V']}
        />
        <ShortcutRow
          variant={variant}
          label="Settings"
          keys={['⌃', ',']}
        />
      </div>
    </div>
  );
}

interface ShortcutRowProps {
  variant: 'windows' | 'macos' | 'linux';
  label: string;
  keys: string[];
}

function ShortcutRow({ variant, label, keys }: ShortcutRowProps) {
  return (
    <div className="flex items-center justify-between gap-3">
      <span className="text-[11px] text-slate-500 flex-1 leading-tight tracking-normal">
        {label}
      </span>
      <div className="flex items-center gap-0.5 flex-shrink-0">
        {keys.map((key, keyIndex) => (
          <React.Fragment key={keyIndex}>
            <KeycapChip variant={variant} keyLabel={key} />
            {keyIndex < keys.length - 1 && (
              <span className="text-[10px] text-slate-400 font-medium mx-0.5">+</span>
            )}
          </React.Fragment>
        ))}
      </div>
    </div>
  );
}