use crate::util::units::Second;

pub struct BenchMarkResult {
    /// The full command line of the program that is being benchmarked
    pub command: String,

    /// The average run time
    pub mean: Second,

    /// The standard deviation of all run times. Not available if only one run has been performed
    pub stddev: Option<Second>,

    /// The median run time
    pub median: Second,

    /// Time spent in user mode
    pub user: Second,

    /// Time spent in kernel mode
    pub system: Second,

    /// Minimum of all measured times
    pub min: Second,

    /// Maximum of all measured times
    pub max: Second,
}
