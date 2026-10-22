import { invoke } from '@tauri-apps/api/core'

export interface RecognizeTextOptions {
  /** A file path or a `content://` URI. */
  path: string
}

export interface RecognizedText {
  /** All the text, blocks separated by a blank line. */
  text: string
  /** The image as the engine saw it, after its rotation is applied. */
  imageWidth: number
  imageHeight: number
  blocks: TextBlock[]
}

export interface TextBlock {
  text: string
  /** Where the block is, when the engine can tell. */
  bounds: Bounds | null
  lines: TextLine[]
}

export interface TextLine {
  text: string
  /** Where the line is, when the engine can tell. */
  bounds: Bounds | null
  /** From 0 to 1, when the engine gives it. */
  confidence: number | null
  /** Degrees, when the engine gives it. */
  angle: number | null
}

/** A rectangle in pixels of the image. */
export interface Bounds {
  left: number
  top: number
  width: number
  height: number
}

/**
 * Reads the printed text in an image, on the device.
 *
 * An image with no text gives an empty result; an image that cannot be read
 * is an error.
 */
export async function recognizeText(options: RecognizeTextOptions): Promise<RecognizedText> {
  return await invoke<RecognizedText>('plugin:text-recognition|recognize_text', {
    payload: options,
  })
}
