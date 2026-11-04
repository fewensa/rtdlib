use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Story {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique story identifier among stories posted by the given chat
  id: i64,
  /// Identifier of the chat that posted the story
  poster_chat_id: i64,
  /// Identifier of the user or chat that posted the story; may be null if the story is posted on behalf of the poster_chat_id
  poster_id: Option<MessageSender>,
  /// Point in time (Unix timestamp) when the story was published
  date: i64,
  /// True, if the story is being posted by the current user
  is_being_posted: bool,
  /// True, if the story is being edited by the current user
  is_being_edited: bool,
  /// True, if the story was edited
  is_edited: bool,
  /// True, if the story is saved in the profile of the chat that posted it and will be available there after expiration
  is_posted_to_chat_page: bool,
  /// True, if the story is visible only for the current user
  is_visible_only_for_self: bool,
  /// True, if the story can be added to an album using createStoryAlbum and addStoryAlbumStories
  can_be_added_to_album: bool,
  /// True, if the story can be deleted
  can_be_deleted: bool,
  /// True, if the story can be edited
  can_be_edited: bool,
  /// True, if the story can be forwarded as a message or reposted as a story. Otherwise, screenshotting and saving of the story content must be also forbidden
  can_be_forwarded: bool,
  /// True, if the story can be replied in the chat with the user who posted the story
  can_be_replied: bool,
  /// True, if the story privacy settings can be changed
  can_set_privacy_settings: bool,
  /// True, if the story's is_posted_to_chat_page value can be changed
  can_toggle_is_posted_to_chat_page: bool,
  /// True, if the story statistics are available through getStoryStatistics
  can_get_statistics: bool,
  /// True, if interactions with the story can be received through getStoryInteractions
  can_get_interactions: bool,
  /// True, if users who viewed the story can't be received, because the story has expired more than getOption("story_viewers_expiration_delay") seconds ago
  has_expired_viewers: bool,
  /// Information about the original story; may be null if the story wasn't reposted
  repost_info: Option<StoryRepostInfo>,
  /// Information about interactions with the story; may be null if the story isn't owned or there were no interactions
  interaction_info: Option<StoryInteractionInfo>,
  /// Type of the chosen reaction; may be null if none
  chosen_reaction_type: Option<ReactionType>,
  /// Privacy rules affecting story visibility; may be approximate for non-owned stories
  privacy_settings: StoryPrivacySettings,
  /// Content of the story
  content: StoryContent,
  /// Clickable areas to be shown on the story content
  areas: Vec<StoryArea>,
  /// Caption of the story
  caption: FormattedText,
  /// Identifiers of story albums to which the story is added; only for manageable stories
  album_ids: Vec<i64>,
}

impl RObject for Story {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "story"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Story {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryBuilder {
    let inner = Story {
      td_name: "story".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn poster_chat_id(&self) -> i64 {
    self.poster_chat_id
  }

  pub fn poster_id(&self) -> &Option<MessageSender> {
    &self.poster_id
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn is_being_posted(&self) -> bool {
    self.is_being_posted
  }

  pub fn is_being_edited(&self) -> bool {
    self.is_being_edited
  }

  pub fn is_edited(&self) -> bool {
    self.is_edited
  }

  pub fn is_posted_to_chat_page(&self) -> bool {
    self.is_posted_to_chat_page
  }

  pub fn is_visible_only_for_self(&self) -> bool {
    self.is_visible_only_for_self
  }

  pub fn can_be_added_to_album(&self) -> bool {
    self.can_be_added_to_album
  }

  pub fn can_be_deleted(&self) -> bool {
    self.can_be_deleted
  }

  pub fn can_be_edited(&self) -> bool {
    self.can_be_edited
  }

  pub fn can_be_forwarded(&self) -> bool {
    self.can_be_forwarded
  }

  pub fn can_be_replied(&self) -> bool {
    self.can_be_replied
  }

  pub fn can_set_privacy_settings(&self) -> bool {
    self.can_set_privacy_settings
  }

  pub fn can_toggle_is_posted_to_chat_page(&self) -> bool {
    self.can_toggle_is_posted_to_chat_page
  }

  pub fn can_get_statistics(&self) -> bool {
    self.can_get_statistics
  }

  pub fn can_get_interactions(&self) -> bool {
    self.can_get_interactions
  }

  pub fn has_expired_viewers(&self) -> bool {
    self.has_expired_viewers
  }

  pub fn repost_info(&self) -> &Option<StoryRepostInfo> {
    &self.repost_info
  }

  pub fn interaction_info(&self) -> &Option<StoryInteractionInfo> {
    &self.interaction_info
  }

  pub fn chosen_reaction_type(&self) -> &Option<ReactionType> {
    &self.chosen_reaction_type
  }

  pub fn privacy_settings(&self) -> &StoryPrivacySettings {
    &self.privacy_settings
  }

  pub fn content(&self) -> &StoryContent {
    &self.content
  }

  pub fn areas(&self) -> &Vec<StoryArea> {
    &self.areas
  }

  pub fn caption(&self) -> &FormattedText {
    &self.caption
  }

  pub fn album_ids(&self) -> &Vec<i64> {
    &self.album_ids
  }
}

#[doc(hidden)]
pub struct RTDStoryBuilder {
  inner: Story,
}

impl RTDStoryBuilder {
  pub fn build(&self) -> Story {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn poster_chat_id(&mut self, poster_chat_id: i64) -> &mut Self {
    self.inner.poster_chat_id = poster_chat_id;
    self
  }

  pub fn poster_id<T: AsRef<MessageSender>>(&mut self, poster_id: T) -> &mut Self {
    self.inner.poster_id = Some(poster_id.as_ref().clone());
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn is_being_posted(&mut self, is_being_posted: bool) -> &mut Self {
    self.inner.is_being_posted = is_being_posted;
    self
  }

  pub fn is_being_edited(&mut self, is_being_edited: bool) -> &mut Self {
    self.inner.is_being_edited = is_being_edited;
    self
  }

  pub fn is_edited(&mut self, is_edited: bool) -> &mut Self {
    self.inner.is_edited = is_edited;
    self
  }

  pub fn is_posted_to_chat_page(&mut self, is_posted_to_chat_page: bool) -> &mut Self {
    self.inner.is_posted_to_chat_page = is_posted_to_chat_page;
    self
  }

  pub fn is_visible_only_for_self(&mut self, is_visible_only_for_self: bool) -> &mut Self {
    self.inner.is_visible_only_for_self = is_visible_only_for_self;
    self
  }

  pub fn can_be_added_to_album(&mut self, can_be_added_to_album: bool) -> &mut Self {
    self.inner.can_be_added_to_album = can_be_added_to_album;
    self
  }

  pub fn can_be_deleted(&mut self, can_be_deleted: bool) -> &mut Self {
    self.inner.can_be_deleted = can_be_deleted;
    self
  }

  pub fn can_be_edited(&mut self, can_be_edited: bool) -> &mut Self {
    self.inner.can_be_edited = can_be_edited;
    self
  }

  pub fn can_be_forwarded(&mut self, can_be_forwarded: bool) -> &mut Self {
    self.inner.can_be_forwarded = can_be_forwarded;
    self
  }

  pub fn can_be_replied(&mut self, can_be_replied: bool) -> &mut Self {
    self.inner.can_be_replied = can_be_replied;
    self
  }

  pub fn can_set_privacy_settings(&mut self, can_set_privacy_settings: bool) -> &mut Self {
    self.inner.can_set_privacy_settings = can_set_privacy_settings;
    self
  }

  pub fn can_toggle_is_posted_to_chat_page(
    &mut self,
    can_toggle_is_posted_to_chat_page: bool,
  ) -> &mut Self {
    self.inner.can_toggle_is_posted_to_chat_page = can_toggle_is_posted_to_chat_page;
    self
  }

  pub fn can_get_statistics(&mut self, can_get_statistics: bool) -> &mut Self {
    self.inner.can_get_statistics = can_get_statistics;
    self
  }

  pub fn can_get_interactions(&mut self, can_get_interactions: bool) -> &mut Self {
    self.inner.can_get_interactions = can_get_interactions;
    self
  }

  pub fn has_expired_viewers(&mut self, has_expired_viewers: bool) -> &mut Self {
    self.inner.has_expired_viewers = has_expired_viewers;
    self
  }

  pub fn repost_info<T: AsRef<StoryRepostInfo>>(&mut self, repost_info: T) -> &mut Self {
    self.inner.repost_info = Some(repost_info.as_ref().clone());
    self
  }

  pub fn interaction_info<T: AsRef<StoryInteractionInfo>>(
    &mut self,
    interaction_info: T,
  ) -> &mut Self {
    self.inner.interaction_info = Some(interaction_info.as_ref().clone());
    self
  }

  pub fn chosen_reaction_type<T: AsRef<ReactionType>>(
    &mut self,
    chosen_reaction_type: T,
  ) -> &mut Self {
    self.inner.chosen_reaction_type = Some(chosen_reaction_type.as_ref().clone());
    self
  }

  pub fn privacy_settings<T: AsRef<StoryPrivacySettings>>(
    &mut self,
    privacy_settings: T,
  ) -> &mut Self {
    self.inner.privacy_settings = privacy_settings.as_ref().clone();
    self
  }

  pub fn content<T: AsRef<StoryContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }

  pub fn areas(&mut self, areas: Vec<StoryArea>) -> &mut Self {
    self.inner.areas = areas;
    self
  }

  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

  pub fn album_ids(&mut self, album_ids: Vec<i64>) -> &mut Self {
    self.inner.album_ids = album_ids;
    self
  }
}

impl AsRef<Story> for Story {
  fn as_ref(&self) -> &Story {
    self
  }
}

impl AsRef<Story> for RTDStoryBuilder {
  fn as_ref(&self) -> &Story {
    &self.inner
  }
}
