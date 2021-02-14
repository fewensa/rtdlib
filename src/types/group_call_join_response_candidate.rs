
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a join response candidate for interaction with tgcalls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallJoinResponseCandidate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Value of the field port
  port: String,
  /// Value of the field protocol
  protocol: String,
  /// Value of the field network
  network: String,
  /// Value of the field generation
  generation: String,
  /// Value of the field id
  id: String,
  /// Value of the field component
  component: String,
  /// Value of the field foundation
  foundation: String,
  /// Value of the field priority
  priority: String,
  /// Value of the field ip
  ip: String,
  /// Value of the field type
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: String,
  /// Value of the field tcp_type
  tcp_type: String,
  /// Value of the field rel_addr
  rel_addr: String,
  /// Value of the field rel_port
  rel_port: String,
  
}

impl RObject for GroupCallJoinResponseCandidate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCallJoinResponseCandidate" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl GroupCallJoinResponseCandidate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallJoinResponseCandidateBuilder {
    let mut inner = GroupCallJoinResponseCandidate::default();
    inner.td_name = "groupCallJoinResponseCandidate".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallJoinResponseCandidateBuilder { inner }
  }

  pub fn port(&self) -> &String { &self.port }

  pub fn protocol(&self) -> &String { &self.protocol }

  pub fn network(&self) -> &String { &self.network }

  pub fn generation(&self) -> &String { &self.generation }

  pub fn id(&self) -> &String { &self.id }

  pub fn component(&self) -> &String { &self.component }

  pub fn foundation(&self) -> &String { &self.foundation }

  pub fn priority(&self) -> &String { &self.priority }

  pub fn ip(&self) -> &String { &self.ip }

  pub fn type_(&self) -> &String { &self.type_ }

  pub fn tcp_type(&self) -> &String { &self.tcp_type }

  pub fn rel_addr(&self) -> &String { &self.rel_addr }

  pub fn rel_port(&self) -> &String { &self.rel_port }

}

#[doc(hidden)]
pub struct RTDGroupCallJoinResponseCandidateBuilder {
  inner: GroupCallJoinResponseCandidate
}

impl RTDGroupCallJoinResponseCandidateBuilder {
  pub fn build(&self) -> GroupCallJoinResponseCandidate { self.inner.clone() }

   
  pub fn port<T: AsRef<str>>(&mut self, port: T) -> &mut Self {
    self.inner.port = port.as_ref().to_string();
    self
  }

   
  pub fn protocol<T: AsRef<str>>(&mut self, protocol: T) -> &mut Self {
    self.inner.protocol = protocol.as_ref().to_string();
    self
  }

   
  pub fn network<T: AsRef<str>>(&mut self, network: T) -> &mut Self {
    self.inner.network = network.as_ref().to_string();
    self
  }

   
  pub fn generation<T: AsRef<str>>(&mut self, generation: T) -> &mut Self {
    self.inner.generation = generation.as_ref().to_string();
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn component<T: AsRef<str>>(&mut self, component: T) -> &mut Self {
    self.inner.component = component.as_ref().to_string();
    self
  }

   
  pub fn foundation<T: AsRef<str>>(&mut self, foundation: T) -> &mut Self {
    self.inner.foundation = foundation.as_ref().to_string();
    self
  }

   
  pub fn priority<T: AsRef<str>>(&mut self, priority: T) -> &mut Self {
    self.inner.priority = priority.as_ref().to_string();
    self
  }

   
  pub fn ip<T: AsRef<str>>(&mut self, ip: T) -> &mut Self {
    self.inner.ip = ip.as_ref().to_string();
    self
  }

   
  pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().to_string();
    self
  }

   
  pub fn tcp_type<T: AsRef<str>>(&mut self, tcp_type: T) -> &mut Self {
    self.inner.tcp_type = tcp_type.as_ref().to_string();
    self
  }

   
  pub fn rel_addr<T: AsRef<str>>(&mut self, rel_addr: T) -> &mut Self {
    self.inner.rel_addr = rel_addr.as_ref().to_string();
    self
  }

   
  pub fn rel_port<T: AsRef<str>>(&mut self, rel_port: T) -> &mut Self {
    self.inner.rel_port = rel_port.as_ref().to_string();
    self
  }

}

impl AsRef<GroupCallJoinResponseCandidate> for GroupCallJoinResponseCandidate {
  fn as_ref(&self) -> &GroupCallJoinResponseCandidate { self }
}

impl AsRef<GroupCallJoinResponseCandidate> for RTDGroupCallJoinResponseCandidateBuilder {
  fn as_ref(&self) -> &GroupCallJoinResponseCandidate { &self.inner }
}



