
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a web page preview. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebPage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // webPage
  /// Original URL of the link.
  url: Option<String>,
  /// URL to display.
  display_url: Option<String>,
  /// Type of the web page. Can be: article, photo, audio, video, document, profile, app, or something else.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<String>,
  /// Short name of the site (e.g., Google Docs, App Store).
  site_name: Option<String>,
  /// Title of the content.
  title: Option<String>,
  /// Description of the content.
  description: Option<String>,
  /// Image representing the content; may be null.
  photo: Option<Photo>,
  /// URL to show in the embedded preview.
  embed_url: Option<String>,
  /// MIME type of the embedded preview, (e.g., text/html or video/mp4).
  embed_type: Option<String>,
  /// Width of the embedded preview.
  embed_width: Option<i32>,
  /// Height of the embedded preview.
  embed_height: Option<i32>,
  /// Duration of the content, in seconds.
  duration: Option<i32>,
  /// Author of the content.
  author: Option<String>,
  /// Preview of the content as an animation, if available; may be null.
  animation: Option<Animation>,
  /// Preview of the content as an audio file, if available; may be null.
  audio: Option<Audio>,
  /// Preview of the content as a document, if available (currently only available for small PDF files and ZIP archives); may be null.
  document: Option<Document>,
  /// Preview of the content as a sticker for small WEBP files, if available; may be null.
  sticker: Option<Sticker>,
  /// Preview of the content as a video, if available; may be null.
  video: Option<Video>,
  /// Preview of the content as a video note, if available; may be null.
  video_note: Option<VideoNote>,
  /// Preview of the content as a voice note, if available; may be null.
  voice_note: Option<VoiceNote>,
  /// Version of instant view, available for the web page (currently can be 1 or 2), 0 if none.
  instant_view_version: Option<i32>,
  
}



impl Object for WebPage {}
impl RObject for WebPage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "webPage" }
  fn td_type(&self) -> RTDType { RTDType::WebPage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl WebPage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "webPage".to_string(),
      url: None,
      display_url: None,
      type_: None,
      site_name: None,
      title: None,
      description: None,
      photo: None,
      embed_url: None,
      embed_type: None,
      embed_width: None,
      embed_height: None,
      duration: None,
      author: None,
      animation: None,
      audio: None,
      document: None,
      sticker: None,
      video: None,
      video_note: None,
      voice_note: None,
      instant_view_version: None,
      
    }
  }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn display_url(&self) -> Option<String> { self.display_url.clone() }
  #[doc(hidden)] pub fn _set_display_url(&mut self, display_url: String) -> &mut Self { self.display_url = Some(display_url); self }
  
  pub fn type_(&self) -> Option<String> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: String) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn site_name(&self) -> Option<String> { self.site_name.clone() }
  #[doc(hidden)] pub fn _set_site_name(&mut self, site_name: String) -> &mut Self { self.site_name = Some(site_name); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn photo(&self) -> Option<Photo> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: Photo) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn embed_url(&self) -> Option<String> { self.embed_url.clone() }
  #[doc(hidden)] pub fn _set_embed_url(&mut self, embed_url: String) -> &mut Self { self.embed_url = Some(embed_url); self }
  
  pub fn embed_type(&self) -> Option<String> { self.embed_type.clone() }
  #[doc(hidden)] pub fn _set_embed_type(&mut self, embed_type: String) -> &mut Self { self.embed_type = Some(embed_type); self }
  
  pub fn embed_width(&self) -> Option<i32> { self.embed_width.clone() }
  #[doc(hidden)] pub fn _set_embed_width(&mut self, embed_width: i32) -> &mut Self { self.embed_width = Some(embed_width); self }
  
  pub fn embed_height(&self) -> Option<i32> { self.embed_height.clone() }
  #[doc(hidden)] pub fn _set_embed_height(&mut self, embed_height: i32) -> &mut Self { self.embed_height = Some(embed_height); self }
  
  pub fn duration(&self) -> Option<i32> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn author(&self) -> Option<String> { self.author.clone() }
  #[doc(hidden)] pub fn _set_author(&mut self, author: String) -> &mut Self { self.author = Some(author); self }
  
  pub fn animation(&self) -> Option<Animation> { self.animation.clone() }
  #[doc(hidden)] pub fn _set_animation(&mut self, animation: Animation) -> &mut Self { self.animation = Some(animation); self }
  
  pub fn audio(&self) -> Option<Audio> { self.audio.clone() }
  #[doc(hidden)] pub fn _set_audio(&mut self, audio: Audio) -> &mut Self { self.audio = Some(audio); self }
  
  pub fn document(&self) -> Option<Document> { self.document.clone() }
  #[doc(hidden)] pub fn _set_document(&mut self, document: Document) -> &mut Self { self.document = Some(document); self }
  
  pub fn sticker(&self) -> Option<Sticker> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: Sticker) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn video(&self) -> Option<Video> { self.video.clone() }
  #[doc(hidden)] pub fn _set_video(&mut self, video: Video) -> &mut Self { self.video = Some(video); self }
  
  pub fn video_note(&self) -> Option<VideoNote> { self.video_note.clone() }
  #[doc(hidden)] pub fn _set_video_note(&mut self, video_note: VideoNote) -> &mut Self { self.video_note = Some(video_note); self }
  
  pub fn voice_note(&self) -> Option<VoiceNote> { self.voice_note.clone() }
  #[doc(hidden)] pub fn _set_voice_note(&mut self, voice_note: VoiceNote) -> &mut Self { self.voice_note = Some(voice_note); self }
  
  pub fn instant_view_version(&self) -> Option<i32> { self.instant_view_version.clone() }
  #[doc(hidden)] pub fn _set_instant_view_version(&mut self, instant_view_version: i32) -> &mut Self { self.instant_view_version = Some(instant_view_version); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



