import React from 'react';
import { WindowsVariant } from './components/WindowsVariant';
import { MacOSVariant } from './components/MacOSVariant';
import { LinuxVariant } from './components/LinuxVariant';

export default function App() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 via-slate-100 to-slate-200 p-16">
      <div className="max-w-7xl mx-auto">
        <div className="text-center mb-12">
          <h1 className="text-slate-800 mb-2">
            VietFlux – Cross-Platform Tray Popover
          </h1>
          <p className="text-slate-500 text-sm">
            400×600px • Production-ready UI for Windows, macOS, and Linux
          </p>
        </div>
        
        <div className="grid grid-cols-3 gap-10">
          {/* Windows Variant */}
          <div className="flex flex-col items-center gap-4">
            <div className="text-center">
              <h2 className="text-slate-700">Windows</h2>
              <p className="text-xs text-slate-500 mt-0.5">Fluent Design</p>
            </div>
            <WindowsVariant />
          </div>

          {/* macOS Variant */}
          <div className="flex flex-col items-center gap-4">
            <div className="text-center">
              <h2 className="text-slate-700">macOS</h2>
              <p className="text-xs text-slate-500 mt-0.5">Native Popover</p>
            </div>
            <MacOSVariant />
          </div>

          {/* Linux Variant */}
          <div className="flex flex-col items-center gap-4">
            <div className="text-center">
              <h2 className="text-slate-700">Linux</h2>
              <p className="text-xs text-slate-500 mt-0.5">GNOME Style</p>
            </div>
            <LinuxVariant />
          </div>
        </div>
      </div>
    </div>
  );
}