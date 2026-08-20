import { Bot, Plus } from "lucide-react";

export default function ModelsPage() {
  return (
    <div className="flex min-h-full flex-col items-center justify-center gap-6 p-8">
      <div className="card w-full max-w-xl bg-base-200/80 shadow-xl backdrop-blur-sm">
        <div className="card-body items-center gap-4 text-center">
          <div className="avatar placeholder">
            <div className="w-16 rounded-box bg-base-300">
              <Bot className="h-8 w-8" />
            </div>
          </div>
          <h2 className="text-2xl font-bold tracking-tight">No models yet</h2>
          <p className="max-w-sm text-base-content/60">
            Connect a model provider to start using Nila. Your configured
            models will show up here.
          </p>
          <button className="btn btn-primary mt-2">
            <Plus className="h-4 w-4" />
            Add a model
          </button>
        </div>
      </div>
    </div>
  );
}