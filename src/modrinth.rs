use crate::error::{self, AxolotlError};
use modrinth_api::{
  apis::{self, configuration, versions_api::CreateVersionError, ResponseContent},
  models,
};
use reqwest::multipart::Part;

pub async fn create_version(
  configuration: &configuration::Configuration,
  data: models::CreatableVersion,
  file: Part,
) -> Result<models::Version, error::AxolotlError> {
  let local_var_configuration = configuration;

  let local_var_client = &local_var_configuration.client;

  let local_var_uri_str = format!("{}/version", local_var_configuration.base_path);
  let mut local_var_req_builder =
    local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

  if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
    local_var_req_builder =
      local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
  }
  if let Some(ref local_var_apikey) = local_var_configuration.api_key {
    let local_var_key = local_var_apikey.key.clone();
    let local_var_value = match local_var_apikey.prefix {
      Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
      None => local_var_key,
    };
    local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
  };
  let mut local_var_form = reqwest::multipart::Form::new();
  local_var_form = local_var_form
    .text("data", serde_json::to_string(&data)?)
    .part("file", file);
  local_var_req_builder = local_var_req_builder.multipart(local_var_form);

  let local_var_req = local_var_req_builder.build()?;
  let local_var_resp = local_var_client.execute(local_var_req).await?;

  let local_var_status = local_var_resp.status();
  let local_var_content = local_var_resp.text().await?;

  if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
    serde_json::from_str(&local_var_content)
      .map_err(|error| AxolotlError::ModrinthCreateVersionError(apis::Error::Serde(error)))
  } else {
    let local_var_entity: Option<CreateVersionError> =
      serde_json::from_str(&local_var_content).ok();
    let local_var_error = ResponseContent {
      status: local_var_status,
      content: local_var_content,
      entity: local_var_entity,
    };

    Err(AxolotlError::ModrinthCreateVersionError(
      apis::Error::ResponseError(local_var_error),
    ))
  }
}
