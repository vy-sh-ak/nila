import { useEffect, useState } from "react";
import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  Copy,
  Minus,
  PanelLeftClose,
  PanelLeftOpen,
  Sparkles,
  Square,
  X,
} from "lucide-react";

interface TitleBarProps {
  collapsed: boolean;
  onToggleSidebar: () => void;
}

function appWindow() {
  try {
    return getCurrentWindow();
  } catch {
    return null;
  }
}

export default function TitleBar({ collapsed, onToggleSidebar }: TitleBarProps) {
  const [isNative, setIsNative] = useState(false);
  const [maximized, setMaximized] = useState(false);

  useEffect(() => {
    setIsNative(isTauri());
    if (!isTauri()) return;

    const win = getCurrentWindow();
    let unlisten: (() => void) | undefined;
    win.isMaximized().then(setMaximized);
    win
      .onResized(() => {
        win.isMaximized().then(setMaximized);
      })
      .then((fn) => {
        unlisten = fn;
      });
    return () => {
      unlisten?.();
    };
  }, []);

  const handleToggleMaximize = () => appWindow()?.toggleMaximize();
  const handleDoubleClick = (e: React.MouseEvent) => {
    if ((e.target as HTMLElement).closest("button")) return;
    appWindow()?.toggleMaximize();
  };

  return (
    <header
      data-tauri-drag-region
      onDoubleClick={handleDoubleClick}
      className="relative flex h-12 shrink-0 select-none items-center border-b border-base-300/70 bg-base-200/60 backdrop-blur-md"
    >
      <div data-tauri-drag-region className="flex w-60 items-center pl-2">
        <button
          className="btn btn-ghost btn-square btn-sm"
          onClick={onToggleSidebar}
          aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
        >
          {collapsed ? (
            <PanelLeftOpen className="h-4 w-4" />
          ) : (
            <PanelLeftClose className="h-4 w-4" />
          )}
        </button>
      </div>

      <div className="pointer-events-none absolute left-1/2 top-1/2 flex -translate-x-1/2 -translate-y-1/2 items-center gap-2">
        <div className="avatar placeholder">
          <div className="w-4 rounded-md bg-primary text-primary-content">
            <Sparkles className="h-3.5 w-3.5" />
          </div>
        </div>
        <span className="text-sm font-semibold tracking-tight">Nila</span>
      </div>

      {isNative && (
        <div className="ml-auto flex h-full items-center mr-2">
          <button
            className="btn btn-ghost btn-square btn-sm"
            onClick={() => appWindow()?.minimize()}
            aria-label="Minimize"
          >
            <Minus className="h-4 w-4" />
          </button>
          <button
            className="btn btn-ghost btn-square btn-sm"
            onClick={handleToggleMaximize}
            aria-label={maximized ? "Restore" : "Maximize"}
          >
            {maximized ? (
              <Copy className="h-3.5 w-3.5 -scale-x-100" />
            ) : (
              <Square className="h-3.5 w-3.5" />
            )}
          </button>
          <button
            className="btn btn-ghost btn-square btn-sm hover:bg-error hover:text-error-content"
            onClick={() => appWindow()?.close()}
            aria-label="Close"
          >
            <X className="h-4 w-4" />
          </button>
        </div>
      )}
    </header>
  );
}