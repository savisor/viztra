import { SymbolTimeframeModal } from "@/features/chart/components/SymbolTimeframeModal";

interface SettingsModalProps {
  open: boolean;
  onClose: () => void;
}

export function SettingsModal({ open, onClose }: SettingsModalProps) {
  return <SymbolTimeframeModal open={open} onClose={onClose} />;
}

