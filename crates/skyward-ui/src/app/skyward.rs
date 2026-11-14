//! Skyward's own update page: asking GitHub for its newest release and
//! updating to it.

use super::*;

/// How far asking GitHub for Skyward's newest release has come.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum SkywardCheck {
    #[default]
    Idle,
    Asking,
    Failed(Unanswered),
    Answered(Newest),
}

/// Updating Skyward, as its page shows it: downloading the newest release's
/// build for the phone, then writing it into the recovery partitions, how
/// far that has come, and how it ended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SkywardJob {
    /// None until the download has begun.
    pub stage: Option<SkywardStage>,
    /// Bytes a second, while it downloads.
    pub rate: u64,
    pub outcome: Option<SkywardEnd>,
    /// Stopping the download was asked for and is under way.
    pub stopping: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkywardStage {
    Download(Progress),
    /// The build is written into the recovery partitions and read back:
    /// bytes so far, of all.
    Write { done: u64, total: u64 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SkywardEnd {
    /// The build is in the recovery partitions of these slots.
    Updated(Vec<Slot>),
    /// The download was stopped; what it downloaded is kept to go on from.
    Stopped,
    /// The download did not end with the build whole.
    Download(Failure),
    /// The build could not go in, or where it goes could not be told: why.
    /// Nothing was written.
    NotWritten(String),
    /// Writing it into the recovery partitions of these slots failed, after
    /// some of it was written: why.
    WriteFailed(Vec<Slot>, String),
}

impl App {
    /// What GitHub answered of Skyward's newest release.
    pub fn skyward_checked(&mut self, answer: Result<Newest, Unanswered>) {
        self.state.skyward_check = match answer {
            Ok(newest) => SkywardCheck::Answered(newest),
            Err(why) => SkywardCheck::Failed(why),
        };
        if matches!(self.screen(), Screen::Settings | Screen::SkywardUpdate) {
            self.refresh();
        }
    }

    /// How far updating Skyward has come, and how fast the download goes,
    /// in bytes a second.
    pub fn skyward_progress(&mut self, stage: SkywardStage, rate: u64) {
        let job = self.state.skyward_job.as_mut().expect("only an update under way moves on");
        job.stage = Some(stage);
        job.rate = rate;
        if self.screen() == Screen::SkywardUpdate {
            self.refresh();
        }
    }

    /// Updating Skyward has ended: its page says how, lit for the user to
    /// read.
    pub fn skyward_ended(&mut self, end: SkywardEnd, now: u64) {
        let job = self.state.skyward_job.as_mut().expect("only an update under way ends");
        job.stopping = false;
        job.outcome = Some(end);
        self.wake(now);
        self.refresh();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::testing::*;

    fn skyward_newest(newer: bool) -> Newest {
        Newest {
            version: "0.1.4".into(),
            newer,
            build: Some(Build {
                name: "skyward-v0.1.4-ishtar-OS3.0.308.0.WMACNXM.zip".into(),
                url: "https://api.github.com/repos/kirklin/skyward/releases/assets/719751598".into(),
                link: "https://github.com/kirklin/skyward/releases/download/v0.1.4/skyward-v0.1.4-ishtar-OS3.0.308.0.WMACNXM.zip".into(),
                size: 42_739_608,
                sha256: "62b66f7d13a0eb6c9af3359b5ec6f9c223fd9a86b1f8abab3e9d2646cb2ce534".into(),
            }),
        }
    }

    #[test]
    fn the_about_page_holds_no_control_and_the_entry_comes_as_the_phone_allows() {
        let (f, i) = (fonts(), info());
        let mut app = App::new(Screen::Settings, W, H, PPD);
        app.refresh();
        let offered = |app: &mut App| app.frame(&f, &i).hits.iter().any(|h| h.action == Action::Open(Screen::SkywardUpdate));
        assert!(!offered(&mut app), "a phone whose WLAN the recovery does not bring up");
        app.state.wlan = Some(Wlan::default());
        app.refresh();
        assert!(!offered(&mut app), "a phone that keeps no recovery partition in each slot");
        app.state.recovery_partitions = true;
        app.refresh();
        assert!(offered(&mut app), "the page is offered once the phone allows it");
    }

    #[test]
    fn the_about_page_shows_the_credits_and_asks_github_nothing() {
        let (f, i) = (fonts(), info());
        let mut app = App::new(Screen::Settings, W, H, PPD);
        app.state.wlan = Some(connected("Skyward"));
        app.state.recovery_partitions = true;
        app.refresh();
        assert_eq!(tap(&mut app, &f, &i, Action::Open(Screen::SkywardUpdate)), vec![Effect::Haptic], "opening it asks nothing");
        assert_eq!(app.screen(), Screen::SkywardUpdate);
        assert!(app.frame(&f, &i).hits.is_empty(), "the page holds no control");
    }
}
