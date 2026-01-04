import styles from "./Error.module.css";

interface ErrorProps {
  error: string;
}

export function Error({ error }: ErrorProps) {
  return (
    <div className={styles.error}>
      Error: {error}
    </div>
  );
}
