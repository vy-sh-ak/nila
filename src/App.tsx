import { useState } from "react";
import Sidebar, { type PageId } from "./components/Sidebar";
import TitleBar from "./components/TitleBar";
import HomePage from "./pages/HomePage";
import ModelsPage from "./pages/ModelsPage";
import SettingsPage from "./pages/SettingsPage";

function App() {
  const [collapsed, setCollapsed] = useState(false);
  const [activePage, setActivePage] = useState<PageId>("home");

  return (
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
          className={`relative h-full transition-[padding] duration-500 ease-out ${
            collapsed ? "pl-0" : "pl-60"
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
  );
}

export default App;