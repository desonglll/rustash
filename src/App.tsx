import { useState, useEffect, useRef } from "react";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import Player from "xgplayer";

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

interface ScannedFile {
  path: string;
  name: string;
  size: number;
}

interface VideoMetadata {
  duration: number | null;
  width: number | null;
  height: number | null;
  codec: string | null;
  bitrate: number | null;
}

interface Settings {
  ffmpegPath: string;
}

function App() {
  const [videos, setVideos] = useState<Video[]>([]);
  const [loading, setLoading] = useState(true);
  const [showAddForm, setShowAddForm] = useState(false);
  const [showScanForm, setShowScanForm] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [currentVideo, setCurrentVideo] = useState<Video | null>(null);
  const [hasFfmpeg, setHasFfmpeg] = useState(false);
  const [settings, setSettings] = useState<Settings>({
    ffmpegPath: "",
  });
  const playerRef = useRef<HTMLDivElement>(null);
  const playerInstanceRef = useRef<Player | null>(null);
  const [newVideo, setNewVideo] = useState({ title: "", path: "" });
  const [scanPath, setScanPath] = useState("");
  const [scannedFiles, setScannedFiles] = useState<ScannedFile[]>([]);
  const [scanning, setScanning] = useState(false);
  const [importing, setImporting] = useState(false);
  const [playerError, setPlayerError] = useState<string | null>(null);

  useEffect(() => {
    loadVideos();
    checkFfmpeg();
    loadSettings();
  }, []);

  // Cleanup player on unmount
  useEffect(() => {
    return () => {
      if (playerInstanceRef.current) {
        playerInstanceRef.current.destroy();
      }
    };
  }, []);

  // Initialize player when currentVideo changes
  useEffect(() => {
    if (currentVideo && playerRef.current) {
      // Destroy existing player
      if (playerInstanceRef.current) {
        playerInstanceRef.current.destroy();
        playerInstanceRef.current = null;
      }

      setPlayerError(null);

      // Use convertFileSrc for Tauri
      const videoUrl = convertFileSrc(currentVideo.path);
      console.log("Playing video:", videoUrl);

      try {
        playerInstanceRef.current = new Player({
          el: playerRef.current,
          url: videoUrl,
          autoplay: true,
          playsinline: true,
          height: "100%",
          width: "100%",
          lang: "en",
          volume: 0.8,
          controls: [
            "play",
            "time",
            "volume",
            "fullscreen"
          ],
          errorTips: "Unable to play video. Please check if the file format is supported.",
        });

        playerInstanceRef.current.on("error", (err: any) => {
          console.error("Player error:", err);
          setPlayerError("Playback error. Format may not be supported.");
        });
      } catch (err) {
        console.error("Failed to create player:", err);
        setPlayerError("Failed to initialize player.");
      }
    }
  }, [currentVideo]);

  const loadSettings = async () => {
    try {
      const savedSettings = await invoke<Settings>("get_settings");
      if (savedSettings) {
        setSettings(savedSettings);
      }
    } catch (err) {
      console.log("No saved settings, using defaults");
    }
  };

  const saveSettings = async () => {
    try {
      await invoke("save_settings", { settings });
      setShowSettings(false);
      // Re-check ffmpeg with new path
      checkFfmpeg();
    } catch (err) {
      console.error("Failed to save settings:", err);
    }
  };

  const checkFfmpeg = async () => {
    try {
      const available = await invoke<boolean>("check_ffmpeg");
      setHasFfmpeg(available);
    } catch (err) {
      console.error("Failed to check ffmpeg:", err);
      setHasFfmpeg(false);
    }
  };

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

  const getVideoMetadata = async (path: string): Promise<VideoMetadata | null> => {
    try {
      return await invoke<VideoMetadata>("get_video_metadata", { path });
    } catch (err) {
      console.error("Failed to get metadata:", err);
      return null;
    }
  };

  const handleAddVideo = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      let metadata: VideoMetadata | null = null;
      if (hasFfmpeg) {
        metadata = await getVideoMetadata(newVideo.path);
      }

      await invoke("add_video", {
        input: {
          title: newVideo.title,
          path: newVideo.path,
          duration: metadata?.duration ?? null,
          width: metadata?.width ?? null,
          height: metadata?.height ?? null,
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

  const handlePlayVideo = (video: Video) => {
    setCurrentVideo(video);
  };

  const handleClosePlayer = () => {
    if (playerInstanceRef.current) {
      playerInstanceRef.current.destroy();
      playerInstanceRef.current = null;
    }
    setCurrentVideo(null);
    setPlayerError(null);
  };

  const handleSelectFfmpeg = async () => {
    try {
      const selected = await open({
        directory: false,
        multiple: false,
        filters: [
          {
            name: "Executable",
            extensions: process.platform === "win32" ? ["exe"] : [],
          },
        ],
      });
      if (selected) {
        setSettings({ ...settings, ffmpegPath: selected as string });
      }
    } catch (err) {
      console.error("Failed to select ffmpeg:", err);
    }
  };

  const handleSelectFolder = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });
      if (selected) {
        setScanPath(selected as string);
      }
    } catch (err) {
      console.error("Failed to select folder:", err);
    }
  };

  const handleScan = async () => {
    if (!scanPath) return;
    setScanning(true);
    try {
      const result = await invoke<{ files: ScannedFile[]; total: number }>("scan_directory", {
        path: scanPath,
      });
      setScannedFiles(result.files);
    } catch (err) {
      console.error("Failed to scan directory:", err);
    } finally {
      setScanning(false);
    }
  };

  const handleImport = async () => {
    if (scannedFiles.length === 0) return;
    setImporting(true);
    try {
      const paths = scannedFiles.map((f) => f.path);
      await invoke("import_scanned_files", { files: paths });
      setScannedFiles([]);
      setScanPath("");
      setShowScanForm(false);
      loadVideos();
    } catch (err) {
      console.error("Failed to import files:", err);
    } finally {
      setImporting(false);
    }
  };

  const toggleFile = (path: string) => {
    setScannedFiles((prev) => {
      if (prev.some((f) => f.path === path)) {
        return prev.filter((f) => f.path !== path);
      } else {
        const file = scannedFiles.find((f) => f.path === path);
        return file ? [...prev, file] : prev;
      }
    });
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
          <div className="flex gap-2">
            <button
              onClick={() => setShowSettings(!showSettings)}
              className="px-4 py-2 bg-gray-600 hover:bg-gray-500 rounded-lg transition-colors"
            >
              ⚙️ Settings
            </button>
            <button
              onClick={() => setShowScanForm(!showScanForm)}
              className="px-4 py-2 bg-purple-600 hover:bg-purple-700 rounded-lg transition-colors"
            >
              {showScanForm ? "Cancel" : "📁 Scan"}
            </button>
            <button
              onClick={() => setShowAddForm(!showAddForm)}
              className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
            >
              {showAddForm ? "Cancel" : "+ Add"}
            </button>
          </div>
        </div>
      </header>

      {/* Settings Modal */}
      {showSettings && (
        <div className="max-w-7xl mx-auto px-4 py-4">
          <div className="bg-gray-800 rounded-lg p-6 max-w-xl">
            <h2 className="text-xl font-semibold mb-4">Settings</h2>

            <div className="space-y-4">
              {/* FFmpeg Path */}
              <div>
                <label className="block text-sm text-gray-400 mb-1">FFmpeg Path</label>
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={settings.ffmpegPath}
                    onChange={(e) => setSettings({ ...settings, ffmpegPath: e.target.value })}
                    className="flex-1 px-3 py-2 bg-gray-700 rounded border border-gray-600 focus:border-blue-500 outline-none"
                    placeholder="Auto-detect if empty"
                  />
                  <button
                    onClick={handleSelectFfmpeg}
                    className="px-4 py-2 bg-gray-600 hover:bg-gray-500 rounded transition-colors"
                  >
                    Browse
                  </button>
                </div>
                <p className="text-xs text-gray-500 mt-1">
                  {hasFfmpeg ? "✓ FFmpeg is available" : "✗ FFmpeg not found"}
                </p>
              </div>

              <div className="flex justify-end gap-2 pt-4">
                <button
                  onClick={() => setShowSettings(false)}
                  className="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors"
                >
                  Cancel
                </button>
                <button
                  onClick={saveSettings}
                  className="px-4 py-2 bg-green-600 hover:bg-green-700 rounded-lg transition-colors"
                >
                  Save
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Video Player Modal */}
      {currentVideo && (
        <div className="fixed inset-0 bg-black/90 z-50 flex items-center justify-center">
          <div className="w-full max-w-5xl mx-4">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-xl font-semibold">{currentVideo.title}</h2>
              <button
                onClick={handleClosePlayer}
                className="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg transition-colors"
              >
                ✕ Close
              </button>
            </div>
            {playerError ? (
              <div className="w-full aspect-video bg-gray-800 rounded-lg flex items-center justify-center">
                <div className="text-center">
                  <p className="text-red-400 text-lg mb-2">Playback Error</p>
                  <p className="text-gray-400">{playerError}</p>
                  <p className="text-gray-500 text-sm mt-2">Path: {currentVideo.path}</p>
                </div>
              </div>
            ) : (
              <div
                ref={playerRef}
                className="w-full aspect-video bg-black rounded-lg overflow-hidden"
              />
            )}
          </div>
        </div>
      )}

      {/* Scan Form */}
      {showScanForm && (
        <div className="max-w-7xl mx-auto px-4 py-4">
          <div className="bg-gray-800 rounded-lg p-4">
            <div className="flex gap-4 items-end mb-4">
              <div className="flex-1">
                <label className="block text-sm text-gray-400 mb-1">Folder Path</label>
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={scanPath}
                    onChange={(e) => setScanPath(e.target.value)}
                    className="flex-1 px-3 py-2 bg-gray-700 rounded border border-gray-600 focus:border-blue-500 outline-none"
                    placeholder="/path/to/videos"
                  />
                  <button
                    onClick={handleSelectFolder}
                    className="px-4 py-2 bg-gray-600 hover:bg-gray-500 rounded transition-colors"
                  >
                    Browse
                  </button>
                  <button
                    onClick={handleScan}
                    disabled={!scanPath || scanning}
                    className="px-4 py-2 bg-purple-600 hover:bg-purple-700 disabled:opacity-50 rounded transition-colors"
                  >
                    {scanning ? "Scanning..." : "Scan"}
                  </button>
                </div>
              </div>
            </div>

            {scannedFiles.length > 0 && (
              <div>
                <div className="flex items-center justify-between mb-2">
                  <span className="text-sm text-gray-400">
                    Found {scannedFiles.length} video files
                  </span>
                  <button
                    onClick={handleImport}
                    disabled={importing}
                    className="px-4 py-2 bg-green-600 hover:bg-green-700 disabled:opacity-50 rounded transition-colors"
                  >
                    {importing ? "Importing..." : `Import (${scannedFiles.length})`}
                  </button>
                </div>
                <div className="max-h-60 overflow-y-auto bg-gray-900 rounded p-2">
                  {scannedFiles.map((file) => (
                    <div
                      key={file.path}
                      className="flex items-center gap-2 py-1 hover:bg-gray-800 rounded px-2 cursor-pointer"
                      onClick={() => toggleFile(file.path)}
                    >
                      <input type="checkbox" checked={true} onChange={() => {}} className="accent-blue-500" />
                      <span className="truncate flex-1">{file.name}</span>
                      <span className="text-gray-500 text-sm">{formatSize(file.size)}</span>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Add Form */}
      {showAddForm && (
        <div className="max-w-7xl mx-auto px-4 py-4">
          <form onSubmit={handleAddVideo} className="bg-gray-800 rounded-lg p-4 flex gap-4 items-end">
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
            <button type="submit" className="px-6 py-2 bg-green-600 hover:bg-green-700 rounded-lg transition-colors">
              Add
            </button>
          </form>
        </div>
      )}

      {/* ffmpeg warning */}
      {!hasFfmpeg && (
        <div className="max-w-7xl mx-auto px-4 py-2">
          <div className="bg-yellow-900/50 border border-yellow-700 rounded-lg p-3 text-yellow-200 text-sm">
            ⚠️ FFmpeg not found. Click Settings to configure the path for metadata extraction.
          </div>
        </div>
      )}

      {/* Video List */}
      <main className="max-w-7xl mx-auto px-4 py-6">
        {loading ? (
          <div className="text-center py-12 text-gray-500">Loading...</div>
        ) : videos.length === 0 ? (
          <div className="text-center py-12 text-gray-500">
            No videos yet. Click "Scan" or "Add" to get started.
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {videos.map((video) => (
              <div key={video.id} className="bg-gray-800 rounded-lg p-4 border border-gray-700 hover:border-gray-600 transition-colors">
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
                <div className="mt-3 flex items-center gap-4 text-sm text-gray-500">
                  <span>{formatSize(video.size)}</span>
                  {video.width && video.height && <span>{video.width} × {video.height}</span>}
                  {video.duration && <span>{Math.floor(video.duration / 60)}:{String(Math.floor(video.duration % 60)).padStart(2, "0")}</span>}
                </div>
                <button
                  onClick={() => handlePlayVideo(video)}
                  className="mt-3 w-full py-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors flex items-center justify-center gap-2"
                >
                  ▶ Play
                </button>
              </div>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}

export default App;