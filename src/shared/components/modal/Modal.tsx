import { Fragment, ReactNode } from "react";
import { Dialog, DialogPanel, DialogTitle, Transition, TransitionChild } from "@headlessui/react";
import clsx from "clsx";
import styles from "./Modal.module.css";

interface ModalProps {
    open: boolean;
    onClose: () => void;
    title?: string;
    children: ReactNode;
    className?: string;
}

/**
 * Reusable Modal component
 * Uses Headless UI Dialog with CSS modules for styling
 */
export function Modal({ open, onClose, title, children, className }: ModalProps) {
    return (
        <Transition show={open} as={Fragment}>
            <Dialog onClose={onClose} className={styles.dialog}>
                {/* Modal Content */}
                <div className={styles.contentWrapper}>
                    <TransitionChild
                        as={Fragment}
                        enter="ease-out duration-200"
                        enterFrom="opacity-0 scale-95"
                        enterTo="opacity-100 scale-100"
                        leave="ease-out duration-200"
                        leaveFrom="opacity-100 scale-100"
                        leaveTo="opacity-0 scale-95"
                    >
                        <DialogPanel className={clsx(styles.content, className)}>
                            {title && (
                                <div className={styles.header}>
                                    <DialogTitle as="h2" className={styles.title}>
                                        {title}
                                    </DialogTitle>
                                </div>
                            )}
                            <div className={styles.body}>{children}</div>
                            <button
                                type="button"
                                onClick={onClose}
                                className={styles.closeButton}
                                aria-label="Close"
                            >
                                <svg
                                    className={styles.closeIcon}
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                    aria-hidden="true"
                                >
                                    <path
                                        strokeLinecap="round"
                                        strokeLinejoin="round"
                                        strokeWidth={2}
                                        d="M6 18L18 6M6 6l12 12"
                                    />
                                </svg>
                            </button>
                        </DialogPanel>
                    </TransitionChild>
                </div>
            </Dialog>
        </Transition>
    );
}

