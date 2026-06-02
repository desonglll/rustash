import { useEffect, useState } from "react";
import { fetchTags, createTag, deleteTag } from "@/utils/api";
import type { Tag, TagCreate } from "@/types";

export default function TagsPage() {
  const [tags, setTags] = useState<Tag[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showCreate, setShowCreate] = useState(false);
  const [newName, setNewName] = useState("");
  const [newDesc, setNewDesc] = useState("");
  const [creating, setCreating] = useState(false);

  function loadTags() {
    setLoading(true);
    fetchTags()
      .then(setTags)
      .catch((e) => setError(String(e)))
      .finally(() => setLoading(false));
  }

  useEffect(() => { loadTags(); }, []);

  async function handleCreate(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    if (!newName.trim()) return;
    setCreating(true);
    try {
      const input: TagCreate = {
        name: newName.trim(),
        description: newDesc.trim() || undefined,
      };
      await createTag(input);
      setNewName("");
      setNewDesc("");
      setShowCreate(false);
      loadTags();
    } catch (e) {
      setError(String(e));
    } finally {
      setCreating(false);
    }
  }

  async function handleDelete(id: number) {
    if (!confirm("Delete this tag?")) return;
    await deleteTag(id);
    loadTags();
  }

  return (
    <div className="p-6">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Tags</h1>
        <button
          className="px-3 py-1.5 rounded-lg bg-blue-600 hover:bg-blue-700
                     text-white text-sm"
          onClick={() => setShowCreate(!showCreate)}
        >
          {showCreate ? "Cancel" : "New Tag"}
        </button>
      </div>

      {error && <p className="text-red-500 mb-4">{error}</p>}

      {showCreate && (
        <form onSubmit={handleCreate} className="mb-6 p-4 bg-white dark:bg-gray-800 rounded-lg">
          <div className="flex gap-3">
            <input
              type="text"
              placeholder="Tag name"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              className="flex-1 px-3 py-2 rounded border border-gray-300
                         dark:border-gray-600 bg-white dark:bg-gray-800"
              autoFocus
            />
            <input
              type="text"
              placeholder="Description (optional)"
              value={newDesc}
              onChange={(e) => setNewDesc(e.target.value)}
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

      {!loading && tags.length > 0 && (
        <div className="flex flex-wrap gap-2">
          {tags.map((tag) => (
            <span
              key={tag.id}
              className="inline-flex items-center gap-1.5 px-3 py-1.5
                         bg-gray-200 dark:bg-gray-700 rounded-full text-sm
                         hover:bg-gray-300 dark:hover:bg-gray-600 group"
            >
              {tag.favorite && <span className="text-yellow-500">★</span>}
              <span>{tag.name}</span>
              <button
                onClick={() => handleDelete(tag.id)}
                className="text-gray-400 hover:text-red-500 opacity-0
                           group-hover:opacity-100 transition-opacity"
                aria-label={`Delete ${tag.name}`}
              >
                ×
              </button>
            </span>
          ))}
        </div>
      )}

      {!loading && tags.length === 0 && (
        <p className="text-gray-500">No tags yet. Create one to get started.</p>
      )}
    </div>
  );
}
