import { useState, useEffect, Dispatch, SetStateAction } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * Persistent state hook backed by the Tauri Store plugin
 * (settings.json in the app config directory).
 *
 * Mirrors the `useLocalStorage` API: same <T> generic and tuple return,
 * but reads/writes go through the Rust-side store commands.
 */
export function useStore<T>(
  key: string,
  defaultValue: T
): [T, Dispatch<SetStateAction<T>>] {
  const [value, setValue] = useState<T>(defaultValue);
  const [loaded, setLoaded] = useState(false);

  // Load the persisted value once per key.
  useEffect(() => {
    let active = true;
    setLoaded(false);

    invoke<T | null>('store_get', { key })
      .then((saved) => {
        if (!active || saved === null || saved === undefined) return;
        setValue(saved as T);
      })
      .catch((error) => {
        console.error(`Error reading store key "${key}":`, error);
      })
      .finally(() => {
        if (active) setLoaded(true);
      });

    return () => {
      active = false;
    };
  }, [key]);

  // Persist changes, but only after the initial load has completed so we
  // never overwrite the stored value with the default.
  useEffect(() => {
    if (!loaded) return;

    invoke('store_set', { key, value })
      .catch((error) => {
        console.error(`Error setting store key "${key}":`, error);
      });
  }, [key, value, loaded]);

  // Return as a constant tuple to prevent type widening to an array of (T | Dispatch)[]
  return [value, setValue];
}
