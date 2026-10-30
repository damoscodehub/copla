export type SyncMode = 'none' | 'all' | 'selection';
export type Viewport = { x: number; y: number; zoom: number; fit: 'fit' | 'fill' };
export const initialViewport = (): Viewport => ({ x: .5, y: .5, zoom: 1, fit: 'fit' });
export function targets(origin: string, comparison: string[], selection: string[], mode: SyncMode) {
  if (mode === 'all') return comparison;
  if (mode === 'selection' && selection.includes(origin)) return comparison.filter(id => selection.includes(id));
  return [origin];
}
export function dimensions(width: number, height: number, boxWidth: number, boxHeight: number, view: Viewport) {
  const baseline = (view.fit === 'fit' ? Math.min : Math.max)(boxWidth / width, boxHeight / height);
  const scale = baseline * view.zoom;
  return { width: width * scale, height: height * scale, scale };
}
export function clampView(view: Viewport, width: number, height: number, boxWidth: number, boxHeight: number): Viewport {
  const size = dimensions(width, height, boxWidth, boxHeight, view);
  const clampAxis = (center: number, extent: number, box: number) => extent <= box ? .5 : Math.max(box / (2 * extent), Math.min(1 - box / (2 * extent), center));
  return { ...view, x: clampAxis(view.x, size.width, boxWidth), y: clampAxis(view.y, size.height, boxHeight) };
}
export function zoomAt(view: Viewport, factor: number, offsetX: number, offsetY: number, width: number, height: number, boxWidth: number, boxHeight: number): Viewport {
  const before = dimensions(width, height, boxWidth, boxHeight, view);
  const zoom = Math.max(.1, Math.min(32, view.zoom * factor));
  const after = dimensions(width, height, boxWidth, boxHeight, { ...view, zoom });
  return clampView({ ...view, zoom, x: view.x + offsetX / before.width - offsetX / after.width, y: view.y + offsetY / before.height - offsetY / after.height }, width, height, boxWidth, boxHeight);
}
