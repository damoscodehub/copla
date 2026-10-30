import { describe, expect, it } from 'vitest';
import { clampView, dimensions, initialViewport, targets, zoomAt } from './geometry';

describe('normalized viewports', () => {
  it('keeps synchronization within pinned images and requires a selected origin', () => {
    expect(targets('a', ['a','b','c'], ['a','b','d'], 'selection')).toEqual(['a','b']);
    expect(targets('c', ['a','b','c'], ['a','b'], 'selection')).toEqual(['c']);
    expect(targets('a', ['a','b'], [], 'all')).toEqual(['a','b']);
    expect(targets('a', ['a','b'], ['a','b'], 'none')).toEqual(['a']);
  });
  it('preserves aspect ratio for fit and fill', () => {
    expect(dimensions(200, 100, 100, 100, initialViewport())).toMatchObject({width:100,height:50});
    expect(dimensions(200, 100, 100, 100, {...initialViewport(),fit:'fill'})).toMatchObject({width:200,height:100});
  });
  it('anchors pointer zoom and clamps boundaries', () => {
    const v = zoomAt(initialViewport(), 2, 25, 0, 100, 100, 100, 100);
    expect(v.x).toBe(.625);
    expect(v.zoom).toBe(2);
    expect(clampView({...v,x:4},100,100,100,100).x).toBe(.75);
  });
});
