
import { invoke } from '@tauri-apps/api/core';
import { LogOut, Minimize2 } from 'lucide-react';

interface FooterLinksProps {
  variant: 'windows' | 'macos' | 'linux';
}

// Auto-update version from build time or package.json
const APP_VERSION = typeof import.meta !== 'undefined' && import.meta.env?.VITE_APP_VERSION
  ? import.meta.env.VITE_APP_VERSION
  : '0.1.0';

export function FooterLinks({ variant }: FooterLinksProps) {
  const containerStyles = {
    windows: 'bg-white/60 backdrop-blur-xl border-t border-slate-200/40',
    macos: 'bg-white/40 border-t border-slate-200/20 backdrop-blur-2xl',
    linux: 'bg-white border-t border-slate-200/50',
  };

  const linkStyles = {
    windows: 'text-[10px] text-slate-500 hover:text-slate-700 focus:outline-none focus:ring-2 focus:ring-slate-300/60 focus:ring-offset-0 rounded px-0.5 -mx-0.5 transition-colors duration-150 font-medium tracking-wide',
    macos: 'text-[10px] text-slate-500 hover:text-slate-700 focus:outline-none focus:ring-2 focus:ring-slate-300/50 focus:ring-offset-0 rounded-md px-0.5 -mx-0.5 transition-colors duration-150 tracking-wide',
    linux: 'text-[10px] text-slate-500 hover:text-slate-700 focus:outline-none focus:ring-2 focus:ring-slate-300/70 focus:ring-offset-0 rounded px-0.5 -mx-0.5 transition-colors duration-150 font-medium tracking-wide',
  };

  const separatorColor = 'text-slate-300';

  const openLink = (url: string) => {
    window.open(url, '_blank', 'noopener,noreferrer');
  };

  const handleQuit = async () => {
    await invoke('quit_app');
  };

  const handleHide = async () => {
    await invoke('hide_window');
  };

  return (
    <div className={`px-4 py-3 ${containerStyles[variant]} flex-shrink-0`}>
      {/* Top row: Author & Donate */}
      <div className="flex items-center gap-1.5 mb-1.5">
        <button
          className={linkStyles[variant]}
          onClick={() => openLink('https://github.com/ThanhNguyxn')}
          title="Author"
        >
          Author
        </button>
        <span className={`${separatorColor} select-none text-[10px]`}>•</span>
        <button
          className={linkStyles[variant]}
          onClick={() => openLink('https://github.com/sponsors/ThanhNguyxn')}
          title="GitHub Sponsors"
        >
          Sponsor
        </button>
        <span className={`${separatorColor} select-none text-[10px]`}>•</span>
        <button
          className={linkStyles[variant]}
          onClick={() => openLink('https://buymeacoffee.com/thanhnguyxn')}
          title="Buy Me a Coffee"
        >
          Donate
        </button>
      </div>

      {/* Middle row: Source & Version */}
      <div className="flex items-center justify-between mb-3">
        <button
          className={linkStyles[variant]}
          onClick={() => openLink('https://github.com/ThanhNguyxn/vietflux-ime')}
          title="Source code on GitHub"
        >
          Source code
        </button>

        <span className="text-[10px] text-slate-400 tabular-nums font-medium tracking-wide">
          v{APP_VERSION}
        </span>
      </div>

      {/* Bottom row: App Controls */}
      <div className="flex items-center justify-end gap-2 pt-2 border-t border-slate-200/30">
        <button
          onClick={handleHide}
          className="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-slate-600 bg-slate-100 hover:bg-slate-200 rounded-md transition-colors"
        >
          <Minimize2 size={14} />
          Ẩn xuống khay
        </button>
        <button
          onClick={handleQuit}
          className="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-red-600 bg-red-50 hover:bg-red-100 rounded-md transition-colors"
        >
          <LogOut size={14} />
          Kết thúc
        </button>
      </div>
    </div>
  );
}