/**
 * Main app provider component
 * Add global providers here (theme, auth, etc.)
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

