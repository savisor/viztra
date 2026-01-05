/**
 * Shared types for insights system
 */

export interface InsightRequest {
  insight_id: string;
  parameters: Record<string, unknown>;
}

export interface InsightResponse {
  success: boolean;
  data?: Record<string, unknown>[];
  error?: string;
  columns: string[];
}

export interface BatchInsightRequest {
  requests: InsightRequest[];
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

