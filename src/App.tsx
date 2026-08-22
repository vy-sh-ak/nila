import { useState } from "react";
import Sidebar, { type PageId } from "./components/Sidebar";
import TitleBar from "./components/TitleBar";
import HomePage from "./pages/HomePage";
import ModelsPage from "./pages/ModelsPage";
import SettingsPage from "./pages/SettingsPage";
import { useStore } from "./hooks/useStore";
import { SWRConfig } from "swr";

function App() {
  const [collapsed, setCollapsed] = useStore("sidebar-collapsed", false);
  const [activePage, setActivePage] = useState<PageId>("home");

  return (
    <SWRConfig
      value={{
        refreshInterval: 3000,
      }}
    >
      <div className="flex h-screen w-screen flex-col overflow-hidden bg-base-100 text-base-content">
        <TitleBar
          collapsed={collapsed}
          onToggleSidebar={() => setCollapsed((c) => !c)}
        />

        <div className="relative flex-1">
          <Sidebar
            collapsed={collapsed}
            activePage={activePage}
            onNavigate={setActivePage}
          />

          <main
            className={`relative h-full transition-[padding] duration-500 ease-out pr-2 ${
              collapsed ? "pl-20" : "pl-60"
            }`}
          >
            <div className="canvas-dots absolute inset-0" aria-hidden="true" />
            <div className="relative h-full overflow-y-auto">
              {activePage === "home" && <HomePage />}
              {activePage === "models" && <ModelsPage />}
              {activePage === "settings" && <SettingsPage />}
            </div>
          </main>
        </div>
      </div>
    </SWRConfig>
  );
}

export default App;
