// FFmpeg module for video metadata extraction
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub duration: Option<f64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub codec: Option<String>,
    pub bitrate: Option<i64>,
}

/// Get video metadata using ffprobe
pub fn get_video_metadata(path: &str) -> Result<VideoMetadata, String> {
    // Check if ffprobe is available
    let ffprobe_output = Command::new("ffprobe")
        .args([
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
            path
        ])
        .output();

    match ffprobe_output {
        Ok(output) => {
            if !output.status.success() {
                return Ok(VideoMetadata {
                    duration: None,
                    width: None,
                    height: None,
                    codec: None,
                    bitrate: None,
                });
            }

            let json_str = String::from_utf8_lossy(&output.stdout);
            let json: serde_json::Value = serde_json::from_str(&json_str)
                .map_err(|e| e.to_string())?;

            // Extract duration
            let duration = json.get("format")
                .and_then(|f| f.get("duration"))
                .and_then(|d| d.as_str())
                .and_then(|d| d.parse::<f64>().ok());

            // Extract video stream info
            let (width, height, codec) = json.get("streams")
                .and_then(|s| s.as_array())
                .and_then(|streams| streams.iter().find(|s| s.get("codec_type").and_then(|c| c.as_str()) == Some("video")))
                .map(|stream| {
                    let w = stream.get("width").and_then(|w| w.as_i64()).map(|w| w as i32);
                    let h = stream.get("height").and_then(|h| h.as_i64()).map(|h| h as i32);
                    let c = stream.get("codec_name").and_then(|c| c.as_str()).map(|c| c.to_string());
                    (w, h, c)
                })
                .unwrap_or((None, None, None));

            // Extract bitrate
            let bitrate = json.get("format")
                .and_then(|f| f.get("bit_rate"))
                .and_then(|b| b.as_str())
                .and_then(|b| b.parse::<i64>().ok());

            log::info!("Got metadata for {}: {}x{}, duration: {:?}", path, width.unwrap_or(0), height.unwrap_or(0), duration);

            Ok(VideoMetadata {
                duration,
                width,
                height,
                codec,
                bitrate,
            })
        }
        Err(e) => {
            log::warn!("ffprobe not available: {}", e);
            Ok(VideoMetadata {
                duration: None,
                width: None,
                height: None,
                codec: None,
                bitrate: None,
            })
        }
    }
}

/// Check if ffmpeg/ffprobe is available
pub fn is_ffmpeg_available() -> bool {
    Command::new("ffprobe")
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}