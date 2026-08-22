import { invoke } from "@tauri-apps/api/core";
import { Plus, X } from "lucide-react";
import useSWR from "swr";
import { CuteLoading } from "../components/Loaders";
import { CuteError } from "../components/Errors";
import { useState } from "react";
import { ModelFormCard, ModelCard, type Model } from "../components/ModelCard";

export default function ModelsPage() {
  const [formOpen, setFormOpen] = useState(false);
  const getModelList = (cmd: string) => invoke<Model[]>(cmd);
  const { data, isLoading, error, mutate } = useSWR("model_list", getModelList);

  // The add button toggles the inline form card.
  function toggleForm() {
    setFormOpen((open) => !open);
  }

  if (isLoading) {
    return <CuteLoading />;
  }
  if (error) {
    return (
      <CuteError
        title="Failed to load models"
        description="Couldn't load models due to an unknown error"
      />
    );
  }

  return (
    <div className="min-h-full flex flex-col p-2 gap-4">
      <div className="w-full text-end">
        <button className="btn btn-primary mt-2" onClick={toggleForm} aria-expanded={formOpen}>
          {formOpen ? <X className="h-4 w-4" /> : <Plus className="h-4 w-4" />}
          {formOpen ? "Close" : "Add a model"}
        </button>
      </div>

      {formOpen && (
        <ModelFormCard
          onCancel={() => setFormOpen(false)}
          onSaved={() => {
            setFormOpen(false);
            mutate();
          }}
        />
      )}

      {data && data.length > 0 ? (
        <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 xl:grid-cols-3">
          {data.map((model) => (
            <ModelCard key={model.id} model={model} onChanged={() => mutate()} />
          ))}
        </div>
      ) : (
        <p className="text-center text-base-content/60">
          No models yet. Click &quot;Add a model&quot; to create one.
        </p>
      )}
    </div>
  );
}
