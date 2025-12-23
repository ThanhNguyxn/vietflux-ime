import React from 'react';

interface PopoverContainerProps {
  variant: 'windows' | 'macos' | 'linux';
  children: React.ReactNode;
}

export function PopoverContainer({ variant, children }: PopoverContainerProps) {
  const variantStyles = {
    windows: 'bg-white shadow-[0_8px_32px_rgba(0,0,0,0.08),0_2px_8px_rgba(0,0,0,0.04)] rounded-xl border border-slate-200/60',
    macos: 'bg-white shadow-[0_16px_48px_rgba(0,0,0,0.1),0_4px_16px_rgba(0,0,0,0.06)] rounded-[16px] border border-slate-200/50',
    linux: 'bg-white shadow-[0_4px_24px_rgba(0,0,0,0.1),0_1px_4px_rgba(0,0,0,0.06)] rounded-xl border border-slate-200/70',
  };

  const fontStyles = {
    windows: 'font-[system-ui,Segoe_UI,sans-serif]',
    macos: 'font-[-apple-system,BlinkMacSystemFont,SF_Pro_Text,sans-serif]',
    linux: 'font-[system-ui,Ubuntu,Cantarell,sans-serif]',
  };

  return (
    <div className="w-[400px] h-[600px] flex items-center justify-center">
      <div className={`w-full h-full ${variantStyles[variant]} ${fontStyles[variant]} overflow-hidden flex flex-col`}>
        {children}
      </div>
    </div>
  );
}