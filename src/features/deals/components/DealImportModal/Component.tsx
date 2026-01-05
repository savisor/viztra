import { useState, useCallback, DragEvent, ChangeEvent } from "react";
import { Modal } from "@/shared/ui/Modal";
import { invokeCommand } from "@/shared/services/tauri";
import type { DealImportResult } from "../../types";
import clsx from "clsx";
import styles from "./Component.module.css";

interface DealImportModalProps {
  open: boolean;
  onClose: () => void;
}

interface FileWithData {
  filename: string;
  data: Uint8Array;
}

export function DealImportModal({ open, onClose }: DealImportModalProps) {
  const [isDragging, setIsDragging] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [selectedFiles, setSelectedFiles] = useState<FileWithData[]>([]);
  const [result, setResult] = useState<DealImportResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleDragEnter = useCallback((e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(true);
  }, []);

  const handleDragLeave = useCallback((e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
  }, []);

  const handleDragOver = useCallback((e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
  }, []);

  const readFiles = async (files: FileList | File[]): Promise<FileWithData[]> => {
    const fileArray = Array.from(files);
    const filesWithData: FileWithData[] = [];

    for (const file of fileArray) {
      // Only accept .parquet files
      if (!file.name.toLowerCase().endsWith(".parquet")) {
        continue;
      }

      try {
        const arrayBuffer = await file.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);
        filesWithData.push({
          filename: file.name,
          data: uint8Array,
        });
      } catch (err) {
        console.error(`Failed to read file ${file.name}:`, err);
      }
    }

    return filesWithData;
  };

  const handleDrop = useCallback(
    async (e: DragEvent<HTMLDivElement>) => {
      e.preventDefault();
      e.stopPropagation();
      setIsDragging(false);

      if (!e.dataTransfer.files || e.dataTransfer.files.length === 0) {
        return;
      }

      const filesWithData = await readFiles(e.dataTransfer.files);
      if (filesWithData.length > 0) {
        setSelectedFiles(filesWithData);
        setResult(null);
        setError(null);
      }
    },
    []
  );

  const handleFileInput = useCallback(
    async (e: ChangeEvent<HTMLInputElement>) => {
      if (!e.target.files || e.target.files.length === 0) {
        return;
      }

      const filesWithData = await readFiles(e.target.files);
      if (filesWithData.length > 0) {
        setSelectedFiles(filesWithData);
        setResult(null);
        setError(null);
      }
    },
    []
  );

  const handleImport = useCallback(async () => {
    if (selectedFiles.length === 0) {
      return;
    }

    setIsProcessing(true);
    setError(null);
    setResult(null);

    try {
      // Convert files to format expected by Rust: Vec<(String, Vec<u8>)>
      const files = selectedFiles.map((file) => [
        file.filename,
        Array.from(file.data),
      ]) as [string, number[]][];

      const result = await invokeCommand<DealImportResult>(
        "validate_and_store_deals",
        { files }
      );

      setResult(result);
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : "Failed to import deals";
      setError(errorMessage);
      console.error("Error importing deals:", err);
    } finally {
      setIsProcessing(false);
    }
  }, [selectedFiles]);

  const handleClear = useCallback(() => {
    setSelectedFiles([]);
    setResult(null);
    setError(null);
  }, []);

  const handleClose = useCallback(() => {
    handleClear();
    onClose();
  }, [handleClear, onClose]);

  return (
    <Modal open={open} onClose={handleClose} title="Import Deals">
      <div className={styles.content}>
        {/* Drop zone */}
        <div
          className={clsx(styles.dropZone, isDragging && styles.dropZoneActive)}
          onDragEnter={handleDragEnter}
          onDragLeave={handleDragLeave}
          onDragOver={handleDragOver}
          onDrop={handleDrop}
        >
          <input
            type="file"
            id="file-input"
            accept=".parquet"
            multiple
            onChange={handleFileInput}
            className={styles.fileInput}
          />
          <label htmlFor="file-input" className={styles.dropZoneLabel}>
            <div className={styles.dropZoneIcon}>
              <svg
                width="48"
                height="48"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
              >
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="17 8 12 3 7 8" />
                <line x1="12" y1="3" x2="12" y2="15" />
              </svg>
            </div>
            <div className={styles.dropZoneText}>
              <strong>Drag and drop Parquet files here</strong>
              <span>or click to browse</span>
            </div>
          </label>
        </div>

        {/* Selected files list */}
        {selectedFiles.length > 0 && (
          <div className={styles.filesList}>
            <div className={styles.filesListHeader}>
              <span>Selected files ({selectedFiles.length})</span>
              <button
                type="button"
                onClick={handleClear}
                className={styles.clearButton}
              >
                Clear
              </button>
            </div>
            <ul className={styles.filesListItems}>
              {selectedFiles.map((file, index) => (
                <li key={index} className={styles.filesListItem}>
                  {file.filename}
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* Import button */}
        {selectedFiles.length > 0 && (
          <button
            type="button"
            onClick={handleImport}
            disabled={isProcessing}
            className={styles.importButton}
          >
            {isProcessing ? "Processing..." : "Import Files"}
          </button>
        )}

        {/* Results */}
        {result && (
          <div className={styles.results}>
            <div
              className={clsx(
                styles.resultMessage,
                result.success ? styles.resultSuccess : styles.resultError
              )}
            >
              {result.message}
            </div>
            {result.file_results.length > 0 && (
              <div className={styles.fileResults}>
                {result.file_results.map((fileResult, index) => (
                  <div
                    key={index}
                    className={clsx(
                      styles.fileResult,
                      fileResult.success
                        ? styles.fileResultSuccess
                        : styles.fileResultError
                    )}
                  >
                    <div className={styles.fileResultName}>
                      {fileResult.filename}
                    </div>
                    <div className={styles.fileResultMessage}>
                      {fileResult.message}
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}

        {/* Error message */}
        {error && (
          <div className={clsx(styles.resultMessage, styles.resultError)}>
            Error: {error}
          </div>
        )}
      </div>
    </Modal>
  );
}

