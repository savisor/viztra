/**
 * Generic hook for executing insights
 */

import { useState, useEffect, useCallback } from "react";
import { invokeCommand } from "@/shared/services/tauri";
import type { InsightRequest, InsightResponse } from "./types";

export interface UseInsightOptions {
  autoFetch?: boolean;
}

export interface UseInsightReturn<TResult = Record<string, unknown>> {
  data: TResult[] | null;
  isLoading: boolean;
  error: string | null;
  columns: string[];
  execute: () => Promise<void>;
}

/**
 * Hook for executing insights with loading, error, and data state management
 * 
 * @param insightId - The unique identifier of the insight to execute
 * @param parameters - The parameters for the insight
 * @param options - Optional configuration (autoFetch)
 * @returns Object containing data, loading state, error state, columns, and execute function
 */
export function useInsight<TParams = Record<string, unknown>, TResult = Record<string, unknown>>(
  insightId: string,
  parameters: TParams,
  options: UseInsightOptions = {}
): UseInsightReturn<TResult> {
  const { autoFetch = false } = options;

  const [data, setData] = useState<TResult[] | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [columns, setColumns] = useState<string[]>([]);

  const execute = useCallback(async () => {
    if (!insightId.trim()) {
      setError("Insight ID is required");
      return;
    }

    setIsLoading(true);
    setError(null);
    setData(null);
    setColumns([]);

    try {
      const request: InsightRequest = {
        insight_id: insightId,
        parameters: parameters as Record<string, unknown>,
      };

      const response = await invokeCommand<InsightResponse>("execute_insight", { request });

      if (response.success && response.data) {
        setData(response.data as TResult[]);
        setColumns(response.columns);
      } else {
        setError(response.error || "Unknown error occurred");
        setData(null);
      }
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : "Failed to execute insight";
      setError(errorMessage);
      setData(null);
    } finally {
      setIsLoading(false);
    }
  }, [insightId, parameters]);

  // Auto-fetch on mount if autoFetch is enabled
  useEffect(() => {
    if (autoFetch && insightId) {
      execute();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [autoFetch, insightId]);

  return {
    data,
    isLoading,
    error,
    columns,
    execute,
  };
}

