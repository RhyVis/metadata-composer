export interface DragDropPayload {
  paths: string[];
  position: {
    x: number;
    y: number;
  };
}

export type CompressionInfoPayload = [number, number, string];

export type DecompressionInfoPayload = [number, number, string];
