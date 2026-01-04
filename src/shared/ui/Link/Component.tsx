import clsx from "clsx";
import NextLink from "next/link";
import { ArrowSquareOutIcon } from "@phosphor-icons/react";
import styles from "./Component.module.css";
import type { Shape } from "@/shared/ui/types";

export type LinkVariant = "button" | "nav";

export interface LinkProps {
  href: string;
  children: React.ReactNode;
  active?: boolean;
  variant?: LinkVariant;
  shape?: Shape;
  badge?: string | number;
  className?: string;
  style?: React.CSSProperties;
  ariaLabel?: string;
}

export const Link = ({
  href,
  children,
  active = false,
  variant = "button",
  shape = "sm",
  badge,
  className,
  style,
  ariaLabel,
}: LinkProps) => {
  const isExternal = href.startsWith("http://") || href.startsWith("https://");

  return (
    <NextLink
      href={href}
      className={clsx(styles.link, styles[`link-${variant}`], className)}
      data-active={active}
      data-shape={shape}
      style={style}
      {...(isExternal && { target: "_blank", rel: "noopener noreferrer" })}
      {...(ariaLabel && { "aria-label": ariaLabel })}
    >
      <span className={styles["link-text"]}>{children}</span>
      {badge && <span className={styles.badge}>[{badge}]</span>}
      {isExternal && (
        <ArrowSquareOutIcon
          size={16}
          weight="regular"
          className={styles["external-icon"]}
        />
      )}
    </NextLink>
  );
};

Link.displayName = "Link";
