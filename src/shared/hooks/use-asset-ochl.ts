import { useState, useEffect, useCallback } from "react";
import { invokeCommand } from "@/shared/lib/tauri";
import type { OHLCData } from "@/types";
import { useSymbolTimeframe } from "./use-symbol-timeframe";

/**
 * Hook for retrieving OHLC data for a symbol and timeframe
 * Reads symbol and timeframe from the SymbolTimeframeProvider context
 * @param autoFetch - If true, automatically fetches data when symbol or timeframe changes
 * @returns Object containing OHLC data array, loading state, error state, and fetch function
 */
interface UseAssetOHLCReturn {
  data: OHLCData[];
  isLoading: boolean;
  error: string | null;
  fetchOHLC: (overrideSymbol?: string, overrideTimeframe?: string) => Promise<void>;
  symbol: string;
  timeframe: string;
}

export function useAssetOHLC(autoFetch: boolean = false): UseAssetOHLCReturn {
  const { symbol, timeframe } = useSymbolTimeframe();
  const [data, setData] = useState<OHLCData[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchOHLC = useCallback(async (overrideSymbol?: string, overrideTimeframe?: string): Promise<void> => {
    const targetSymbol = overrideSymbol ?? symbol;
    const targetTimeframe = overrideTimeframe ?? timeframe;

    if (!targetSymbol.trim() || !targetTimeframe.trim()) {
      setError("Symbol and timeframe are required");
      return;
    }

    setIsLoading(true);
    setError(null);
    setData([]);

    try {
      const result = await invokeCommand<OHLCData[]>("retrieve_asset_ochl", {
        symbol: targetSymbol.trim(),
        timeframe: targetTimeframe.trim(),
      });
      setData(result);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "Failed to retrieve OHLC data";
      setError(errorMessage);
      setData([]);
    } finally {
      setIsLoading(false);
    }
  }, [symbol, timeframe]);

  // Auto-fetch when symbol or timeframe changes if autoFetch is enabled
  useEffect(() => {
    if (autoFetch && symbol && timeframe) {
      fetchOHLC();
    }
  }, [symbol, timeframe, autoFetch, fetchOHLC]);

  return {
    data,
    isLoading,
    error,
    fetchOHLC,
    symbol,
    timeframe,
  };
}

