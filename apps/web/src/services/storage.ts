import type { GameDNA, VersionInfo } from '@entropic/types';

const DB_NAME = 'entropic-dev-portal';
const DB_VERSION = 1;

type StoreName = 'configs' | 'versions';

/**
 * Initializes and opens the application's IndexedDB database and ensures required object stores exist.
 *
 * @returns The opened IndexedDB `IDBDatabase` instance.
 */
function openDB(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(DB_NAME, DB_VERSION);

    request.onupgradeneeded = () => {
      const db = request.result;
      if (!db.objectStoreNames.contains('configs')) {
        db.createObjectStore('configs', { keyPath: 'id' });
      }
      if (!db.objectStoreNames.contains('versions')) {
        db.createObjectStore('versions', { keyPath: 'key' });
      }
    };

    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}

/**
 * Execute an IndexedDB request on the specified object store within a transaction and return its result.
 *
 * @param fn - A callback that is given the target `IDBObjectStore` and must return an `IDBRequest<T>` to execute.
 * @returns The value produced by the provided `IDBRequest`, typed as `T`.
 */
async function tx<T>(storeName: StoreName, mode: IDBTransactionMode, fn: (store: IDBObjectStore) => IDBRequest<T>): Promise<T> {
  const db = await openDB();
  return new Promise((resolve, reject) => {
    const transaction = db.transaction(storeName, mode);
    const store = transaction.objectStore(storeName);
    const request = fn(store);

    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}

export const storage = {
  async listConfigs(): Promise<GameDNA[]> {
    const db = await openDB();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction('configs', 'readonly');
      const store = transaction.objectStore('configs');
      const request = store.getAll();
      request.onsuccess = () => resolve(request.result as GameDNA[]);
      request.onerror = () => reject(request.error);
    });
  },

  async getConfig(id: string): Promise<GameDNA | null> {
    const result = await tx<GameDNA | undefined>('configs', 'readonly', (store) => store.get(id));
    return result ?? null;
  },

  async putConfig(config: GameDNA): Promise<void> {
    await tx('configs', 'readwrite', (store) => store.put(config));
  },

  async deleteConfig(id: string): Promise<void> {
    await tx('configs', 'readwrite', (store) => store.delete(id));
  },

  async getVersions(id: string): Promise<VersionInfo[]> {
    const key = `versions:${id}`;
    const record = await tx<{ key: string; versions: VersionInfo[] } | undefined>('versions', 'readonly', (store) => store.get(key));
    return record?.versions ?? [];
  },

  async pushVersion(id: string, version: VersionInfo): Promise<void> {
    const key = `versions:${id}`;
    const versions = await this.getVersions(id);
    const next = [...versions, version];
    await tx('versions', 'readwrite', (store) => store.put({ key, versions: next }));
  },

  async setVersions(id: string, versions: VersionInfo[]): Promise<void> {
    const key = `versions:${id}`;
    await tx('versions', 'readwrite', (store) => store.put({ key, versions }));
  },
};