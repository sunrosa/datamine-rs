use chrono::Duration;

use crate::model::Score;

pub trait Scores {
    /// The highest win count as reported in the scores. NOT the number of winning scores in the set. The game itself reports the win count of a player.
    fn win_count(self) -> i32;

    /// The average time spent on each run.
    fn avg_run_time(self) -> Duration;

    /// The longest run.
    fn max_run_time(self) -> Duration;
}

impl<'a, T> Scores for T
where
    T: Iterator<Item = &'a Score>,
{
    fn win_count(self) -> i32 {
        self.filter_map(|s| s.game.win_total)
            .fold(std::i32::MIN, |a, b| a.max(b))
    }

    fn avg_run_time(self) -> Duration {
        let (run_count, duration) = self.fold((0, Duration::zero()), |a, b| {
            (a.0 + 1, a.1 + b.game.run_time)
        });

        duration / run_count
    }

    fn max_run_time(self) -> Duration {
        self.fold(Duration::zero(), |a, b| a.max(b.game.run_time))
    }
}
