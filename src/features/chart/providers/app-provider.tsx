/**
 * Main app provider component
 * Wraps chart-specific providers
 */

import React from "react";
import { SymbolTimeframeProvider } from "./symbol-timeframe-provider";

interface AppProviderProps {
  children: React.ReactNode;
}

export function AppProvider({ children }: AppProviderProps) {
  return (
    <SymbolTimeframeProvider>
      {children}
    </SymbolTimeframeProvider>
  );
}
