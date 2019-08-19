
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a game. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // game
  /// Game ID.
  id: Option<i64>,
  /// Game short name. To share a game use the URL https://t.me/{bot_username}?game={game_short_name}.
  short_name: Option<String>,
  /// Game title.
  title: Option<String>,
  /// Game text, usually containing scoreboards for a game.
  text: Option<FormattedText>,
  /// Game description.
  description: Option<String>,
  /// Game photo.
  photo: Option<Photo>,
  /// Game animation; may be null.
  animation: Option<Animation>,
  
}



impl Object for Game {}
impl RObject for Game {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "game" }
  fn td_type(&self) -> RTDType { RTDType::Game }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Game {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "game".to_string(),
      id: None,
      short_name: None,
      title: None,
      text: None,
      description: None,
      photo: None,
      animation: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn short_name(&self) -> Option<String> { self.short_name.clone() }
  #[doc(hidden)] pub fn _set_short_name(&mut self, short_name: String) -> &mut Self { self.short_name = Some(short_name); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn text(&self) -> Option<FormattedText> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: FormattedText) -> &mut Self { self.text = Some(text); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn photo(&self) -> Option<Photo> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: Photo) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn animation(&self) -> Option<Animation> { self.animation.clone() }
  #[doc(hidden)] pub fn _set_animation(&mut self, animation: Animation) -> &mut Self { self.animation = Some(animation); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



