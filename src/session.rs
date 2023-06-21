use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use crate::items::FullNowPlayingItem;
use crate::items::MediaItem;
use crate::items::NowPlayingQueue;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
    #[serde(rename = "PlayState")]
    pub play_state: PlayState,
    #[serde(rename = "AdditionalUsers")]
    pub additional_users: Vec<AdditionalUser>,
    #[serde(rename = "Capabilities")]
    pub capabilities: Capabilities,
    #[serde(rename = "RemoteEndPoint")]
    pub remote_end_point: String,
    #[serde(rename = "PlayableMediaTypes")]
    pub playable_media_types: Vec<String>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "UserName")]
    pub user_name: String,
    #[serde(rename = "Client")]
    pub client: String,
    #[serde(rename = "LastActivityDate")]
    pub last_activity_date: String,
    #[serde(rename = "LastPlaybackCheckIn")]
    pub last_playback_check_in: String,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "DeviceType")]
    pub device_type: String,
    #[serde(rename = "NowPlayingItem")]
    pub now_playing_item: MediaItem,
    #[serde(rename = "FullNowPlayingItem")]
    pub full_now_playing_item: FullNowPlayingItem,
    #[serde(rename = "NowViewingItem")]
    pub now_viewing_item: MediaItem,
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    #[serde(rename = "ApplicationVersion")]
    pub application_version: String,
    #[serde(rename = "TranscodingInfo")]
    pub transcoding_info: TranscodingInfo,
    #[serde(rename = "IsActive")]
    pub is_active: bool,
    #[serde(rename = "SupportsMediaControl")]
    pub supports_media_control: bool,
    #[serde(rename = "SupportsRemoteControl")]
    pub supports_remote_control: bool,
    #[serde(rename = "NowPlayingQueue")]
    pub now_playing_queue: Vec<NowPlayingQueue>,
    #[serde(rename = "NowPlayingQueueFullItems")]
    pub now_playing_queue_full_items: Vec<MediaItem>,
    #[serde(rename = "HasCustomDeviceName")]
    pub has_custom_device_name: bool,
    #[serde(rename = "PlaylistItemId")]
    pub playlist_item_id: String,
    #[serde(rename = "ServerId")]
    pub server_id: String,
    #[serde(rename = "UserPrimaryImageTag")]
    pub user_primary_image_tag: String,
    #[serde(rename = "SupportedCommands")]
    pub supported_commands: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayState {
    #[serde(rename = "PositionTicks")]
    pub position_ticks: i64,
    #[serde(rename = "CanSeek")]
    pub can_seek: bool,
    #[serde(rename = "IsPaused")]
    pub is_paused: bool,
    #[serde(rename = "IsMuted")]
    pub is_muted: bool,
    #[serde(rename = "VolumeLevel")]
    pub volume_level: i64,
    #[serde(rename = "AudioStreamIndex")]
    pub audio_stream_index: i64,
    #[serde(rename = "SubtitleStreamIndex")]
    pub subtitle_stream_index: i64,
    #[serde(rename = "MediaSourceId")]
    pub media_source_id: String,
    #[serde(rename = "PlayMethod")]
    pub play_method: String,
    #[serde(rename = "RepeatMode")]
    pub repeat_mode: String,
    #[serde(rename = "LiveStreamId")]
    pub live_stream_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalUser {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    #[serde(rename = "PlayableMediaTypes")]
    pub playable_media_types: Vec<String>,
    #[serde(rename = "SupportedCommands")]
    pub supported_commands: Vec<String>,
    #[serde(rename = "SupportsMediaControl")]
    pub supports_media_control: bool,
    #[serde(rename = "SupportsContentUploading")]
    pub supports_content_uploading: bool,
    #[serde(rename = "MessageCallbackUrl")]
    pub message_callback_url: String,
    #[serde(rename = "SupportsPersistentIdentifier")]
    pub supports_persistent_identifier: bool,
    #[serde(rename = "SupportsSync")]
    pub supports_sync: bool,
    #[serde(rename = "DeviceProfile")]
    pub device_profile: DeviceProfile,
    #[serde(rename = "AppStoreUrl")]
    pub app_store_url: String,
    #[serde(rename = "IconUrl")]
    pub icon_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceProfile {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Identification")]
    pub identification: Identification,
    #[serde(rename = "FriendlyName")]
    pub friendly_name: String,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "ManufacturerUrl")]
    pub manufacturer_url: String,
    #[serde(rename = "ModelName")]
    pub model_name: String,
    #[serde(rename = "ModelDescription")]
    pub model_description: String,
    #[serde(rename = "ModelNumber")]
    pub model_number: String,
    #[serde(rename = "ModelUrl")]
    pub model_url: String,
    #[serde(rename = "SerialNumber")]
    pub serial_number: String,
    #[serde(rename = "EnableAlbumArtInDidl")]
    pub enable_album_art_in_didl: bool,
    #[serde(rename = "EnableSingleAlbumArtLimit")]
    pub enable_single_album_art_limit: bool,
    #[serde(rename = "EnableSingleSubtitleLimit")]
    pub enable_single_subtitle_limit: bool,
    #[serde(rename = "SupportedMediaTypes")]
    pub supported_media_types: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "AlbumArtPn")]
    pub album_art_pn: String,
    #[serde(rename = "MaxAlbumArtWidth")]
    pub max_album_art_width: i64,
    #[serde(rename = "MaxAlbumArtHeight")]
    pub max_album_art_height: i64,
    #[serde(rename = "MaxIconWidth")]
    pub max_icon_width: i64,
    #[serde(rename = "MaxIconHeight")]
    pub max_icon_height: i64,
    #[serde(rename = "MaxStreamingBitrate")]
    pub max_streaming_bitrate: i64,
    #[serde(rename = "MaxStaticBitrate")]
    pub max_static_bitrate: i64,
    #[serde(rename = "MusicStreamingTranscodingBitrate")]
    pub music_streaming_transcoding_bitrate: i64,
    #[serde(rename = "MaxStaticMusicBitrate")]
    pub max_static_music_bitrate: i64,
    #[serde(rename = "SonyAggregationFlags")]
    pub sony_aggregation_flags: String,
    #[serde(rename = "ProtocolInfo")]
    pub protocol_info: String,
    #[serde(rename = "TimelineOffsetSeconds")]
    pub timeline_offset_seconds: i64,
    #[serde(rename = "RequiresPlainVideoItems")]
    pub requires_plain_video_items: bool,
    #[serde(rename = "RequiresPlainFolders")]
    pub requires_plain_folders: bool,
    #[serde(rename = "EnableMSMediaReceiverRegistrar")]
    pub enable_msmedia_receiver_registrar: bool,
    #[serde(rename = "IgnoreTranscodeByteRangeRequests")]
    pub ignore_transcode_byte_range_requests: bool,
    #[serde(rename = "XmlRootAttributes")]
    pub xml_root_attributes: Vec<XmlRootAttribute>,
    #[serde(rename = "DirectPlayProfiles")]
    pub direct_play_profiles: Vec<DirectPlayProfile>,
    #[serde(rename = "TranscodingProfiles")]
    pub transcoding_profiles: Vec<TranscodingProfile>,
    #[serde(rename = "ContainerProfiles")]
    pub container_profiles: Vec<ContainerProfile>,
    #[serde(rename = "CodecProfiles")]
    pub codec_profiles: Vec<CodecProfile>,
    #[serde(rename = "ResponseProfiles")]
    pub response_profiles: Vec<ResponseProfile>,
    #[serde(rename = "SubtitleProfiles")]
    pub subtitle_profiles: Vec<SubtitleProfile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identification {
    #[serde(rename = "FriendlyName")]
    pub friendly_name: String,
    #[serde(rename = "ModelNumber")]
    pub model_number: String,
    #[serde(rename = "SerialNumber")]
    pub serial_number: String,
    #[serde(rename = "ModelName")]
    pub model_name: String,
    #[serde(rename = "ModelDescription")]
    pub model_description: String,
    #[serde(rename = "ModelUrl")]
    pub model_url: String,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "ManufacturerUrl")]
    pub manufacturer_url: String,
    #[serde(rename = "Headers")]
    pub headers: Vec<Header>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmlRootAttribute {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectPlayProfile {
    #[serde(rename = "Container")]
    pub container: String,
    #[serde(rename = "AudioCodec")]
    pub audio_codec: String,
    #[serde(rename = "VideoCodec")]
    pub video_codec: String,
    #[serde(rename = "Type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscodingProfile {
    #[serde(rename = "Container")]
    pub container: String,
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "VideoCodec")]
    pub video_codec: String,
    #[serde(rename = "AudioCodec")]
    pub audio_codec: String,
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "EstimateContentLength")]
    pub estimate_content_length: bool,
    #[serde(rename = "EnableMpegtsM2TsMode")]
    pub enable_mpegts_m2ts_mode: bool,
    #[serde(rename = "TranscodeSeekInfo")]
    pub transcode_seek_info: String,
    #[serde(rename = "CopyTimestamps")]
    pub copy_timestamps: bool,
    #[serde(rename = "Context")]
    pub context: String,
    #[serde(rename = "EnableSubtitlesInManifest")]
    pub enable_subtitles_in_manifest: bool,
    #[serde(rename = "MaxAudioChannels")]
    pub max_audio_channels: String,
    #[serde(rename = "MinSegments")]
    pub min_segments: i64,
    #[serde(rename = "SegmentLength")]
    pub segment_length: i64,
    #[serde(rename = "BreakOnNonKeyFrames")]
    pub break_on_non_key_frames: bool,
    #[serde(rename = "Conditions")]
    pub conditions: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerProfile {
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Conditions")]
    pub conditions: Vec<Value>,
    #[serde(rename = "Container")]
    pub container: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodecProfile {
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Conditions")]
    pub conditions: Vec<Value>,
    #[serde(rename = "ApplyConditions")]
    pub apply_conditions: Vec<Value>,
    #[serde(rename = "Codec")]
    pub codec: String,
    #[serde(rename = "Container")]
    pub container: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseProfile {
    #[serde(rename = "Container")]
    pub container: String,
    #[serde(rename = "AudioCodec")]
    pub audio_codec: String,
    #[serde(rename = "VideoCodec")]
    pub video_codec: String,
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "OrgPn")]
    pub org_pn: String,
    #[serde(rename = "MimeType")]
    pub mime_type: String,
    #[serde(rename = "Conditions")]
    pub conditions: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtitleProfile {
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "Method")]
    pub method: String,
    #[serde(rename = "DidlMode")]
    pub didl_mode: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Container")]
    pub container: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscodingInfo {
    #[serde(rename = "AudioCodec")]
    pub audio_codec: String,
    #[serde(rename = "VideoCodec")]
    pub video_codec: String,
    #[serde(rename = "Container")]
    pub container: String,
    #[serde(rename = "IsVideoDirect")]
    pub is_video_direct: bool,
    #[serde(rename = "IsAudioDirect")]
    pub is_audio_direct: bool,
    #[serde(rename = "Bitrate")]
    pub bitrate: i64,
    #[serde(rename = "Framerate")]
    pub framerate: i64,
    #[serde(rename = "CompletionPercentage")]
    pub completion_percentage: i64,
    #[serde(rename = "Width")]
    pub width: i64,
    #[serde(rename = "Height")]
    pub height: i64,
    #[serde(rename = "AudioChannels")]
    pub audio_channels: i64,
    #[serde(rename = "HardwareAccelerationType")]
    pub hardware_acceleration_type: String,
    #[serde(rename = "TranscodeReasons")]
    pub transcode_reasons: Vec<String>,
}