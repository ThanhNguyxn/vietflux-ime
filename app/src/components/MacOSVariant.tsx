import React, { useState } from 'react';
import { AppHeader } from './AppHeader';
import { PrimarySwitch } from './PrimarySwitch';
import { SegmentedControl } from './SegmentedControl';
import { ToggleRow } from './ToggleRow';
import { ShortcutsCard } from './ShortcutsCard';
import { FooterLinks } from './FooterLinks';

export function MacOSVariant() {
  const [isEnabled, setIsEnabled] = useState(true);
  const [inputMethod, setInputMethod] = useState('Telex');
  const [autoCapitalize, setAutoCapitalize] = useState(true);
  const [smartQuotes, setSmartQuotes] = useState(false);
  const [spellCheck, setSpellCheck] = useState(true);

  return (
    <div className="flex flex-col h-full bg-white">
      {/* Header */}
      <AppHeader variant="macos" isEnabled={isEnabled} />

      {/* Scrollable Content */}
      <div className="flex-1 overflow-y-auto min-h-0">
        {/* Enable VietFlux */}
        <div className="px-4 py-3">
          <PrimarySwitch
            variant="macos"
            enabled={isEnabled}
            onChange={setIsEnabled}
          />
        </div>

        {/* Separator */}
        <div className="h-px bg-slate-200/30 mx-4" />

        {/* Input Method */}
        <div className="px-4 py-3">
          <label className="block mb-2 text-[13px] text-slate-700 font-medium leading-tight tracking-[-0.005em]">
            Input Method
          </label>
          <SegmentedControl
            variant="macos"
            options={['Telex', 'VNI', 'Auto']}
            value={inputMethod}
            onChange={setInputMethod}
          />
          <p className="mt-2 text-[11px] text-slate-500 leading-[1.5] tracking-normal">
            Auto learns from your recent usage
          </p>
        </div>

        {/* Separator */}
        <div className="h-px bg-slate-200/30 mx-4" />

        {/* Quick Toggles */}
        <div className="px-4 py-3">
          <h3 className="text-[13px] text-slate-700 mb-2 font-medium leading-tight tracking-[-0.005em]">
            Smart Typing
          </h3>
          <div className="space-y-0">
            <ToggleRow
              variant="macos"
              label="Auto-capitalize"
              description="First letter after punctuation"
              enabled={autoCapitalize}
              onChange={setAutoCapitalize}
            />
            <ToggleRow
              variant="macos"
              label="Smart quotes"
              description="Convert to curly quotes"
              enabled={smartQuotes}
              onChange={setSmartQuotes}
            />
            <ToggleRow
              variant="macos"
              label="Spell check"
              description="Underline misspelled words"
              enabled={spellCheck}
              onChange={setSpellCheck}
            />
          </div>
        </div>

        {/* Separator */}
        <div className="h-px bg-slate-200/30 mx-4" />

        {/* Keyboard Shortcuts */}
        <div className="px-4 py-3">
          <ShortcutsCard variant="macos" />
        </div>
      </div>

      {/* Footer */}
      <FooterLinks variant="macos" />
    </div>
  );
}