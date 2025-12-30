import { useState } from "react";
import { AppProvider } from "@/core/providers";
import { useAssetOHLC, useKeyboardShortcut } from "@/shared/hooks";
import { ChartContainer } from "@/shared/components/candlestick";
import { SymbolTimeframeModal } from "@/shared/components";
import "./App.css";

function ChartComponent() {
  const { data, isLoading, error, symbol, timeframe } = useAssetOHLC(true);

  return (
    <ChartContainer
      data={data}
      isLoading={isLoading}
      error={error}
      symbol={symbol}
      timeframe={timeframe}
    />
  );
}

function App() {
  const [symbolTimeframeModalOpen, setSymbolTimeframeModalOpen] = useState(false);

  // Keyboard shortcut for Cmd+A / Ctrl+A
  useKeyboardShortcut(
    "a",
    () => {
      setSymbolTimeframeModalOpen((prev) => !prev);
    },
    { metaKey: true, ctrlKey: true }
  );

  return (
    <AppProvider>
      <SymbolTimeframeModal
        open={symbolTimeframeModalOpen}
        onClose={() => setSymbolTimeframeModalOpen(false)}
      />
      <ChartComponent />
    </AppProvider>
  );
}

export default App;
