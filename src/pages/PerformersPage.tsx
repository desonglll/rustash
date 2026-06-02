import { useEffect, useState } from "react";
import { fetchPerformers, createPerformer, deletePerformer } from "@/utils/api";
import type { Performer, PerformerCreate, PaginatedResult } from "@/types";

export default function PerformersPage() {
  const [data, setData] = useState<PaginatedResult<Performer> | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [page, setPage] = useState(1);
  const [showCreate, setShowCreate] = useState(false);
  const [newName, setNewName] = useState("");
  const [creating, setCreating] = useState(false);
  const perPage = 25;

  function loadPerformers() {
    setLoading(true);
    fetchPerformers(page, perPage)
      .then(setData)
      .catch((e) => setError(String(e)))
      .finally(() => setLoading(false));
  }

  useEffect(() => { loadPerformers(); }, [page]);

  async function handleCreate(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    if (!newName.trim()) return;
    setCreating(true);
    try {
      const input: PerformerCreate = { name: newName.trim() };
      await createPerformer(input);
      setNewName("");
      setShowCreate(false);
      loadPerformers();
    } catch (e) {
      setError(String(e));
    } finally {
      setCreating(false);
    }
  }

  async function handleDelete(id: number) {
    if (!confirm("Delete this performer?")) return;
    await deletePerformer(id);
    loadPerformers();
  }

  const totalPages = data ? Math.ceil(data.total / perPage) : 0;

  return (
    <div className="p-6">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Performers</h1>
        <button
          className="px-3 py-1.5 rounded-lg bg-blue-600 hover:bg-blue-700 text-white text-sm"
          onClick={() => setShowCreate(!showCreate)}
        >
          {showCreate ? "Cancel" : "New Performer"}
        </button>
      </div>

      {error && <p className="text-red-500 mb-4">{error}</p>}

      {showCreate && (
        <form onSubmit={handleCreate} className="mb-6 p-4 bg-white dark:bg-gray-800 rounded-lg">
          <div className="flex gap-3">
            <input
              type="text"
              placeholder="Performer name"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              className="flex-1 px-3 py-2 rounded border border-gray-300
                         dark:border-gray-600 bg-white dark:bg-gray-800"
              autoFocus
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

      {data && data.items.length > 0 && (
        <>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
            {data.items.map((performer) => (
              <div
                key={performer.id}
                className="bg-white dark:bg-gray-800 rounded-lg overflow-hidden
                           shadow hover:shadow-lg transition-shadow group"
              >
                <div className="aspect-square bg-gray-200 dark:bg-gray-700
                                flex items-center justify-center">
                  <span className="text-4xl">👤</span>
                </div>
                <div className="p-3">
                  <h3 className="text-sm font-medium truncate">
                    {performer.name}
                  </h3>
                  <div className="flex items-center gap-2 mt-1 text-xs text-gray-500">
                    {performer.favorite && <span className="text-yellow-500">★</span>}
                    {performer.birthdate && <span>{performer.birthdate}</span>}
                    {performer.country && <span>{performer.country}</span>}
                  </div>
                  <button
                    onClick={() => handleDelete(performer.id)}
                    className="mt-2 text-xs text-gray-400 hover:text-red-500
                               opacity-0 group-hover:opacity-100 transition-opacity"
                  >
                    Delete
                  </button>
                </div>
              </div>
            ))}
          </div>

          {totalPages > 1 && (
            <div className="flex items-center justify-center gap-2 mt-6">
              <button
                className="px-3 py-1 rounded bg-gray-200 dark:bg-gray-700 text-sm disabled:opacity-50"
                disabled={page <= 1}
                onClick={() => setPage((p) => p - 1)}
              >
                Previous
              </button>
              <span className="text-sm text-gray-500">Page {page} of {totalPages}</span>
              <button
                className="px-3 py-1 rounded bg-gray-200 dark:bg-gray-700 text-sm disabled:opacity-50"
                disabled={page >= totalPages}
                onClick={() => setPage((p) => p + 1)}
              >
                Next
              </button>
            </div>
          )}
        </>
      )}

      {!loading && data && data.items.length === 0 && (
        <p className="text-gray-500">No performers yet. Create one to get started.</p>
      )}
    </div>
  );
}
