import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { Lock, LockOpen, Pencil, Trash2 } from "lucide-react";
import { getModelProvider } from "../helpers/getModelProviders";

/** Mirrors the Rust `Model` (snake_case serde fields). */
export type Model = {
  id: number;
  provider: string;
  url: string;
  name: string;
  description: string | null;
  logo: string | null;
  last_pinged_at: string | null;
  status: number;
  locked: number;
  created_at: string;
  updated_at: string;
};

type ConfirmAction = "delete" | "unlock" | null;

type EditForm = {
  name: string;
  url: string;
  description: string;
  apiKey: string;
};

export function ModelFormCard({
  onSaved,
  onCancel,
}: {
  onSaved: (model: Model) => void;
  onCancel: () => void;
}) {
  const [name, setName] = useState("");
  const [url, setUrl] = useState("");
  const [apiKey, setApiKey] = useState("");
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Provider is derived from the URL, not typed by the user.
  const provider = getModelProvider(url);

  async function handleSave() {
    if (!name.trim() || !url.trim() || !apiKey.trim()) {
      setError("Name, URL and API key are required");
      return;
    }
    if (!provider) {
      setError("Could not detect a provider from this URL");
      return;
    }
    setSaving(true);
    setError(null);
    try {
      const model = await invoke<Model>("model_create", {
        input: {
          name: name.trim(),
          url: url.trim(),
          api_key: apiKey,
          provider,
        },
      });
      onSaved(model);
    } catch (e) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="card card-border border-base-300 bg-base-100 shadow-sm">
      <div className="card-body gap-3">
        <h2 className="card-title text-base">Add a model</h2>

        <label className="floating-label">
          <span>Name</span>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Name"
            className="input input-md w-full"
            disabled={saving}
          />
        </label>
        <label className="floating-label">
          <span>URL</span>
          <input
            type="text"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            placeholder="https://integrate.api.nvidia.com/v1"
            className="input input-md w-full"
            disabled={saving}
          />
        </label>
        <label className="floating-label">
          <span>API key</span>
          <input
            type="password"
            value={apiKey}
            onChange={(e) => setApiKey(e.target.value)}
            placeholder="API key"
            className="input input-md w-full"
            disabled={saving}
          />
        </label>

        <div className="flex items-center gap-2 text-sm text-base-content/60">
          Provider
          {provider ? (
            <span className="badge badge-outline badge-sm">{provider}</span>
          ) : (
            <span>detected from the URL</span>
          )}
        </div>

        {error && <p className="text-error text-sm">{error}</p>}

        <div className="card-actions justify-end mt-1">
          <button className="btn" onClick={onCancel} disabled={saving}>
            Cancel
          </button>
          <button
            className="btn btn-primary"
            onClick={handleSave}
            disabled={saving || !provider}
          >
            {saving && <span className="loading loading-spinner loading-xs" />}
            Save
          </button>
        </div>
      </div>
    </div>
  );
}

export function ModelCard({
  model,
  onChanged,
}: {
  model: Model;
  onChanged: () => void;
}) {
  const [editing, setEditing] = useState(false);
  const [form, setForm] = useState<EditForm>({
    name: "",
    url: "",
    description: "",
    apiKey: "",
  });
  const [confirm, setConfirm] = useState<ConfirmAction>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function run(action: () => Promise<void>) {
    setBusy(true);
    setError(null);
    try {
      await action();
      onChanged();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }

  function startEdit() {
    setForm({
      name: model.name,
      url: model.url,
      description: model.description ?? "",
      apiKey: "",
    });
    setError(null);
    setConfirm(null);
    setEditing(true);
  }

  function handleSaveEdit() {
    const provider = getModelProvider(form.url);
    if (!form.name.trim() || !form.url.trim()) {
      setError("Name and URL are required");
      return;
    }
    if (!provider) {
      setError("Could not detect a provider from this URL");
      return;
    }
    run(async () => {
      await invoke<Model>("model_update", {
        id: model.id,
        input: {
          name: form.name.trim(),
          url: form.url.trim(),
          provider,
          // Empty description clears the optional field.
          description: form.description.trim() || null,
          ...(form.apiKey ? { api_key: form.apiKey } : {}),
        },
      });
      setEditing(false);
    });
  }

  function handleLockToggle() {
    if (model.locked === 1) {
      // Unlocking removes deletion protection: ask first.
      setConfirm("unlock");
      return;
    }
    run(() =>
      invoke<Model>("model_set_locked", { id: model.id, locked: 1 }).then(
        () => undefined,
      ),
    );
  }

  function handleConfirm() {
    if (confirm === "delete") {
      // Backend refuses deleting locked models unless forced; we already
      // confirmed with the user.
      run(async () => {
        await invoke<boolean>("model_delete", {
          id: model.id,
          force: model.locked === 1,
        });
      });
    } else if (confirm === "unlock") {
      run(async () => {
        await invoke<Model>("model_set_locked", {
          id: model.id,
          locked: 0,
        });
      });
    }
    setConfirm(null);
  }

  if (editing) {
    const provider = getModelProvider(form.url);
    return (
      <div className="card card-border border-primary/40 bg-base-100 shadow-sm">
        <div className="card-body gap-3">
          <h2 className="card-title text-base">Edit model</h2>

          <label className="floating-label">
            <span>Name</span>
            <input
              type="text"
              value={form.name}
              onChange={(e) => setForm({ ...form, name: e.target.value })}
              placeholder="Name"
              className="input input-md w-full"
              disabled={busy}
            />
          </label>
          <label className="floating-label">
            <span>URL</span>
            <input
              type="text"
              value={form.url}
              onChange={(e) => setForm({ ...form, url: e.target.value })}
              placeholder="URL"
              className="input input-md w-full"
              disabled={busy}
            />
          </label>
          <label className="floating-label">
            <span>Description</span>
            <textarea
              value={form.description}
              onChange={(e) => setForm({ ...form, description: e.target.value })}
              placeholder="Description"
              className="textarea textarea-md w-full"
              rows={2}
              disabled={busy}
            />
          </label>
          <label className="floating-label">
            <span>New API key</span>
            <input
              type="password"
              value={form.apiKey}
              onChange={(e) => setForm({ ...form, apiKey: e.target.value })}
              placeholder="Leave blank to keep current"
              className="input input-md w-full"
              disabled={busy}
            />
          </label>

          <div className="flex items-center gap-2 text-sm text-base-content/60">
            Provider
            {provider ? (
              <span className="badge badge-outline badge-sm">{provider}</span>
            ) : (
              <span className="text-warning">unknown for this URL</span>
            )}
          </div>

          {error && <p className="text-error text-sm">{error}</p>}

          <div className="card-actions justify-end mt-1">
            <button
              className="btn"
              onClick={() => setEditing(false)}
              disabled={busy}
            >
              Cancel
            </button>
            <button
              className="btn btn-primary"
              onClick={handleSaveEdit}
              disabled={busy || !provider}
            >
              {busy && <span className="loading loading-spinner loading-xs" />}
              Save changes
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="card card-border border-base-300 bg-base-100 shadow-sm transition-shadow hover:shadow-md">
      <div className="card-body gap-2">
        <div className="flex items-center justify-between gap-2">
          <h2 className="card-title truncate">{model.name}</h2>
          <div className="flex items-center gap-1.5">
            {model.locked === 1 && (
              <Lock className="h-4 w-4 text-base-content/50" aria-label="locked" />
            )}
            <span
              className={`status ${model.status === 1 ? "status-success" : "status-error"}`}
              title={model.status === 1 ? "active" : "inactive"}
            />
          </div>
        </div>
        <div className="flex items-center gap-2 min-w-0">
          <span className="badge badge-outline badge-sm">{model.provider}</span>
          <span className="truncate text-sm text-base-content/60">{model.url}</span>
        </div>
        {model.description && (
          <p className="text-sm line-clamp-2 text-base-content/70">{model.description}</p>
        )}
        <p className="text-xs text-base-content/50">
          Last pinged: {model.last_pinged_at ?? "never"}
        </p>

        {confirm && (
          <div role="alert" className={`alert ${confirm === "delete" ? "alert-error" : "alert-warning"} py-2 text-sm`}>
            <span>
              {confirm === "delete"
                ? "Delete this model permanently?"
                : "Unlock this model? It will become deletable."}
            </span>
            <div className="flex items-center gap-1">
              <button
                className="btn btn-xs"
                onClick={() => setConfirm(null)}
                disabled={busy}
              >
                Cancel
              </button>
              <button
                className={`btn btn-xs ${confirm === "delete" ? "btn-error" : "btn-warning"}`}
                onClick={handleConfirm}
                disabled={busy}
              >
                {busy && <span className="loading loading-spinner loading-xs" />}
                {confirm === "delete" ? "Delete" : "Unlock"}
              </button>
            </div>
          </div>
        )}

        {!confirm && error && <p className="text-error text-sm">{error}</p>}

        <div className="card-actions justify-end mt-1">
          <button className="btn btn-ghost btn-xs" onClick={startEdit} disabled={busy}>
            <Pencil className="h-3.5 w-3.5" />
            Edit
          </button>
          <button className="btn btn-ghost btn-xs" onClick={handleLockToggle} disabled={busy}>
            {model.locked === 1 ? (
              <>
                <LockOpen className="h-3.5 w-3.5" />
                Unlock
              </>
            ) : (
              <>
                <Lock className="h-3.5 w-3.5" />
                Lock
              </>
            )}
          </button>
          <button
            className="btn btn-ghost btn-xs text-error"
            onClick={() => setConfirm("delete")}
            disabled={busy}
          >
            <Trash2 className="h-3.5 w-3.5" />
            Delete
          </button>
        </div>
      </div>
    </div>
  );
}
