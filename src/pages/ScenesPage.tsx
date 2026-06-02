import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import SceneCard from "@/components/Scenes/SceneCard";
import { fetchScenes } from "@/utils/api";
import type { Scene, PaginatedResult } from "@/types";

export default function ScenesPage() {
  const navigate = useNavigate();
  const [data, setData] = useState<PaginatedResult<Scene> | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [page, setPage] = useState(1);
  const perPage = 25;

  useEffect(() => {
    setLoading(true);
    setError(null);
    fetchScenes(page, perPage)
      .then(setData)
      .catch((e) => setError(String(e)))
      .finally(() => setLoading(false));
  }, [page]);

  const totalPages = data ? Math.ceil(data.total / perPage) : 0;

  return (
    <div className="p-6">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Scenes</h1>
        {data && (
          <span className="text-sm text-gray-500">
            {data.total} scene{data.total !== 1 ? "s" : ""}
          </span>
        )}
      </div>

      {loading && <p className="text-gray-500">Loading...</p>}
      {error && <p className="text-red-500">{error}</p>}

      {data && data.items.length > 0 && (
        <>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
            {data.items.map((scene) => (
              <SceneCard
                key={scene.id}
                scene={scene}
                onClick={() => navigate(`/scenes/${scene.id}`)}
              />
            ))}
          </div>

          {totalPages > 1 && (
            <div className="flex items-center justify-center gap-2 mt-6">
              <button
                className="px-3 py-1 rounded bg-gray-200 dark:bg-gray-700
                           disabled:opacity-50 text-sm"
                disabled={page <= 1}
                onClick={() => setPage((p) => p - 1)}
              >
                Previous
              </button>
              <span className="text-sm text-gray-500">
                Page {page} of {totalPages}
              </span>
              <button
                className="px-3 py-1 rounded bg-gray-200 dark:bg-gray-700
                           disabled:opacity-50 text-sm"
                disabled={page >= totalPages}
                onClick={() => setPage((p) => p + 1)}
              >
                Next
              </button>
            </div>
          )}
        </>
      )}

      {data && data.items.length === 0 && (
        <p className="text-gray-500">No scenes found. Try scanning your library.</p>
      )}
    </div>
  );
}
