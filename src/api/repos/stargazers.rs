use super::*;

#[derive(serde::Serialize)]
pub struct ListStarGazersBuilder<'octo, 'r> {
  #[serde(skip)]
  handler: &'r RepoHandler<'octo>,
  #[serde(skip_serializing_if = "Option::is_none")]
  per_page: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  page: Option<u32>,
}

impl<'octo, 'r> ListStarGazersBuilder<'octo, 'r> {
  pub fn new(handler: &'r RepoHandler<'octo>) -> Self {
    Self {
      handler,
      per_page: None,
      page: None,
    }
  }

  /// Results per page (max 100).
  pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
    self.per_page = Some(per_page.into());
    self
  }

  /// Page number of the results to fetch.
  pub fn page(mut self, page: impl Into<u32>) -> Self {
    self.page = Some(page.into());
    self
  }

  /// Sends the actual request.
  pub async fn send(self) -> crate::Result<crate::Page<crate::models::User>> {
    let url = format!(
      "repos/{owner}/{repo}/stargazers",
      owner = self.handler.owner,
      repo = self.handler.repo
    );
    self.handler.crab.get(url, Some(&self)).await
  }
}
