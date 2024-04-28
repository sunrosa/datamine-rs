use chrono::Duration;

use crate::model::Score;

pub trait ScoresRef {
    /// Filter scores where it matches player name.
    fn filter_player_name(self, name: String) -> impl Iterator;

    /// Filter scores that are wins.
    fn filter_wins(self) -> impl Iterator;

    /// The highest win count as reported in the scores. NOT the number of winning scores in the set. The game itself reports the win count of a player.
    fn win_count(self) -> i32;

    /// Count the number of scores in the dataset that are wins. Does NOT care about the win count reported by the scores themselves.
    fn win_scores_count(self) -> usize;

    /// The average time spent on each run.
    fn avg_run_time(self) -> Duration;

    /// The longest run.
    fn max_run_time(self) -> Duration;

    /// Total score earned across all runs.
    fn total_score(self) -> i64;
}

impl<'a, T> ScoresRef for T
where
    T: Iterator<Item = &'a Score>,
{
    fn filter_player_name(self, name: String) -> impl Iterator<Item = &'a Score> {
        self.filter(move |s| s.header.player_name == name)
    }

    fn filter_wins(self) -> impl Iterator<Item = &'a Score> {
        self.filter(move |s| s.game.win_type != -1)
    }

    fn win_count(self) -> i32 {
        self.filter_map(|s| s.game.win_total)
            .fold(std::i32::MIN, |a, b| a.max(b))
    }

    fn win_scores_count(self) -> usize {
        self.filter_wins().count()
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

    fn total_score(self) -> i64 {
        self.fold(0, |a, b| a + b.performance.total_score as i64)
    }
}
