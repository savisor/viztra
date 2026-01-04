/**
 * Symbol and Timeframe selection provider
 * Manages global state for selected symbol and timeframe
 */

import { createContext, useContext, useState, useCallback, ReactNode } from "react";

interface SymbolTimeframeContextValue {
  symbol: string;
  timeframe: string;
  setSymbol: (symbol: string) => void;
  setTimeframe: (timeframe: string) => void;
  setSymbolAndTimeframe: (symbol: string, timeframe: string) => void;
}

const SymbolTimeframeContext = createContext<SymbolTimeframeContextValue | undefined>(undefined);

interface SymbolTimeframeProviderProps {
  children: ReactNode;
  defaultSymbol?: string;
  defaultTimeframe?: string;
}

export function SymbolTimeframeProvider({
  children,
  defaultSymbol = "USDJPY",
  defaultTimeframe = "1M",
}: SymbolTimeframeProviderProps) {
  const [symbol, setSymbolState] = useState<string>(defaultSymbol);
  const [timeframe, setTimeframeState] = useState<string>(defaultTimeframe);

  const setSymbol = useCallback((newSymbol: string) => {
    setSymbolState(newSymbol);
  }, []);

  const setTimeframe = useCallback((newTimeframe: string) => {
    setTimeframeState(newTimeframe);
  }, []);

  const setSymbolAndTimeframe = useCallback((newSymbol: string, newTimeframe: string) => {
    setSymbolState(newSymbol);
    setTimeframeState(newTimeframe);
  }, []);

  const value: SymbolTimeframeContextValue = {
    symbol,
    timeframe,
    setSymbol,
    setTimeframe,
    setSymbolAndTimeframe,
  };

  return (
    <SymbolTimeframeContext.Provider value={value}>
      {children}
    </SymbolTimeframeContext.Provider>
  );
}

export function useSymbolTimeframeContext() {
  const context = useContext(SymbolTimeframeContext);
  if (context === undefined) {
    throw new Error("useSymbolTimeframeContext must be used within a SymbolTimeframeProvider");
  }
  return context;
}
