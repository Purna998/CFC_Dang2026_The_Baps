//! Organization route handlers

use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use eemp_domain::TenantId;
use eemp_error::Result;
use eemp_organization_service::{dto::*, OrganizationService};
use serde::Deserialize;
use std::str::FromStr;

use crate::{extractors::ValidatedJson, state::AppState};

/// Create organization routes
pub fn organization_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_organization))
        .route("/", get(list_organizations))
        .route("/:tenant_id", get(get_organization))
        .route("/:tenant_id", put(update_organization))
        .route("/:tenant_id", delete(delete_organization))
        .route("/domain/:domain", get(get_organization_by_domain))
        .route("/subdomain/:subdomain", get(get_organization_by_subdomain))
}

#[derive(Deserialize)]
struct PaginationQuery {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
}

fn default_limit() -> i64 {
    20
}

/// Create organization handler
///
/// POST /api/v1/organizations
async fn create_organization(
    State(state): State<AppState>,
    ValidatedJson(req): ValidatedJson<CreateOrganizationRequest>,
) -> Result<Json<CreateOrganizationResponse>> {
    let org_service = OrganizationService::new(state.db.clone());
    let response = org_service.create_organization(req).await?;
    Ok(Json(response))
}

/// Get organization handler
///
/// GET /api/v1/organizations/:tenant_id
async fn get_organization(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> Result<Json<OrganizationResponse>> {
    let tenant_id = TenantId::from_str(&tenant_id)
        .map_err(|e| eemp_error::AppError::ValidationError(format!("Invalid tenant ID: {}", e)))?;

    let org_service = OrganizationService::new(state.db.clone());
    let response = org_service.get_organization(tenant_id).await?;
    Ok(Json(response))
}

/// Get organization by domain handler
///
/// GET /api/v1/organizations/domain/:domain
async fn get_organization_by_domain(
    State(state): State<AppState>,
    Path(domain): Path<String>,
) -> Result<Json<OrganizationResponse>> {
    let org_service = OrganizationService::new(state.db.clone());
    let response = org_service.get_organization_by_domain(&domain).await?;
    Ok(Json(response))
}

/// Get organization by subdomain handler
///
/// GET /api/v1/organizations/subdomain/:subdomain
async fn get_organization_by_subdomain(
    State(state): State<AppState>,
    Path(subdomain): Path<String>,
) -> Result<Json<OrganizationResponse>> {
    let org_service = OrganizationService::new(state.db.clone());
    let response = org_service
        .get_organization_by_subdomain(&subdomain)
        .await?;
    Ok(Json(response))
}

/// Update organization handler
///
/// PUT /api/v1/organizations/:tenant_id
async fn update_organization(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
    ValidatedJson(req): ValidatedJson<UpdateOrganizationRequest>,
) -> Result<Json<OrganizationResponse>> {
    let tenant_id = TenantId::from_str(&tenant_id)
        .map_err(|e| eemp_error::AppError::ValidationError(format!("Invalid tenant ID: {}", e)))?;

    let org_service = OrganizationService::new(state.db.clone());
    let response = org_service.update_organization(tenant_id, req).await?;
    Ok(Json(response))
}

/// List organizations handler
///
/// GET /api/v1/organizations?limit=20&offset=0
async fn list_organizations(
    State(state): State<AppState>,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<OrganizationListResponse>> {
    let org_service = OrganizationService::new(state.db.clone());
    let response = org_service
        .list_organizations(params.limit, params.offset)
        .await?;
    Ok(Json(response))
}

/// Delete organization handler
///
/// DELETE /api/v1/organizations/:tenant_id
async fn delete_organization(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> Result<Json<()>> {
    let tenant_id = TenantId::from_str(&tenant_id)
        .map_err(|e| eemp_error::AppError::ValidationError(format!("Invalid tenant ID: {}", e)))?;

    let org_service = OrganizationService::new(state.db.clone());
    org_service.delete_organization(tenant_id).await?;
    Ok(Json(()))
}
