import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Video {
  id: number;
  title: string;
  path: string;
  duration: number | null;
  width: number | null;
  height: number | null;
  size: number | null;
  created_at: string;
  updated_at: string;
}

function App() {
  const [videos, setVideos] = useState<Video[]>([]);
  const [loading, setLoading] = useState(true);
  const [showAddForm, setShowAddForm] = useState(false);
  const [newVideo, setNewVideo] = useState({ title: "", path: "" });

  // Fetch videos on mount
  useEffect(() => {
    loadVideos();
  }, []);

  const loadVideos = async () => {
    try {
      const data = await invoke<Video[]>("get_videos");
      setVideos(data);
    } catch (err) {
      console.error("Failed to load videos:", err);
    } finally {
      setLoading(false);
    }
  };

  const handleAddVideo = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await invoke("add_video", {
        input: {
          title: newVideo.title,
          path: newVideo.path,
        },
      });
      setNewVideo({ title: "", path: "" });
      setShowAddForm(false);
      loadVideos();
    } catch (err) {
      console.error("Failed to add video:", err);
    }
  };

  const handleDeleteVideo = async (id: number) => {
    try {
      await invoke("delete_video", { id });
      loadVideos();
    } catch (err) {
      console.error("Failed to delete video:", err);
    }
  };

  const formatSize = (bytes: number | null) => {
    if (!bytes) return "-";
    const mb = bytes / (1024 * 1024);
    return mb.toFixed(1) + " MB";
  };

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100">
      {/* Header */}
      <header className="bg-gray-800 border-b border-gray-700">
        <div className="max-w-7xl mx-auto px-4 py-4 flex items-center justify-between">
          <h1 className="text-2xl font-bold text-blue-400">rustash</h1>
          <button
            onClick={() => setShowAddForm(!showAddForm)}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
          >
            {showAddForm ? "Cancel" : "+ Add Video"}
          </button>
        </div>
      </header>

      {/* Add Form */}
      {showAddForm && (
        <div className="max-w-7xl mx-auto px-4 py-4">
          <form
            onSubmit={handleAddVideo}
            className="bg-gray-800 rounded-lg p-4 flex gap-4 items-end"
          >
            <div className="flex-1">
              <label className="block text-sm text-gray-400 mb-1">Title</label>
              <input
                type="text"
                value={newVideo.title}
                onChange={(e) => setNewVideo({ ...newVideo, title: e.target.value })}
                className="w-full px-3 py-2 bg-gray-700 rounded border border-gray-600 focus:border-blue-500 outline-none"
                required
              />
            </div>
            <div className="flex-1">
              <label className="block text-sm text-gray-400 mb-1">File Path</label>
              <input
                type="text"
                value={newVideo.path}
                onChange={(e) => setNewVideo({ ...newVideo, path: e.target.value })}
                className="w-full px-3 py-2 bg-gray-700 rounded border border-gray-600 focus:border-blue-500 outline-none"
                placeholder="/path/to/video.mp4"
                required
              />
            </div>
            <button
              type="submit"
              className="px-6 py-2 bg-green-600 hover:bg-green-700 rounded-lg transition-colors"
            >
              Add
            </button>
          </form>
        </div>
      )}

      {/* Video List */}
      <main className="max-w-7xl mx-auto px-4 py-6">
        {loading ? (
          <div className="text-center py-12 text-gray-500">Loading...</div>
        ) : videos.length === 0 ? (
          <div className="text-center py-12 text-gray-500">
            No videos yet. Click "Add Video" to get started.
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {videos.map((video) => (
              <div
                key={video.id}
                className="bg-gray-800 rounded-lg p-4 border border-gray-700 hover:border-gray-600 transition-colors"
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1 min-w-0">
                    <h3 className="text-lg font-semibold truncate">{video.title}</h3>
                    <p className="text-sm text-gray-400 truncate">{video.path}</p>
                  </div>
                  <button
                    onClick={() => handleDeleteVideo(video.id)}
                    className="ml-2 p-1 text-gray-500 hover:text-red-400 transition-colors"
                    title="Delete"
                  >
                    ✕
                  </button>
                </div>
                <div className="mt-3 flex gap-4 text-sm text-gray-500">
                  <span>{formatSize(video.size)}</span>
                  {video.width && video.height && (
                    <span>{video.width} × {video.height}</span>
                  )}
                  {video.duration && (
                    <span>{Math.floor(video.duration / 60)}:{String(Math.floor(video.duration % 60)).padStart(2, "0")}</span>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}

export default App;