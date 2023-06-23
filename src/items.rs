use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaItem {
    pub name: String,    
    pub original_title: String,    
    pub server_id: String,    
    pub id: String,    
    pub etag: String,    
    pub source_type: String,    
    pub playlist_item_id: Option<String>,    
    pub date_created: String,    
    pub date_last_media_added: String,    
    pub extra_type: String,    
    pub airs_before_season_number: i64,    
    pub airs_after_season_number: i64,    
    pub airs_before_episode_number: i64,    
    pub can_delete: bool,    
    pub can_download: bool,    
    pub has_subtitles: bool,    
    pub preferred_metadata_language: String,    
    pub preferred_metadata_country_code: String,    
    pub supports_sync: bool,    
    pub container: String,    
    pub sort_name: String,    
    pub forced_sort_name: String,    
    pub video3dformat: String,    
    pub premiere_date: String,    
    pub external_urls: Vec<ExternalUrl>,    
    pub media_sources: Vec<MediaSource>,    
    pub critic_rating: i64,    
    pub production_locations: Vec<String>,    
    pub path: String,    
    pub enable_media_source_display: bool,    
    pub official_rating: String,    
    pub custom_rating: String,    
    pub channel_id: String,    
    pub channel_name: String,    
    pub overview: String,    
    pub taglines: Vec<String>,    
    pub genres: Vec<String>,    
    pub community_rating: i64,    
    pub cumulative_run_time_ticks: i64,    
    pub run_time_ticks: i64,    
    pub play_access: String,    
    pub aspect_ratio: String,    
    pub production_year: i64,    
    pub is_place_holder: bool,    
    pub number: String,    
    pub channel_number: String,    
    pub index_number: i64,    
    pub index_number_end: i64,    
    pub parent_index_number: i64,    
    pub remote_trailers: Vec<RemoteTrailer>,    
    pub provider_ids: ProviderIds,    
    pub is_hd: bool,    
    pub is_folder: bool,    
    pub parent_id: String,    
    pub type_field: String,    
    pub people: Vec<People>,    
    pub studios: Vec<Studio>,    
    pub genre_items: Vec<GenreItem>,    
    pub parent_logo_item_id: String,    
    pub parent_backdrop_item_id: String,    
    pub parent_backdrop_image_tags: Vec<String>,    
    pub local_trailer_count: i64,    
    pub user_data: UserData,    
    pub recursive_item_count: i64,    
    pub child_count: i64,    
    pub series_name: String,    
    pub series_id: String,    
    pub season_id: String,    
    pub special_feature_count: i64,    
    pub display_preferences_id: String,    
    pub status: String,    
    pub air_time: String,    
    pub air_days: Vec<String>,    
    pub tags: Vec<String>,    
    pub primary_image_aspect_ratio: Option<i64>,    
    pub artists: Vec<String>,    
    pub artist_items: Vec<ArtistItem>,    
    pub album: String,    
    pub collection_type: String,    
    pub display_order: String,    
    pub album_id: String,    
    pub album_primary_image_tag: String,    
    pub series_primary_image_tag: String,    
    pub album_artist: String,    
    pub album_artists: Vec<AlbumArtist>,    
    pub season_name: String,    
    pub media_streams: Vec<MediaStream>,    
    pub video_type: String,    
    pub part_count: i64,    
    pub media_source_count: i64,    
    pub image_tags: ImageTags,    
    pub backdrop_image_tags: Vec<String>,    
    pub screenshot_image_tags: Vec<String>,    
    pub parent_logo_image_tag: String,    
    pub parent_art_item_id: String,    
    pub parent_art_image_tag: String,    
    pub series_thumb_image_tag: String,    
    pub image_blur_hashes: ImageBlurHashes,    
    pub series_studio: String,    
    pub parent_thumb_item_id: String,    
    pub parent_thumb_image_tag: String,    
    pub parent_primary_image_item_id: String,    
    pub parent_primary_image_tag: String,    
    pub chapters: Vec<Chapter>,    
    pub location_type: String,    
    pub iso_type: String,    
    pub media_type: String,    
    pub end_date: String,    
    pub locked_fields: Vec<String>,    
    pub trailer_count: i64,    
    pub movie_count: i64,    
    pub series_count: i64,    
    pub program_count: i64,    
    pub episode_count: i64,    
    pub song_count: i64,    
    pub album_count: i64,    
    pub artist_count: i64,    
    pub music_video_count: i64,    
    pub lock_data: bool,    
    pub width: i64,    
    pub height: i64,    
    pub camera_make: String,    
    pub camera_model: String,    
    pub software: String,    
    pub exposure_time: i64,    
    pub focal_length: i64,    
    pub image_orientation: String,    
    pub aperture: i64,    
    pub shutter_speed: i64,    
    pub latitude: i64,    
    pub longitude: i64,    
    pub altitude: i64,    
    pub iso_speed_rating: i64,    
    pub series_timer_id: String,    
    pub program_id: String,    
    pub channel_primary_image_tag: String,    
    pub start_date: String,    
    pub completion_percentage: i64,    
    pub is_repeat: bool,    
    pub episode_title: String,    
    pub channel_type: String,    
    pub audio: String,    
    pub is_movie: bool,    
    pub is_sports: bool,    
    pub is_series: bool,    
    pub is_live: bool,    
    pub is_news: bool,    
    pub is_kids: bool,    
    pub is_premiere: bool,    
    pub timer_id: String,    
    pub current_program: CurrentProgram
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CurrentProgram;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalUrl {
    pub name: String,    
    pub url: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaSource {
    pub protocol: String,    
    pub id: String,    
    pub path: String,    
    pub encoder_path: String,    
    pub encoder_protocol: String,    
    pub type_field: String,    
    pub container: String,    
    pub size: i64,    
    pub name: String,    
    pub is_remote: bool,    
    pub etag: String,    
    pub run_time_ticks: i64,    
    pub read_at_native_framerate: bool,    
    pub ignore_dts: bool,    
    pub ignore_index: bool,    
    pub gen_pts_input: bool,    
    pub supports_transcoding: bool,    
    pub supports_direct_stream: bool,    
    pub supports_direct_play: bool,    
    pub is_infinite_stream: bool,    
    pub requires_opening: bool,    
    pub open_token: String,    
    pub requires_closing: bool,    
    pub live_stream_id: Option<String>,    
    pub buffer_ms: i64,    
    pub requires_looping: bool,    
    pub supports_probing: bool,    
    pub video_type: String,    
    pub iso_type: String,    
    pub video3dformat: String,    
    pub media_streams: Vec<MediaStream>,    
    pub media_attachments: Vec<MediaAttachment>,    
    pub formats: Vec<String>,    
    pub bitrate: i64,    
    pub timestamp: String,    
    pub required_http_headers: RequiredHttpHeaders,    
    pub transcoding_url: String,    
    pub transcoding_sub_protocol: String,    
    pub transcoding_container: String,    
    pub analyze_duration_ms: i64,    
    pub default_audio_stream_index: i64,    
    pub default_subtitle_stream_index: i64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaStream {
    pub codec: String,    
    pub codec_tag: String,    
    pub language: String,    
    pub color_range: String,    
    pub color_space: String,    
    pub color_transfer: String,    
    pub color_primaries: String,    
    pub dv_version_major: i64,    
    pub dv_version_minor: i64,    
    pub dv_profile: i64,    
    pub dv_level: i64,    
    pub rpu_present_flag: i64,    
    pub el_present_flag: i64,    
    pub bl_present_flag: i64,    
    pub dv_bl_signal_compatibility_id: i64,    
    pub comment: String,    
    pub time_base: String,    
    pub codec_time_base: String,    
    pub title: String,    
    pub video_range: String,    
    pub video_range_type: String,    
    pub video_do_vi_title: String,    
    pub localized_undefined: String,    
    pub localized_default: String,    
    pub localized_forced: String,    
    pub localized_external: String,    
    pub display_title: String,    
    pub nal_length_size: String,    
    pub is_interlaced: bool,    
    pub is_avc: bool,    
    pub channel_layout: String,    
    pub bit_rate: i64,    
    pub bit_depth: i64,    
    pub ref_frames: i64,    
    pub packet_length: i64,    
    pub channels: i64,    
    pub sample_rate: i64,    
    pub is_default: bool,    
    pub is_forced: bool,    
    pub height: i64,    
    pub width: i64,    
    pub average_frame_rate: i64,    
    pub real_frame_rate: i64,    
    pub profile: String,    
    pub type_field: String,    
    pub aspect_ratio: String,    
    pub index: i64,    
    pub score: i64,    
    pub is_external: bool,    
    pub delivery_method: String,    
    pub delivery_url: String,    
    pub is_external_url: bool,    
    pub is_text_subtitle_stream: bool,    
    pub supports_external_stream: bool,    
    pub path: String,    
    pub pixel_format: String,    
    pub level: i64,    
    pub is_anamorphic: bool
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaAttachment {
    pub codec: String,    
    pub codec_tag: String,    
    pub comment: String,    
    pub index: i64,    
    pub file_name: String,    
    pub mime_type: String,    
    pub delivery_url: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequiredHttpHeaders {
    pub property1: String,
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteTrailer {
    pub url: String,    
    pub name: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProviderIds {
    pub property1: String,
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct People {
    pub name: String,    
    pub id: String,    
    pub role: String,    
    pub type_field: String,    
    pub primary_image_tag: Option<String>,    
    pub image_blur_hashes: ImageBlurHashes
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageBlurHashes {
    pub primary: Primary,    
    pub art: Art,    
    pub backdrop: Backdrop,    
    pub banner: Banner,    
    pub logo: Logo,    
    pub thumb: Thumb,    
    pub disc: Disc,    
    pub box_field: Box,    
    pub screenshot: Screenshot,    
    pub menu: Menu,    
    pub chapter: Chapter,    
    pub box_rear: BoxRear,    
    pub profile: Profile,}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Primary {
    pub property1: String,    
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Art {
    pub property1: String,    
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Backdrop {
    pub property1: String,    
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Banner {
    pub property1: String,    
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Logo {
    pub property1: String,    
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Thumb {
    pub property1: String,    
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Disc {
    pub property1: String,   
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Box {
    pub property1: String,    
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Screenshot {
    pub property1: String,   
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Menu {
    pub property1: String,
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Chapter {
    pub property1: String,    
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BoxRear {
    pub property1: String,    
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Profile {
    pub property1: String,    
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Studio {
    pub name: String,    
    pub id: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenreItem {
    pub name: String,    
    pub id: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserData {
    pub rating: i64,    
    pub played_percentage: i64,    
    pub unplayed_item_count: i64,    
    pub playback_position_ticks: i64,    
    pub play_count: i64,    
    pub is_favorite: bool,    
    pub likes: bool,    
    pub last_played_date: String,    
    pub played: bool,    
    pub key: String,    
    pub item_id: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArtistItem {
    pub name: String,    
    pub id: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AlbumArtist {
    pub name: String,    
    pub id: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageTags {
    pub property1: String,    
    pub property2: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FullNowPlayingItem {
    pub size: i64,    
    pub container: String,    
    pub is_hd: bool,    
    pub is_shortcut: bool,    
    pub shortcut_path: String,    
    pub width: i64,    
    pub height: i64,    
    pub extra_ids: Vec<String>,    
    pub date_last_saved: String,    
    pub remote_trailers: Vec<RemoteTrailer>,    
    pub supports_external_transfer: bool
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NowPlayingQueue {
    pub id: String,    
    pub playlist_item_id: Option<String>
}