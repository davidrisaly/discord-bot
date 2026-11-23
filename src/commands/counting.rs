//! for checking if counting goes right

use super::{CONFIG, Message};

/// How it works
/// compare 2 values (current_number and next_number).
///
/// rust pseudocode:
/// ```rust
/// if next_number == current_number + 1 {
///     say("Approved! :thumbs_up:");
///     next_number += 1;
/// }
/// else if next_number != current_number + 1 { current_number = 0; }
/// else {
///     return;
/// }
/// ```
// TODO: Implement this later, i'm too sleepy rn.
pub async fn counting(message: &Message) {
    let mut guard = CONFIG.write().await;

    let current_number = guard.inner.counting.current_number;
}
