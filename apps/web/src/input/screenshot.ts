export type BlobLike = { size: number }
export type CanvasLike = { toBlob(callback: (blob: BlobLike | null) => void, type?: string): void }
export type Saver = (blob: BlobLike, filename: string) => void

const pad = (value: number, width = 2) => String(value).padStart(width, '0')

/** UTC so screenshots from different machines sort together. */
export function screenshotFilename(date = new Date()): string {
  const day = `${date.getUTCFullYear()}-${pad(date.getUTCMonth() + 1)}-${pad(date.getUTCDate())}`
  const time = `${pad(date.getUTCHours())}${pad(date.getUTCMinutes())}${pad(date.getUTCSeconds())}`
  return `arena-${day}-${time}.png`
}

/** Encodes the arena canvas and hands the blob to the saver; returns the filename it will use. */
export function captureCanvas(canvas: CanvasLike | null | undefined, save: Saver, now = new Date()): string | null {
  if (!canvas || typeof canvas.toBlob !== 'function') return null
  const filename = screenshotFilename(now)
  canvas.toBlob(blob => {
    if (blob) save(blob, filename)
  }, 'image/png')
  return filename
}

/** Browser saver: hands the blob to the page as a download and releases the object URL. */
export function downloadBlob(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = filename
  link.click()
  URL.revokeObjectURL(url)
}
