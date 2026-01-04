/**
 * Hook to access symbol and timeframe from the context
 * @returns Object containing symbol, timeframe, and setter functions
 */

import { useSymbolTimeframeContext } from "../providers/symbol-timeframe-provider";

export function useSymbolTimeframe() {
  return useSymbolTimeframeContext();
}
