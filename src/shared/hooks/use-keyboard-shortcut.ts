import { useEffect } from "react";

/**
 * Hook for handling keyboard shortcuts
 * Supports Cmd (Mac) and Ctrl (Windows/Linux) modifiers
 */
export function useKeyboardShortcut(
  key: string,
  callback: (event: KeyboardEvent) => void,
  options: {
    ctrlKey?: boolean;
    metaKey?: boolean;
    shiftKey?: boolean;
    altKey?: boolean;
  } = {}
) {
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      const {
        ctrlKey = false,
        metaKey = false,
        shiftKey = false,
        altKey = false,
      } = options;

      const isMac = navigator.platform.toUpperCase().indexOf("MAC") >= 0;
      
      // For Cmd+A / Ctrl+A, we want either metaKey (Mac) or ctrlKey (Windows/Linux)
      let modifierPressed = false;
      if (metaKey && ctrlKey) {
        // Both specified - use platform-specific check
        modifierPressed = isMac ? event.metaKey : event.ctrlKey;
      } else if (metaKey) {
        modifierPressed = event.metaKey;
      } else if (ctrlKey) {
        modifierPressed = event.ctrlKey;
      }

      if (
        event.key.toLowerCase() === key.toLowerCase() &&
        modifierPressed &&
        shiftKey === event.shiftKey &&
        altKey === event.altKey
      ) {
        event.preventDefault();
        callback(event);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [key, callback, options.ctrlKey, options.metaKey, options.shiftKey, options.altKey]);
}

