import clsx from "clsx";
import styles from "./Component.module.css";

export type ButtonVariant =
  | "primary"
  | "secondary"
  | "tertiary"
  | "ghost"
  | "link";

export type ButtonShape = "small" | "medium" | "large";

export interface ButtonProps
  extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, "prefix"> {
  children: React.ReactNode;
  variant?: ButtonVariant;
  shape?: ButtonShape;
  prefix?: React.ReactNode;
  suffix?: React.ReactNode;
  fullWidth?: boolean;
  disabled?: boolean;
  loading?: boolean;
}

export const Button = ({
  children,
  className,
  variant = "primary",
  shape = "small",
  prefix,
  suffix,
  fullWidth = false,
  disabled = false,
  loading = false,
  ...props
}: ButtonProps) => {
  const isDisabled = disabled || loading;

  return (
    <button
      className={clsx(styles.button, styles[`button-${variant}`], className)}
      data-size={shape}
      data-full-width={fullWidth}
      data-disabled={disabled}
      data-loading={loading}
      disabled={isDisabled}
      {...props}
    >
      {prefix && <span className={styles["button-prefix"]}>{prefix}</span>}
      {children}
      {suffix && <span className={styles["button-suffix"]}>{suffix}</span>}
    </button>
  );
};
