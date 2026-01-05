"use client";

import clsx from "clsx";
import styles from "./Component.module.css";
import { Bordered, Gap, Margin, Padding } from "../types";

export type ContainerVariant = "small" | "medium" | "large";

export interface ContainerProps {
  children: React.ReactNode;
  variant?: ContainerVariant;
  padding?: Padding;
  bordered?: Bordered;
  marginInline?: Margin;
  marginBlock?: Margin;
  gap?: Gap;
  divider?: boolean;
  className?: string;
  style?: React.CSSProperties;
}

export const Container = ({
  children,
  variant = "medium",
  bordered = "full",
  padding = "0",
  marginBlock = "0",
  marginInline = "0",
  gap = "0",
  divider = false,
  className,
  style,
}: ContainerProps) => {
  return (
    <div
      className={clsx(styles.container, styles[variant], className)}
      data-variant={variant}
      data-bordered={bordered}
      data-padding={padding}
      data-margin-block={marginBlock}
      data-margin-inline={marginInline}
      data-gap={gap}
      data-direction="vertical"
      data-divider={divider}
      style={style}
    >
      {children}
    </div>
  );
};
