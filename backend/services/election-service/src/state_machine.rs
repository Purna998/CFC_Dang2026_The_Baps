//! Election state machine
//!
//! State transitions:
//! Draft → Review → Scheduled → Open → Closed → Verifying → Published → Archived

use eemp_domain::ElectionStatus;
use eemp_error::{AppError, Result};

/// Election state machine
pub struct ElectionStateMachine;

impl ElectionStateMachine {
    /// Check if transition from current state to next state is valid
    pub fn can_transition(current: &ElectionStatus, next: &ElectionStatus) -> bool {
        match (current, next) {
            // From Draft
            (ElectionStatus::Draft, ElectionStatus::Review) => true,
            (ElectionStatus::Draft, ElectionStatus::Draft) => true, // Can stay in draft

            // From Review
            (ElectionStatus::Review, ElectionStatus::Scheduled) => true,
            (ElectionStatus::Review, ElectionStatus::Draft) => true, // Can go back to draft

            // From Scheduled
            (ElectionStatus::Scheduled, ElectionStatus::Open) => true,
            (ElectionStatus::Scheduled, ElectionStatus::Draft) => true, // Can cancel

            // From Open
            (ElectionStatus::Open, ElectionStatus::Closed) => true,

            // From Closed
            (ElectionStatus::Closed, ElectionStatus::Verifying) => true,

            // From Verifying
            (ElectionStatus::Verifying, ElectionStatus::Published) => true,
            (ElectionStatus::Verifying, ElectionStatus::Closed) => true, // Verification failed

            // From Published
            (ElectionStatus::Published, ElectionStatus::Archived) => true,

            // From Archived - no transitions allowed
            (ElectionStatus::Archived, _) => false,

            // All other transitions are invalid
            _ => false,
        }
    }

    /// Validate state transition and return error if invalid
    pub fn validate_transition(current: &ElectionStatus, next: &ElectionStatus) -> Result<()> {
        if Self::can_transition(current, next) {
            Ok(())
        } else {
            Err(AppError::ValidationError(format!(
                "Invalid state transition from {:?} to {:?}",
                current, next
            )))
        }
    }

    /// Get allowed next states for current state
    pub fn allowed_next_states(current: &ElectionStatus) -> Vec<ElectionStatus> {
        match current {
            ElectionStatus::Draft => vec![ElectionStatus::Review, ElectionStatus::Draft],
            ElectionStatus::Review => vec![ElectionStatus::Scheduled, ElectionStatus::Draft],
            ElectionStatus::Scheduled => vec![ElectionStatus::Open, ElectionStatus::Draft],
            ElectionStatus::Open => vec![ElectionStatus::Closed],
            ElectionStatus::Closed => vec![ElectionStatus::Verifying],
            ElectionStatus::Verifying => {
                vec![ElectionStatus::Published, ElectionStatus::Closed]
            }
            ElectionStatus::Published => vec![ElectionStatus::Archived],
            ElectionStatus::Archived => vec![],
        }
    }

    /// Check if election is in a final state (no more transitions)
    pub fn is_final_state(status: &ElectionStatus) -> bool {
        matches!(status, ElectionStatus::Archived)
    }

    /// Check if election is active (accepting votes)
    pub fn is_active(status: &ElectionStatus) -> bool {
        matches!(status, ElectionStatus::Open)
    }

    /// Check if election can be modified (positions, candidates)
    pub fn is_modifiable(status: &ElectionStatus) -> bool {
        matches!(
            status,
            ElectionStatus::Draft | ElectionStatus::Review | ElectionStatus::Scheduled
        )
    }

    /// Check if election results can be published
    pub fn can_publish_results(status: &ElectionStatus) -> bool {
        matches!(status, ElectionStatus::Verifying)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(ElectionStateMachine::can_transition(
            &ElectionStatus::Draft,
            &ElectionStatus::Review
        ));
        assert!(ElectionStateMachine::can_transition(
            &ElectionStatus::Review,
            &ElectionStatus::Scheduled
        ));
        assert!(ElectionStateMachine::can_transition(
            &ElectionStatus::Scheduled,
            &ElectionStatus::Open
        ));
        assert!(ElectionStateMachine::can_transition(
            &ElectionStatus::Open,
            &ElectionStatus::Closed
        ));
        assert!(ElectionStateMachine::can_transition(
            &ElectionStatus::Closed,
            &ElectionStatus::Verifying
        ));
        assert!(ElectionStateMachine::can_transition(
            &ElectionStatus::Verifying,
            &ElectionStatus::Published
        ));
        assert!(ElectionStateMachine::can_transition(
            &ElectionStatus::Published,
            &ElectionStatus::Archived
        ));
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(!ElectionStateMachine::can_transition(
            &ElectionStatus::Draft,
            &ElectionStatus::Open
        ));
        assert!(!ElectionStateMachine::can_transition(
            &ElectionStatus::Open,
            &ElectionStatus::Draft
        ));
        assert!(!ElectionStateMachine::can_transition(
            &ElectionStatus::Archived,
            &ElectionStatus::Draft
        ));
    }

    #[test]
    fn test_backward_transitions() {
        // Can go back from Review to Draft
        assert!(ElectionStateMachine::can_transition(
            &ElectionStatus::Review,
            &ElectionStatus::Draft
        ));
        // Can cancel from Scheduled to Draft
        assert!(ElectionStateMachine::can_transition(
            &ElectionStatus::Scheduled,
            &ElectionStatus::Draft
        ));
        // Can retry verification
        assert!(ElectionStateMachine::can_transition(
            &ElectionStatus::Verifying,
            &ElectionStatus::Closed
        ));
    }

    #[test]
    fn test_final_state() {
        assert!(ElectionStateMachine::is_final_state(
            &ElectionStatus::Archived
        ));
        assert!(!ElectionStateMachine::is_final_state(
            &ElectionStatus::Published
        ));
    }

    #[test]
    fn test_active_state() {
        assert!(ElectionStateMachine::is_active(&ElectionStatus::Open));
        assert!(!ElectionStateMachine::is_active(&ElectionStatus::Closed));
    }

    #[test]
    fn test_modifiable_state() {
        assert!(ElectionStateMachine::is_modifiable(&ElectionStatus::Draft));
        assert!(ElectionStateMachine::is_modifiable(&ElectionStatus::Review));
        assert!(ElectionStateMachine::is_modifiable(
            &ElectionStatus::Scheduled
        ));
        assert!(!ElectionStateMachine::is_modifiable(&ElectionStatus::Open));
    }
}
