use std::borrow::Borrow;
use std::fmt::Debug as DebugT;

use chrono::Duration;

use crate::model::{Bonus, Machines, MachinesAccessed, Performance, Score};

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
    fn total_score(self) -> Performance;

    /// Total machines accessed across all runs.
    fn total_machines_accessed(self) -> MachinesAccessed;

    /// Highest performance scores across all runs. Each field is its own maximum. This performance metric is very likely not from one single run.
    fn max_perf_individual(self) -> Performance;

    /// Total bonus earned across all runs.
    fn total_bonus(self) -> Bonus;
}

impl<T, U> ScoresRef for T
where
    T: Iterator<Item = U> + DebugT,
    U: Borrow<Score>,
{
    fn filter_player_name(
        self,
        name: String,
    ) -> impl Iterator<Item = U> + DebugT {
        self.filter(move |s| s.borrow().header.player_name == name)
    }

    fn filter_wins(self) -> impl Iterator<Item = U> + DebugT {
        self.filter(move |s| s.borrow().game.win_type != -1)
    }

    fn win_count(self) -> i32 {
        self.filter_map(|s| s.borrow().game.win_total)
            .fold(std::i32::MIN, |a, b| a.max(b))
    }

    fn win_scores_count(self) -> usize {
        self.filter_wins().count()
    }

    fn avg_run_time(self) -> Duration {
        let (run_count, duration) = self.fold((0, Duration::zero()), |a, b| {
            (a.0 + 1, a.1 + b.borrow().game.run_time)
        });

        duration / run_count
    }

    fn max_run_time(self) -> Duration {
        self.fold(Duration::zero(), |a, b| a.max(b.borrow().game.run_time))
    }

    fn total_score(self) -> Performance {
        self.fold(Performance::default(), move |a, b| {
            &a + &b.borrow().performance
        })
    }

    fn total_machines_accessed(self) -> MachinesAccessed {
        self.fold(MachinesAccessed::default(), |a, b| {
            let b: &Score = b.borrow();
            &a + &b.stats.hacking.machines_accessed
        })
    }

    fn max_perf_individual(self) -> Performance {
        // let total = self.max_by(|a, b| {
        //     let a: &Score = a.borrow();
        //     let b: &Score = b.borrow();
        //     a.performance.total_score.cmp(&b.performance.total_score)
        // });
        // let evolutions = self.max_by(|a, b| {
        //     let a: &Score = a.borrow();
        //     let b: &Score = b.borrow();
        //     a.performance
        //         .evolutions
        //         .points
        //         .cmp(&b.performance.evolutions.points)
        // });
        // let regions_visited = self.max_by(|a, b| {
        //     let a: &Score = a.borrow();
        //     let b: &Score = b.borrow();
        //     a.performance
        //         .regions_visited
        //         .points
        //         .cmp(&b.performance.regions_visited.points)
        // });
        // let robots_destroyed = self.max_by(|a, b| {
        //     let a: &Score = a.borrow();
        //     let b: &Score = b.borrow();
        //     a.performance
        //         .robots_destroyed
        //         .points
        //         .cmp(&b.performance.robots_destroyed.points)
        // });
        // let value_destroyed = self.max_by(|a, b| {
        //     let a: &Score = a.borrow();
        //     let b: &Score = b.borrow();
        //     a.performance
        //         .value_destroyed
        //         .points
        //         .cmp(&b.performance.value_destroyed.points)
        // });
        // let prototypes_identified = self.max_by(|a, b| {
        //     let a: &Score = a.borrow();
        //     let b: &Score = b.borrow();
        //     a.performance
        //         .prototypes_identified
        //         .points
        //         .cmp(&b.performance.prototypes_identified.points)
        // });
        // let alien_tech_used = self.max_by(|a, b| {
        //     let a: &Score = a.borrow();
        //     let b: &Score = b.borrow();
        //     a.performance
        //         .alien_tech_used
        //         .points
        //         .cmp(&b.performance.alien_tech_used.points)
        // });
        // let bonus = self.max_by(|a, b| {
        //     let a: &Score = a.borrow();
        //     let b: &Score = b.borrow();
        //     a.performance.bonus.points.cmp(&b.performance.bonus.points)
        // });

        let mut total: Option<Score> = None;
        let mut evolutions: Option<Score> = None;
        let mut regions_visited: Option<Score> = None;
        let mut robots_destroyed: Option<Score> = None;
        let mut value_destroyed: Option<Score> = None;
        let mut prototypes_identified: Option<Score> = None;
        let mut alien_tech_used: Option<Score> = None;
        let mut bonus: Option<Score> = None;

        for score in self {
            let score: &Score = score.borrow();

            total = Some(if let Some(to) = total {
                if score.performance.total_score > to.performance.total_score {
                    score.clone()
                } else {
                    to
                }
            } else {
                score.clone()
            });
            evolutions = Some(if let Some(ev) = evolutions {
                if score.performance.total_score > ev.performance.total_score {
                    score.clone()
                } else {
                    ev
                }
            } else {
                score.clone()
            });
            regions_visited = Some(if let Some(re) = regions_visited {
                if score.performance.total_score > re.performance.total_score {
                    score.clone()
                } else {
                    re
                }
            } else {
                score.clone()
            });
            robots_destroyed = Some(if let Some(ro) = robots_destroyed {
                if score.performance.total_score > ro.performance.total_score {
                    score.clone()
                } else {
                    ro
                }
            } else {
                score.clone()
            });
            value_destroyed = Some(if let Some(va) = value_destroyed {
                if score.performance.total_score > va.performance.total_score {
                    score.clone()
                } else {
                    va
                }
            } else {
                score.clone()
            });
            prototypes_identified =
                Some(if let Some(evolutions) = prototypes_identified {
                    if score.performance.total_score
                        > evolutions.performance.total_score
                    {
                        score.clone()
                    } else {
                        evolutions
                    }
                } else {
                    score.clone()
                });
            alien_tech_used = Some(if let Some(al) = alien_tech_used {
                if score.performance.total_score > al.performance.total_score {
                    score.clone()
                } else {
                    al
                }
            } else {
                score.clone()
            });
            bonus = Some(if let Some(bo) = bonus {
                if score.performance.total_score > bo.performance.total_score {
                    score.clone()
                } else {
                    bo
                }
            } else {
                score.clone()
            });
        }

        Performance {
            total_score: total.map_or(0, |s| {
                let s: &Score = s.borrow();
                s.performance.total_score
            }),
            evolutions: evolutions.map_or(Default::default(), |s| {
                let s: &Score = s.borrow();
                s.performance.evolutions
            }),
            regions_visited: regions_visited.map_or(Default::default(), |s| {
                let s: &Score = s.borrow();
                s.performance.regions_visited
            }),
            robots_destroyed: robots_destroyed.map_or(
                Default::default(),
                |s| {
                    let s: &Score = s.borrow();
                    s.performance.robots_destroyed
                },
            ),
            value_destroyed: value_destroyed.map_or(Default::default(), |s| {
                let s: &Score = s.borrow();
                s.performance.value_destroyed
            }),
            prototypes_identified: prototypes_identified.map_or(
                Default::default(),
                |s| {
                    let s: &Score = s.borrow();
                    s.performance.prototypes_identified
                },
            ),
            alien_tech_used: alien_tech_used.map_or(Default::default(), |s| {
                let s: &Score = s.borrow();
                s.performance.alien_tech_used
            }),
            bonus: bonus.map_or(Default::default(), |s| {
                let s: &Score = s.borrow();
                s.performance.bonus
            }),
        }
    }

    fn total_bonus(self) -> Bonus {
        self.fold(Bonus::default(), |a, b| &a + &b.borrow().bonus)
    }
}
