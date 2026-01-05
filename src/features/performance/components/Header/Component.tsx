import { Button } from "@/shared/ui/Button";
import styles from "./Component.module.css";

interface HeaderProps {
  onImportClick: () => void;
}

export function Header({ onImportClick }: HeaderProps) {
  return (
    <header className={styles.header}>
      <h1 className={styles.title}>Performance</h1>
      <Button variant="primary" shape="medium" onClick={onImportClick}>
        Import
      </Button>
    </header>
  );
}

