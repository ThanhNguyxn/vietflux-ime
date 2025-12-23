import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { AppHeader } from './AppHeader';
import { PrimarySwitch } from './PrimarySwitch';
import { SegmentedControl } from './SegmentedControl';
import { ToggleRow } from './ToggleRow';
import { ShortcutsCard } from './ShortcutsCard';
import { FooterLinks } from './FooterLinks';

export function WindowsVariant() {
  const [isEnabled, setIsEnabled] = useState(true);
  const [inputMethod, setInputMethod] = useState('Telex');
  const [autoCapitalize, setAutoCapitalize] = useState(true);
  const [smartQuotes, setSmartQuotes] = useState(false);
  const [spellCheck, setSpellCheck] = useState(true);

  // Fetch initial state
  useEffect(() => {
    async function fetchState() {
      try {
        const enabled = await invoke<boolean>('is_enabled');
        setIsEnabled(enabled);

        const method = await invoke<string>('get_method');
        const methodMap: Record<string, string> = {
          'telex': 'Telex',
          'vni': 'VNI',
          'auto': 'Auto'
        };
        setInputMethod(methodMap[method] || 'Telex');

        const [autoCap, smartQ, spell] = await invoke<[boolean, boolean, boolean]>('get_options');
        setAutoCapitalize(autoCap);
        setSmartQuotes(smartQ);
        setSpellCheck(spell);
      } catch (e) {
        console.error('Failed to fetch state:', e);
      }
    }
    fetchState();
  }, []);

  const handleEnabledChange = async (enabled: boolean) => {
    setIsEnabled(enabled);
    await invoke('toggle');
  };

  const handleMethodChange = async (method: string) => {
    setInputMethod(method);
    await invoke('set_method', { method: method.toLowerCase() });
  };

  const updateOptions = async (autoCap: boolean, smartQ: boolean, spell: boolean) => {
    await invoke('set_options', {
      autoCapitalize: autoCap,
      smartQuotes: smartQ,
      spellCheck: spell
    });
  };

  const handleAutoCapChange = (val: boolean) => {
    setAutoCapitalize(val);
    updateOptions(val, smartQuotes, spellCheck);
  };

  const handleSmartQuotesChange = (val: boolean) => {
    setSmartQuotes(val);
    updateOptions(autoCapitalize, val, spellCheck);
  };

  const handleSpellCheckChange = (val: boolean) => {
    setSpellCheck(val);
    updateOptions(autoCapitalize, smartQuotes, val);
  };

  return (
    <div className="flex flex-col h-full bg-white">
      {/* Header */}
      <AppHeader variant="windows" isEnabled={isEnabled} />

      {/* Scrollable Content */}
      <div className="flex-1 overflow-y-auto min-h-0">
        {/* Enable VietFlux */}
        <div className="px-4 py-3 border-b border-slate-200/40">
          <PrimarySwitch
            variant="windows"
            enabled={isEnabled}
            onChange={handleEnabledChange}
          />
        </div>

        {/* Input Method */}
        <div className="px-4 py-3 border-b border-slate-200/40">
          <label className="block mb-2 text-[13px] text-slate-700 font-medium leading-tight tracking-[-0.005em]">
            Input Method
          </label>
          <SegmentedControl
            variant="windows"
            options={['Telex', 'VNI', 'Auto']}
            value={inputMethod}
            onChange={handleMethodChange}
          />
          <p className="mt-2 text-[11px] text-slate-500 leading-[1.5] tracking-normal">
            Auto learns from your recent usage
          </p>
        </div>

        {/* Quick Toggles */}
        <div className="px-4 py-3 border-b border-slate-200/40">
          <h3 className="text-[13px] text-slate-700 mb-2 font-medium leading-tight tracking-[-0.005em]">
            Smart Typing
          </h3>
          <div className="space-y-0">
            <ToggleRow
              variant="windows"
              label="Auto-capitalize"
              description="First letter after punctuation"
              enabled={autoCapitalize}
              onChange={handleAutoCapChange}
            />
            <ToggleRow
              variant="windows"
              label="Smart quotes"
              description="Convert to curly quotes"
              enabled={smartQuotes}
              onChange={handleSmartQuotesChange}
            />
            <ToggleRow
              variant="windows"
              label="Spell check"
              description="Underline misspelled words"
              enabled={spellCheck}
              onChange={handleSpellCheckChange}
            />
          </div>
        </div>

        {/* Keyboard Shortcuts */}
        <div className="px-4 py-3">
          <ShortcutsCard variant="windows" />
        </div>
      </div>

      {/* Footer */}
      <FooterLinks variant="windows" />
    </div>
  );
}