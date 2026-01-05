import { useState, useEffect } from "react";

type View = "home" | "performance" | "chart";

const STORAGE_KEY = "viztra.lastView";
const DEFAULT_VIEW: View = "home";

const isValidView = (value: string | null): value is View => {
  return value === "home" || value === "performance" || value === "chart";
};

/**
 * Hook that persists the current view to localStorage and restores it on mount.
 * Uses synchronous localStorage operations for immediate, zero-delay access.
 */
export function useCachedView(): [View, (view: View) => void] {
  // Initialize state with cached value or default
  const [currentView, setCurrentViewState] = useState<View>(() => {
    try {
      const cached = localStorage.getItem(STORAGE_KEY);
      return isValidView(cached) ? cached : DEFAULT_VIEW;
    } catch {
      // If localStorage is unavailable, return default
      return DEFAULT_VIEW;
    }
  });

  // Persist to localStorage whenever view changes
  useEffect(() => {
    try {
      localStorage.setItem(STORAGE_KEY, currentView);
    } catch {
      // Silently fail if localStorage is unavailable
    }
  }, [currentView]);

  return [currentView, setCurrentViewState];
}

