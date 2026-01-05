import { Menu, MenuButton, MenuItem, MenuItems, Transition } from "@headlessui/react";
import clsx from "clsx";
import styles from "./Component.module.css";

export interface HamburgerMenuProps {
  onSettingsClick: () => void;
  onPerformanceClick: () => void;
  onHomeClick: () => void;
  onTradesClick: () => void;
}

export function HamburgerMenu({
  onSettingsClick,
  onPerformanceClick,
  onHomeClick,
  onTradesClick,
}: HamburgerMenuProps) {
  return (
    <Menu as="div" className={styles.menu}>
      <MenuButton className={styles.button} aria-label="Menu">
        <svg
          className={styles.icon}
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          aria-hidden="true"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M4 6h16M4 12h16M4 18h16"
          />
        </svg>
      </MenuButton>
      <Transition
        enter="transition ease-out duration-100"
        enterFrom="transform opacity-0 scale-95"
        enterTo="transform opacity-100 scale-100"
        leave="transition ease-in duration-75"
        leaveFrom="transform opacity-100 scale-100"
        leaveTo="transform opacity-0 scale-95"
      >
        <MenuItems className={styles.items}>
          <MenuItem>
            {({ focus }) => (
              <button
                type="button"
                onClick={onHomeClick}
                className={clsx(styles.item, focus && styles.itemFocus)}
              >
                Home
              </button>
            )}
          </MenuItem>
          <MenuItem>
            {({ focus }) => (
              <button
                type="button"
                onClick={onPerformanceClick}
                className={clsx(styles.item, focus && styles.itemFocus)}
              >
                Performance
              </button>
            )}
          </MenuItem>
          <MenuItem>
            {({ focus }) => (
              <button
                type="button"
                onClick={onTradesClick}
                className={clsx(styles.item, focus && styles.itemFocus)}
              >
                Trades
              </button>
            )}
          </MenuItem>
          <MenuItem>
            {({ focus }) => (
              <button
                type="button"
                onClick={onSettingsClick}
                className={clsx(styles.item, focus && styles.itemFocus)}
              >
                Settings
              </button>
            )}
          </MenuItem>
        </MenuItems>
      </Transition>
    </Menu>
  );
}

