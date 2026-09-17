use super::{CharacterState, MovementConfig};

pub(crate) fn force_multiplier(state: CharacterState, config: &MovementConfig) -> f32 {
    match state {
        CharacterState::Prone | CharacterState::GoingProne { .. } => config.prone_jet_multiplier,
        CharacterState::Crouching | CharacterState::GettingUp { .. } => {
            config.crouch_jet_multiplier
        }
        _ => 1.0,
    }
}
