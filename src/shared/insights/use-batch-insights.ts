/**
 * Hook for executing multiple insights concurrently
 */

import { useState, useEffect, useCallback } from "react";
import { invokeCommand } from "@/shared/services/tauri";
import type { InsightRequest } from "./types";

export interface BatchInsightConfig {
  insight_id: string;
  parameters: Record<string, unknown>;
}

export interface BatchInsightItem {
  insight_id: string;
  success: boolean;
  data?: Record<string, unknown>[];
  error?: string;
  columns: string[];
}

export interface BatchInsightResponse {
  results: BatchInsightItem[];
}

export interface UseBatchInsightsOptions {
  autoFetch?: boolean;
}

export interface UseBatchInsightsReturn {
  data: BatchInsightItem[] | null;
  isLoading: boolean;
  error: string | null;
  execute: () => Promise<void>;
}

/**
 * Hook for executing multiple insights concurrently with loading, error, and data state management
 * 
 * @param configs - Array of insight configurations (id + parameters)
 * @param options - Optional configuration (autoFetch)
 * @returns Object containing data (array of results), loading state, error state, and execute function
 */
export function useBatchInsights(
  configs: BatchInsightConfig[],
  options: UseBatchInsightsOptions = {}
): UseBatchInsightsReturn {
  const { autoFetch = false } = options;

  const [data, setData] = useState<BatchInsightItem[] | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const execute = useCallback(async () => {
    if (!configs || configs.length === 0) {
      setError("At least one insight configuration is required");
      return;
    }

    setIsLoading(true);
    setError(null);
    setData(null);

    try {
      // Convert configs to InsightRequest format
      const requests: InsightRequest[] = configs.map((config) => ({
        insight_id: config.insight_id,
        parameters: config.parameters,
      }));

      const response = await invokeCommand<BatchInsightResponse>("execute_batch_insights", {
        request: { requests },
      });

      if (response && response.results) {
        setData(response.results);
        // Check if any insights failed
        const hasErrors = response.results.some((r) => !r.success);
        if (hasErrors) {
          const errorMessages = response.results
            .filter((r) => !r.success)
            .map((r) => `${r.insight_id}: ${r.error || "Unknown error"}`)
            .join("; ");
          // Set error but still show partial results
          setError(`Some insights failed: ${errorMessages}`);
        }
      } else {
        setError("Invalid response format");
        setData(null);
      }
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : "Failed to execute batch insights";
      setError(errorMessage);
      setData(null);
    } finally {
      setIsLoading(false);
    }
  }, [configs]);

  // Auto-fetch on mount if autoFetch is enabled
  useEffect(() => {
    if (autoFetch && configs && configs.length > 0) {
      execute();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [autoFetch]);

  return {
    data,
    isLoading,
    error,
    execute,
  };
}

