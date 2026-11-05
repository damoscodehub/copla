// src/services/imageStore.ts - Web-only persistent blob store for Copla
// Stores image blobs in IndexedDB and caches object URLs in memory.

const DB_NAME = 'copla-images';
const STORE_NAME = 'blobs';
const DB_VERSION = 1;

let dbPromise: Promise<IDBDatabase> | null = null;
const urlCache = new Map<string, string>();

function getDB(): Promise<IDBDatabase> {
  if (dbPromise) return dbPromise;
  dbPromise = new Promise((resolve, reject) => {
    const req = indexedDB.open(DB_NAME, DB_VERSION);
    req.onupgradeneeded = () => {
      const db = req.result;
      if (!db.objectStoreNames.contains(STORE_NAME)) {
        db.createObjectStore(STORE_NAME);
      }
    };
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => reject(req.error);
  });
  return dbPromise;
}

export async function putImage(id: string, blob: Blob): Promise<void> {
  const db = await getDB();
  await new Promise<void>((resolve, reject) => {
    const tx = db.transaction(STORE_NAME, 'readwrite');
    tx.objectStore(STORE_NAME).put(blob, id);
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
  });
  // invalidate cached URL so next get creates fresh one for this blob
  const old = urlCache.get(id);
  if (old) {
    URL.revokeObjectURL(old);
    urlCache.delete(id);
  }
}

export async function getImageBlob(id: string): Promise<Blob | undefined> {
  const db = await getDB();
  return new Promise((resolve, reject) => {
    const tx = db.transaction(STORE_NAME, 'readonly');
    const req = tx.objectStore(STORE_NAME).get(id);
    req.onsuccess = () => resolve(req.result as Blob | undefined);
    req.onerror = () => reject(req.error);
  });
}

export async function getImageUrl(id: string): Promise<string | null> {
  if (urlCache.has(id)) return urlCache.get(id)!;
  const blob = await getImageBlob(id);
  if (!blob) return null;
  const url = URL.createObjectURL(blob);
  urlCache.set(id, url);
  return url;
}

export async function deleteImage(id: string): Promise<void> {
  const db = await getDB();
  await new Promise<void>((resolve, reject) => {
    const tx = db.transaction(STORE_NAME, 'readwrite');
    tx.objectStore(STORE_NAME).delete(id);
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
  });
  const old = urlCache.get(id);
  if (old) {
    URL.revokeObjectURL(old);
    urlCache.delete(id);
  }
}

export function revokeAll(): void {
  for (const url of urlCache.values()) {
    try { URL.revokeObjectURL(url); } catch {}
  }
  urlCache.clear();
}

// Optional: cleanup blobs not referenced by current session
export async function gc(keepIds: Set<string>): Promise<void> {
  const db = await getDB();
  const allKeys: string[] = await new Promise((resolve, reject) => {
    const tx = db.transaction(STORE_NAME, 'readonly');
    const req = tx.objectStore(STORE_NAME).getAllKeys();
    req.onsuccess = () => resolve(req.result as string[]);
    req.onerror = () => reject(req.error);
  });
  for (const key of allKeys) {
    if (!keepIds.has(key)) await deleteImage(key);
  }
}
