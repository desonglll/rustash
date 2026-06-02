import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { fetchScene, deleteScene } from "@/utils/api";
import type { Scene } from "@/types";

export default function SceneDetailPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [scene, setScene] = useState<Scene | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!id) return;
    setLoading(true);
    fetchScene(Number(id))
      .then((s) => setScene(s))
      .catch((e) => setError(String(e)))
      .finally(() => setLoading(false));
  }, [id]);

  async function handleDelete() {
    if (!scene || !confirm("Delete this scene?")) return;
    await deleteScene(scene.id);
    navigate("/scenes");
  }

  if (loading) return <div className="p-6 text-gray-500">Loading...</div>;
  if (error) return <div className="p-6 text-red-500">{error}</div>;
  if (!scene) return <div className="p-6 text-gray-500">Scene not found.</div>;

  return (
    <div className="p-6 max-w-4xl">
      <div className="flex items-start justify-between mb-4">
        <h1 className="text-2xl font-bold">{scene.title || "Untitled"}</h1>
        <div className="flex gap-2">
          <button
            className="px-3 py-1.5 rounded bg-blue-600 hover:bg-blue-700
                       text-white text-sm"
            onClick={() => navigate(`/scenes/${scene.id}/edit`)}
          >
            Edit
          </button>
          <button
            className="px-3 py-1.5 rounded bg-red-600 hover:bg-red-700
                       text-white text-sm"
            onClick={handleDelete}
          >
            Delete
          </button>
        </div>
      </div>

      {/* Video placeholder */}
      <div className="aspect-video bg-black rounded-lg mb-6 flex items-center justify-center">
        <span className="text-gray-500 text-lg">Video player coming soon</span>
      </div>

      {/* Metadata */}
      <div className="grid grid-cols-2 gap-4 text-sm">
        {scene.date && (
          <div>
            <span className="text-gray-500">Date: </span>
            <span>{scene.date}</span>
          </div>
        )}
        {scene.rating != null && (
          <div>
            <span className="text-gray-500">Rating: </span>
            <span className="text-yellow-500">★ {scene.rating}</span>
          </div>
        )}
        {scene.studio_id != null && (
          <div>
            <span className="text-gray-500">Studio: </span>
            <span>ID {scene.studio_id}</span>
          </div>
        )}
        <div>
          <span className="text-gray-500">Organized: </span>
          <span>{scene.organized ? "Yes" : "No"}</span>
        </div>
        {scene.code && (
          <div>
            <span className="text-gray-500">Code: </span>
            <span>{scene.code}</span>
          </div>
        )}
        {scene.director && (
          <div>
            <span className="text-gray-500">Director: </span>
            <span>{scene.director}</span>
          </div>
        )}
      </div>

      {scene.details && (
        <div className="mt-6">
          <h2 className="text-lg font-semibold mb-2">Details</h2>
          <p className="text-sm text-gray-700 dark:text-gray-300 whitespace-pre-wrap">
            {scene.details}
          </p>
        </div>
      )}

      {scene.url && (
        <div className="mt-4">
          <a
            href={scene.url}
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-500 hover:underline text-sm"
          >
            {scene.url}
          </a>
        </div>
      )}
    </div>
  );
}
