import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { fetchScene, updateScene, createScene } from "@/utils/api";
import type { SceneUpdate, SceneCreate } from "@/types";

export default function SceneEditPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const isCreating = !id;
  const [loading, setLoading] = useState(!isCreating);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [title, setTitle] = useState("");
  const [details, setDetails] = useState("");
  const [code, setCode] = useState("");
  const [director, setDirector] = useState("");
  const [url, setUrl] = useState("");
  const [date, setDate] = useState("");
  const [rating, setRating] = useState<number | "">("");
  const [organized, setOrganized] = useState(false);

  useEffect(() => {
    if (isCreating) return;
    fetchScene(Number(id)).then((scene) => {
      if (!scene) { setError("Not found"); setLoading(false); return; }
      setTitle(scene.title || "");
      setDetails(scene.details || "");
      setCode(scene.code || "");
      setDirector(scene.director || "");
      setUrl(scene.url || "");
      setDate(scene.date || "");
      setRating(scene.rating ?? "");
      setOrganized(scene.organized);
      setLoading(false);
    }).catch((e) => { setError(String(e)); setLoading(false); });
  }, [id, isCreating]);

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setSaving(true);
    setError(null);
    try {
      if (isCreating) {
        const input: SceneCreate = {
          title: title || undefined,
          details: details || undefined,
          code: code || undefined,
          director: director || undefined,
          url: url || undefined,
          date: date || undefined,
          rating: rating === "" ? undefined : Number(rating),
        };
        const created = await createScene(input);
        navigate(`/scenes/${created.id}`);
      } else {
        const input: SceneUpdate = {
          id: Number(id),
          title: title || undefined,
          details: details || undefined,
          code: code || undefined,
          director: director || undefined,
          url: url || undefined,
          date: date || undefined,
          rating: rating === "" ? undefined : Number(rating),
          organized,
        };
        await updateScene(input);
        navigate(`/scenes/${id}`);
      }
    } catch (e) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  }

  if (loading) return <div className="p-6 text-gray-500">Loading...</div>;

  return (
    <div className="p-6 max-w-2xl">
      <h1 className="text-2xl font-bold mb-6">
        {isCreating ? "Create Scene" : "Edit Scene"}
      </h1>

      {error && <p className="text-red-500 mb-4">{error}</p>}

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium mb-1">Title</label>
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="w-full px-3 py-2 rounded-lg border border-gray-300
                       dark:border-gray-600 bg-white dark:bg-gray-800"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">Details</label>
          <textarea
            value={details}
            onChange={(e) => setDetails(e.target.value)}
            rows={4}
            className="w-full px-3 py-2 rounded-lg border border-gray-300
                       dark:border-gray-600 bg-white dark:bg-gray-800"
          />
        </div>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium mb-1">Code</label>
            <input
              type="text"
              value={code}
              onChange={(e) => setCode(e.target.value)}
              className="w-full px-3 py-2 rounded-lg border border-gray-300
                         dark:border-gray-600 bg-white dark:bg-gray-800"
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Director</label>
            <input
              type="text"
              value={director}
              onChange={(e) => setDirector(e.target.value)}
              className="w-full px-3 py-2 rounded-lg border border-gray-300
                         dark:border-gray-600 bg-white dark:bg-gray-800"
            />
          </div>
        </div>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium mb-1">URL</label>
            <input
              type="url"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              className="w-full px-3 py-2 rounded-lg border border-gray-300
                         dark:border-gray-600 bg-white dark:bg-gray-800"
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Date</label>
            <input
              type="date"
              value={date}
              onChange={(e) => setDate(e.target.value)}
              className="w-full px-3 py-2 rounded-lg border border-gray-300
                         dark:border-gray-600 bg-white dark:bg-gray-800"
            />
          </div>
        </div>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium mb-1">Rating</label>
            <input
              type="number"
              min={1}
              max={5}
              value={rating}
              onChange={(e) => setRating(e.target.value === "" ? "" : Number(e.target.value))}
              className="w-full px-3 py-2 rounded-lg border border-gray-300
                         dark:border-gray-600 bg-white dark:bg-gray-800"
            />
          </div>
          {!isCreating && (
            <div className="flex items-end">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={organized}
                  onChange={(e) => setOrganized(e.target.checked)}
                  className="rounded"
                />
                <span className="text-sm font-medium">Organized</span>
              </label>
            </div>
          )}
        </div>
        <div className="flex gap-3 pt-4">
          <button
            type="submit"
            disabled={saving}
            className="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700
                       text-white font-medium disabled:opacity-50"
          >
            {saving ? "Saving..." : isCreating ? "Create" : "Save"}
          </button>
          <button
            type="button"
            onClick={() => navigate(-1)}
            className="px-4 py-2 rounded-lg bg-gray-200 dark:bg-gray-700
                       hover:bg-gray-300 dark:hover:bg-gray-600"
          >
            Cancel
          </button>
        </div>
      </form>
    </div>
  );
}
