export interface DealImportResult {
  success: boolean;
  message: string;
  file_results: FileImportResult[];
}

export interface FileImportResult {
  filename: string;
  success: boolean;
  message: string;
}

export interface Deal {
  ticket: number;
  order: number;
  time: number;
  time_msc: number;
  type: number;
  entry: number;
  magic: number;
  position_id: number;
  reason: number;
  volume: number;
  price: number;
  commission: number;
  swap: number;
  profit: number;
  fee: number;
  symbol: string;
  comment: string;
  external_id: string;
}

