/**
 * Tauri API wrapper utilities
 */

import { invoke } from "@tauri-apps/api/tauri";
import type { ApiResponse } from "@/types";

/**
 * Generic Tauri command invoker with error handling
 */
export async function invokeCommand<T = unknown>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw new Error(
      `Failed to invoke command '${command}': ${error instanceof Error ? error.message : String(error)}`
    );
  }
}

/**
 * Type-safe command invoker that returns ApiResponse
 */
export async function invokeCommandSafe<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<ApiResponse<T>> {
  try {
    const data = await invokeCommand<T>(command, args);
    return {
      data,
      success: true,
    };
  } catch (error) {
    return {
      data: null as T,
      success: false,
      error: error instanceof Error ? error.message : String(error),
    };
  }
}


