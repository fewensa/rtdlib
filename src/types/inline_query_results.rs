use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResults {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the inline query
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  inline_query_id: isize,
  /// Button to be shown above inline query results; may be null
  button: Option<InlineQueryResultsButton>,
  /// Results of the query
  results: Vec<InlineQueryResult>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for InlineQueryResults {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineQueryResults"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InlineQueryResults {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineQueryResultsBuilder {
    let inner = InlineQueryResults {
      td_name: "inlineQueryResults".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineQueryResultsBuilder { inner }
  }

  pub fn inline_query_id(&self) -> isize {
    self.inline_query_id
  }

  pub fn button(&self) -> &Option<InlineQueryResultsButton> {
    &self.button
  }

  pub fn results(&self) -> &Vec<InlineQueryResult> {
    &self.results
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultsBuilder {
  inner: InlineQueryResults,
}

impl RTDInlineQueryResultsBuilder {
  pub fn build(&self) -> InlineQueryResults {
    self.inner.clone()
  }

  pub fn inline_query_id(&mut self, inline_query_id: isize) -> &mut Self {
    self.inner.inline_query_id = inline_query_id;
    self
  }

  pub fn button<T: AsRef<InlineQueryResultsButton>>(&mut self, button: T) -> &mut Self {
    self.inner.button = Some(button.as_ref().clone());
    self
  }

  pub fn results(&mut self, results: Vec<InlineQueryResult>) -> &mut Self {
    self.inner.results = results;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<InlineQueryResults> for InlineQueryResults {
  fn as_ref(&self) -> &InlineQueryResults {
    self
  }
}

impl AsRef<InlineQueryResults> for RTDInlineQueryResultsBuilder {
  fn as_ref(&self) -> &InlineQueryResults {
    &self.inner
  }
}
