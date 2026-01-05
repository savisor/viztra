import { useState } from "react";
import ChartScreen from "./features/chart";
import PerformanceScreen from "./features/performance";
import HomeScreen from "./features/home";
import { HamburgerMenu } from "@/shared/ui/HamburgerMenu";
import { SettingsModal } from "@/features/settings";
import { useKeyboardShortcut, useCachedView } from "@/shared/hooks";
import { AppProvider } from "./features/chart/providers";
import "@/shared/theme/globals.css";
import { Stack } from "./shared/ui/Stack";

function App() {
  const [currentView, setCurrentView] = useCachedView();
  const [settingsModalOpen, setSettingsModalOpen] = useState(false);

  // Keyboard shortcut for Cmd+A / Ctrl+A - opens Settings (which contains SymbolTimeframeModal)
  useKeyboardShortcut(
    "a",
    () => {
      setSettingsModalOpen((prev) => !prev);
    },
    { metaKey: true, ctrlKey: true }
  );

  return (
    <AppProvider>
      <Stack>
        <HamburgerMenu
          onSettingsClick={() => setSettingsModalOpen(true)}
          onPerformanceClick={() => setCurrentView("performance")}
          onHomeClick={() => setCurrentView("home")}
          onTradesClick={() => setCurrentView("performance")}
        />
        <SettingsModal
          open={settingsModalOpen}
          onClose={() => setSettingsModalOpen(false)}
        />
        {currentView === "home" && <HomeScreen />}
        {currentView === "performance" && <PerformanceScreen />}
        {currentView === "chart" && <ChartScreen />}
      </Stack>
    </AppProvider>
  );
}

export default App;
