import { Bell, Palette, SlidersHorizontal } from "lucide-react";

export default function SettingsPage() {
  return (
    <div className="flex min-h-full flex-col items-center justify-center gap-6 p-8">
      <div className="card w-full max-w-xl bg-base-200/80 shadow-xl backdrop-blur-sm">
        <div className="card-body gap-6">
          <div className="flex items-center gap-3">
            <div className="avatar placeholder">
              <div className="w-10 rounded-box bg-base-300">
                <SlidersHorizontal className="h-5 w-5" />
              </div>
            </div>
            <div>
              <h2 className="text-2xl font-bold tracking-tight">Settings</h2>
              <p className="text-sm text-base-content/60">
                Preferences for your assistant
              </p>
            </div>
          </div>

          <div className="divider my-0"></div>

          <div className="flex flex-col gap-4">
            <label className="flex items-center justify-between gap-4">
              <span className="flex items-center gap-3">
                <Bell className="h-4 w-4 text-base-content/60" />
                Notifications
              </span>
              <input
                type="checkbox"
                className="toggle toggle-sm"
                defaultChecked
              />
            </label>
            <label className="flex items-center justify-between gap-4">
              <span className="flex items-center gap-3">
                <Palette className="h-4 w-4 text-base-content/60" />
                Dark theme
              </span>
              <input type="checkbox" className="toggle toggle-sm" checked readOnly />
            </label>
          </div>
        </div>
      </div>
    </div>
  );
}