//! Data Transfer Objects for Result Service

use eemp_domain::ElectionId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ElectionResultsResponse {
    pub election_id: ElectionId,
    pub total_ballots: i32,
    pub positions: Vec<PositionResultResponse>,
    pub calculated_at: String,
    pub published_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PositionResultResponse {
    pub position_id: Uuid,
    pub position_title: String,
    pub seats_available: i32,
    pub total_votes: i32,
    pub candidates: Vec<CandidateResultResponse>,
}

#[derive(Debug, Serialize)]
pub struct CandidateResultResponse {
    pub candidate_id: Uuid,
    pub candidate_name: String,
    pub vote_count: i32,
    pub vote_percentage: f64,
    pub is_winner: bool,
    pub rank: i32,
}

#[derive(Debug, Deserialize)]
pub struct CalculateResultsRequest {
    pub election_id: ElectionId,
}

#[derive(Debug, Deserialize)]
pub struct PublishResultsRequest {
    pub election_id: ElectionId,
}

#[derive(Debug, Serialize)]
pub struct ResultSummaryResponse {
    pub election_id: ElectionId,
    pub election_title: String,
    pub total_ballots: i32,
    pub status: String,
    pub results_published: bool,
}
