/**
 * Tauri API wrapper utilities
 * Updated for Tauri 2 API
 */

import { invoke } from "@tauri-apps/api/core";

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
