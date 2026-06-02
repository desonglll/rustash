import { useEffect, useState } from "react";
import { fetchStudios, createStudio, deleteStudio } from "@/utils/api";
import type { Studio, StudioCreate } from "@/types";

export default function StudiosPage() {
  const [studios, setStudios] = useState<Studio[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showCreate, setShowCreate] = useState(false);
  const [newName, setNewName] = useState("");
  const [newUrl, setNewUrl] = useState("");
  const [creating, setCreating] = useState(false);

  function loadStudios() {
    setLoading(true);
    fetchStudios()
      .then(setStudios)
      .catch((e) => setError(String(e)))
      .finally(() => setLoading(false));
  }

  useEffect(() => { loadStudios(); }, []);

  async function handleCreate(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    if (!newName.trim()) return;
    setCreating(true);
    try {
      const input: StudioCreate = {
        name: newName.trim(),
        url: newUrl.trim() || undefined,
      };
      await createStudio(input);
      setNewName("");
      setNewUrl("");
      setShowCreate(false);
      loadStudios();
    } catch (e) {
      setError(String(e));
    } finally {
      setCreating(false);
    }
  }

  async function handleDelete(id: number) {
    if (!confirm("Delete this studio?")) return;
    await deleteStudio(id);
    loadStudios();
  }

  return (
    <div className="p-6">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Studios</h1>
        <button
          className="px-3 py-1.5 rounded-lg bg-blue-600 hover:bg-blue-700
                     text-white text-sm"
          onClick={() => setShowCreate(!showCreate)}
        >
          {showCreate ? "Cancel" : "New Studio"}
        </button>
      </div>

      {error && <p className="text-red-500 mb-4">{error}</p>}

      {showCreate && (
        <form onSubmit={handleCreate} className="mb-6 p-4 bg-white dark:bg-gray-800 rounded-lg">
          <div className="flex gap-3">
            <input
              type="text"
              placeholder="Studio name"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              className="flex-1 px-3 py-2 rounded border border-gray-300
                         dark:border-gray-600 bg-white dark:bg-gray-800"
              autoFocus
            />
            <input
              type="url"
              placeholder="URL (optional)"
              value={newUrl}
              onChange={(e) => setNewUrl(e.target.value)}
              className="flex-1 px-3 py-2 rounded border border-gray-300
                         dark:border-gray-600 bg-white dark:bg-gray-800"
            />
            <button
              type="submit"
              disabled={creating || !newName.trim()}
              className="px-4 py-2 rounded bg-green-600 hover:bg-green-700
                         text-white text-sm disabled:opacity-50"
            >
              {creating ? "..." : "Create"}
            </button>
          </div>
        </form>
      )}

      {loading && <p className="text-gray-500">Loading...</p>}

      {!loading && studios.length > 0 && (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {studios.map((studio) => (
            <div
              key={studio.id}
              className="bg-white dark:bg-gray-800 rounded-lg p-4 shadow
                         hover:shadow-lg transition-shadow group"
            >
              <div className="flex items-start justify-between">
                <div>
                  <h3 className="font-medium">{studio.name}</h3>
                  {studio.url && (
                    <a
                      href={studio.url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-xs text-blue-500 hover:underline"
                    >
                      {studio.url}
                    </a>
                  )}
                  {studio.rating != null && (
                    <span className="text-xs text-yellow-500 ml-2">★ {studio.rating}</span>
                  )}
                </div>
                <button
                  onClick={() => handleDelete(studio.id)}
                  className="text-gray-400 hover:text-red-500 opacity-0
                             group-hover:opacity-100 transition-opacity text-sm"
                >
                  ×
                </button>
              </div>
            </div>
          ))}
        </div>
      )}

      {!loading && studios.length === 0 && (
        <p className="text-gray-500">No studios yet. Create one to get started.</p>
      )}
    </div>
  );
}
