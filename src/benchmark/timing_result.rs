use crate::util::units::Second;

pub struct TimingResult {
    /// Wall clock time
    time_real: Second,

    /// Time spent in user mode
    time_user: Second,

    /// Time spent in kernel mode
    time_system: Second,
}
