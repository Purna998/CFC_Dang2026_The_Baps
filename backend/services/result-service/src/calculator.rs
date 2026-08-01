//! Vote counting and result calculation logic

use eemp_error::{AppError, Result};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::{DecryptedBallot, PositionTally};

/// Calculate vote tallies from decrypted ballots
pub fn calculate_tallies(ballots: Vec<DecryptedBallot>) -> Result<HashMap<Uuid, PositionTally>> {
    let mut position_tallies: HashMap<Uuid, PositionTally> = HashMap::new();

    for ballot in ballots {
        for vote in ballot.votes {
            let position_id = Uuid::parse_str(&vote.position_id)
                .map_err(|e| AppError::InternalError(format!("Invalid position ID: {}", e)))?;

            let tally = position_tallies.entry(position_id).or_insert(PositionTally {
                position_id,
                candidate_votes: HashMap::new(),
                abstain_count: 0,
                total_votes: 0,
            });

            tally.total_votes += 1;

            if vote.is_abstain {
                tally.abstain_count += 1;
            } else {
                for candidate_id_str in vote.candidate_ids {
                    let candidate_id = Uuid::parse_str(&candidate_id_str).map_err(|e| {
                        AppError::InternalError(format!("Invalid candidate ID: {}", e))
                    })?;

                    *tally.candidate_votes.entry(candidate_id).or_insert(0) += 1;
                }
            }
        }
    }

    Ok(position_tallies)
}

/// Determine winners for a position based on seats available
pub fn determine_winners(
    candidate_votes: &HashMap<Uuid, i32>,
    seats_available: i32,
) -> Vec<(Uuid, i32, i32)> {
    // Convert to vec and sort by vote count (descending)
    let mut candidates: Vec<(Uuid, i32)> = candidate_votes
        .iter()
        .map(|(id, count)| (*id, *count))
        .collect();

    candidates.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))); // Sort by votes desc, then ID for determinism

    // Assign ranks and determine winners
    let mut results = Vec::new();
    let mut current_rank = 1;
    let mut prev_votes: Option<i32> = None;

    for (idx, (candidate_id, votes)) in candidates.iter().enumerate() {
        // Handle ties - same votes = same rank
        if let Some(prev) = prev_votes {
            if *votes < prev {
                current_rank = (idx + 1) as i32;
            }
        }

        let is_winner = current_rank <= seats_available;
        results.push((*candidate_id, *votes, current_rank));

        prev_votes = Some(*votes);
    }

    results
}

/// Calculate vote percentage
pub fn calculate_percentage(vote_count: i32, total_votes: i32) -> f64 {
    if total_votes == 0 {
        0.0
    } else {
        (vote_count as f64 / total_votes as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_winners_simple() {
        let mut votes = HashMap::new();
        let c1 = Uuid::new_v4();
        let c2 = Uuid::new_v4();
        let c3 = Uuid::new_v4();

        votes.insert(c1, 100);
        votes.insert(c2, 80);
        votes.insert(c3, 60);

        let results = determine_winners(&votes, 2);

        assert_eq!(results.len(), 3);

        // Top 2 should be winners
        let winners: Vec<_> = results.iter().filter(|(_, _, rank)| *rank <= 2).collect();
        assert_eq!(winners.len(), 2);

        // Check ranks
        assert_eq!(results[0].2, 1); // Rank 1 (100 votes)
        assert_eq!(results[1].2, 2); // Rank 2 (80 votes)
        assert_eq!(results[2].2, 3); // Rank 3 (60 votes)
    }

    #[test]
    fn test_determine_winners_tie() {
        let mut votes = HashMap::new();
        let c1 = Uuid::new_v4();
        let c2 = Uuid::new_v4();
        let c3 = Uuid::new_v4();

        votes.insert(c1, 100);
        votes.insert(c2, 100); // Tie for first
        votes.insert(c3, 80);

        let results = determine_winners(&votes, 1);

        // Both with 100 votes should have rank 1
        let rank_1_count = results.iter().filter(|(_, _, rank)| *rank == 1).count();
        assert_eq!(rank_1_count, 2);
    }

    #[test]
    fn test_calculate_percentage() {
        assert_eq!(calculate_percentage(50, 100), 50.0);
        assert_eq!(calculate_percentage(1, 3), 33.333333333333336);
        assert_eq!(calculate_percentage(0, 100), 0.0);
        assert_eq!(calculate_percentage(0, 0), 0.0);
    }

    #[test]
    fn test_calculate_tallies() {
        let ballot = DecryptedBallot {
            ballot_id: Uuid::new_v4(),
            election_id: Uuid::new_v4().to_string(),
            votes: vec![crate::models::DecryptedVote {
                position_id: Uuid::new_v4().to_string(),
                candidate_ids: vec![Uuid::new_v4().to_string()],
                is_abstain: false,
            }],
        };

        let tallies = calculate_tallies(vec![ballot]).unwrap();
        assert_eq!(tallies.len(), 1);
    }
}
