import type { Scene } from "@/types";

interface SceneCardProps {
  scene: Scene;
  onClick?: (scene: Scene) => void;
}

export default function SceneCard({ scene, onClick }: SceneCardProps) {
  return (
    <button
      type="button"
      onClick={() => onClick?.(scene)}
      className="bg-white dark:bg-gray-800 rounded-lg shadow hover:shadow-lg
                 transition-shadow text-left w-full overflow-hidden group"
    >
      <div className="aspect-video bg-gray-200 dark:bg-gray-700 flex items-center justify-center">
        <span className="text-gray-400 text-4xl">🎬</span>
      </div>
      <div className="p-3">
        <h3 className="text-sm font-medium truncate group-hover:text-blue-500">
          {scene.title || "Untitled"}
        </h3>
        <div className="flex items-center gap-2 mt-1 text-xs text-gray-500">
          {scene.date && <span>{scene.date}</span>}
          {scene.rating != null && (
            <span className="text-yellow-500">★ {scene.rating}</span>
          )}
        </div>
      </div>
    </button>
  );
}
