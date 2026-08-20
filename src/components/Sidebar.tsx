import { Cpu, Home, Settings } from "lucide-react";

export type PageId = "home" | "models" | "settings";

const NAV_ITEMS: { id: PageId; label: string; icon: typeof Home }[] = [
  { id: "home", label: "Home", icon: Home },
  { id: "models", label: "Models", icon: Cpu },
  { id: "settings", label: "Settings", icon: Settings },
];

interface SidebarProps {
  collapsed: boolean;
  activePage: PageId;
  onNavigate: (page: PageId) => void;
}

export default function Sidebar({
  collapsed,
  activePage,
  onNavigate,
}: SidebarProps) {
  return (
    <aside
      className={`absolute z-10 transition-all duration-500 ease-out ${
        collapsed
          ? "left-3 top-1/2 h-fit w-14 -translate-y-1/2 rounded-3xl border border-base-300 bg-base-200/90 shadow-xl backdrop-blur-md"
          : "left-0 top-0 h-full w-60 rounded-none border-r border-base-300 bg-base-200/70 backdrop-blur-sm"
      }`}
    >
      <ul
        className={`menu w-full gap-1 p-2 ${
          collapsed ? "justify-center" : ""
        }`}
      >
        {NAV_ITEMS.map(({ id, label, icon: Icon }) => {
          const active = id === activePage;
          return (
            <li
              key={id}
              data-tip={label}
              className={collapsed ? "tooltip tooltip-right" : undefined}
            >
              <button
                onClick={() => onNavigate(id)}
                aria-label={collapsed ? label : undefined}
                aria-current={active ? "page" : undefined}
                className={`${active ? "menu-active" : ""} ${
                  collapsed ? "justify-center gap-0 rounded-full p-2" : "justify-start"
                }`}
              >
                <Icon className="h-5 w-5 shrink-0" />
                <span
                  className={`grid transition-[grid-template-columns] duration-300 ease-out ${
                    collapsed ? "grid-cols-[0fr]" : "grid-cols-[1fr]"
                  }`}
                >
                  <span className="min-w-0 overflow-hidden whitespace-nowrap">
                    {label}
                  </span>
                </span>
              </button>
            </li>
          );
        })}
      </ul>
    </aside>
  );
}