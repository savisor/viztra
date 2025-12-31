import { useState } from "react";
import { Modal } from "./Modal";
import { SymbolSelector } from "./SymbolSelector";
import { TimeframeSelector } from "./TimeframeSelector";
import { useSymbolTimeframe } from "@/shared/hooks";
import { invokeCommand } from "@/shared/lib/tauri";
import type { AssetOperationResult } from "@/types";

interface SymbolTimeframeModalProps {
  open: boolean;
  onClose: () => void;
}

/**
 * Modal for selecting symbol and timeframe
 * Connects to SymbolTimeframeProvider and triggers refetch on change
 */
export function SymbolTimeframeModal({
  open,
  onClose,
}: SymbolTimeframeModalProps) {
  const { symbol, timeframe, setSymbol, setTimeframe } = useSymbolTimeframe();
  const [isPullingAssets, setIsPullingAssets] = useState(false);
  const [pullAssetsError, setPullAssetsError] = useState<string | null>(null);
  const [pullAssetsSuccess, setPullAssetsSuccess] = useState<string | null>(null);

  const handleSymbolSelect = (selectedSymbol: string) => {
    setSymbol(selectedSymbol);
  };

  const handleTimeframeSelect = (selectedTimeframe: string) => {
    setTimeframe(selectedTimeframe);
  };

  const handlePullAssets = async () => {
    setIsPullingAssets(true);
    setPullAssetsError(null);
    setPullAssetsSuccess(null);

    try {
      const result = await invokeCommand<AssetOperationResult>("pull_assets");
      if (result.success) {
        setPullAssetsSuccess(result.message);
      } else {
        setPullAssetsError(result.message);
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to pull assets";
      setPullAssetsError(errorMessage);
      console.error("Error pulling assets:", error);
    } finally {
      setIsPullingAssets(false);
    }
  };

  return (
    <Modal open={open} onClose={onClose} title="Select Symbol and Timeframe">
      <div>
        <button
          type="button"
          onClick={handlePullAssets}
          disabled={isPullingAssets}
          style={{
            padding: "0.5rem 1rem",
            backgroundColor: isPullingAssets ? "#9ca3af" : "#3b82f6",
            color: "white",
            border: "none",
            borderRadius: "0.375rem",
            cursor: isPullingAssets ? "not-allowed" : "pointer",
            fontWeight: 500,
            width: "100%",
            marginBottom: "1rem",
          }}
        >
          {isPullingAssets ? "Pulling Assets..." : "Pull Assets"}
        </button>
        {pullAssetsSuccess && (
          <div style={{ color: "#10b981", marginBottom: "1rem", fontSize: "0.875rem" }}>
            {pullAssetsSuccess}
          </div>
        )}
        {pullAssetsError && (
          <div style={{ color: "#ef4444", marginBottom: "1rem", fontSize: "0.875rem" }}>
            Error: {pullAssetsError}
          </div>
        )}
      </div>
      <TimeframeSelector
        selectedTimeframe={timeframe}
        onTimeframeSelect={handleTimeframeSelect}
      />
      <SymbolSelector
        selectedSymbol={symbol}
        onSymbolSelect={handleSymbolSelect}
      />
    </Modal>
  );
}

