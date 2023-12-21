use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::time::{Duration, Instant};

pub struct StateError(String);

impl StateError {
	fn new(msg: &str) -> Self {
		Self(msg.to_string())
	}

	fn msg(&self) -> &str {
		&self.0
	}

	fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.msg())
	}
}

impl Debug for StateError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for StateError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Error for StateError {}

pub enum Status {
	Initial,
	Running,
	Stopped,
}

pub struct Chronograph {
	pivot: Option<Instant>,
	duration: Duration,
}
impl Default for Chronograph {
	fn default() -> Self {
		Self::new()
	}
}

impl Chronograph {
	pub fn new() -> Self {
		Chronograph {
			pivot: None,
			duration: Duration::new(0, 0),
		}
	}

	pub fn start(&mut self) -> Result<Duration, StateError> {
		let now = Instant::now();

		if self.pivot.is_none() {
			self.pivot = Some(now);
			Ok(self.duration)
		} else {
			Err(StateError::new("Already running."))
		}
	}

	pub fn stop(&mut self) -> Duration {
		let now = Instant::now();

		if let Some(time) = self.pivot {
			self.duration += now - time;
			self.pivot = None;
		}

		self.duration
	}

	pub fn elapsed(&self) -> Duration {
		let now = Instant::now();

		if let Some(piv) = self.pivot {
			self.duration + (now - piv)
		} else {
			self.duration
		}
	}

	pub fn reset(&mut self) -> Duration {
		let now = Instant::now();

		if let Some(piv) = self.pivot {
			let ret = self.duration + (now - piv);
			self.duration = Duration::new(0, 0);
			self.pivot = None;
			ret
		} else {
			let ret = self.duration;
			self.duration = Duration::new(0, 0);
			ret
		}
	}

	pub fn restart(&mut self) -> Duration {
		let ret = self.reset();
		self.start().unwrap();
		ret
	}

	pub fn state(&self) -> Status {
		const ZERO: Duration = Duration::new(0, 0);

		if self.pivot.is_some() {
			Status::Running
		} else if self.duration == ZERO {
			Status::Initial
		} else {
			Status::Stopped
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::time::Duration;

	macro_rules! assert_matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => assert!(true),
            _ => unreachable!()
        }
    };
}

	fn almost_10ms(actual: &Duration, expected: u64) {
		assert!((*actual - Duration::from_millis(expected)) <= Duration::from_millis(20))
	}

	fn just_eq(actual: &Duration, expected: u64) {
		assert_eq!(*actual, Duration::from_millis(expected))
	}

	#[inline]
	fn sleep(milli_seconds: u64) {
		std::thread::sleep(Duration::from_millis(milli_seconds));
	}

	#[test]
	fn new_test() {
		let fixture = Chronograph::new();
		assert_matches!(fixture.state(), Status::Initial);
		assert_matches!(fixture.pivot, None);
		assert_eq!(fixture.duration, Duration::new(0, 0));
	}

	#[test]
	fn start_stop_test() {
		let mut fixture = Chronograph::new();
		assert_matches!(fixture.start(),Ok(act) if act==Duration::from_millis(0));
		assert_matches!(fixture.state(), Status::Running);

		sleep(250);

		let actual = fixture.stop();
		assert_matches!(fixture.state(), Status::Stopped);
		almost_10ms(&actual, 250);

		let other = fixture.stop();
		assert_matches!(fixture.state(), Status::Stopped);
		assert_eq!(actual, other);
	}

	#[test]
	fn dupl_start_test() {
		let mut fixture = Chronograph::new();
		just_eq(&fixture.start().unwrap(), 0);

		sleep(100);

		assert_matches!(fixture.start(), Err(_));
		assert_matches!(fixture.state(), Status::Running);
		almost_10ms(&fixture.elapsed(), 100);

		sleep(100);

		let act = fixture.stop();
		assert_matches!(fixture.state(), Status::Stopped);
		almost_10ms(&act, 200);
	}

	#[test]
	fn start_stop_iteration_test() {
		let mut fixture = Chronograph::new();
		_ = fixture.start();

		sleep(100);

		let expected = fixture.stop();

		sleep(100);

		assert_eq!(fixture.start().unwrap(), expected);
		assert_matches!(fixture.state(), Status::Running);

		sleep(100);

		let act = fixture.stop();
		almost_10ms(&act, 200);
	}

	#[test]
	fn elapsed_test() {
		let mut fixture = Chronograph::new();

		_ = fixture.start();

		sleep(100);
		almost_10ms(&fixture.elapsed(), 100);

		sleep(100);
		almost_10ms(&fixture.elapsed(), 200);

		sleep(100);
		almost_10ms(&fixture.elapsed(), 300);
	}

	#[test]
	fn reset_test() {
		let mut fixture = Chronograph::new();
		_ = fixture.start();

		sleep(100);

		let expected = fixture.stop();
		assert_eq!(fixture.reset(), expected);
		assert_matches!(fixture.state(), Status::Initial);

		sleep(100);

		just_eq(&fixture.reset(), 0);
		assert_matches!(fixture.state(), Status::Initial);

		_ = fixture.start();
		sleep(100);

		almost_10ms(&fixture.reset(), 100);
		assert_matches!(fixture.state(), Status::Initial);

		just_eq(&fixture.reset(), 0);
		assert_matches!(fixture.state(), Status::Initial);
	}
	#[test]
	fn restart_test() {
		let mut fixture = Chronograph::new();
		just_eq(&fixture.restart(), 0);
		assert_matches!(fixture.state(), Status::Running);

		sleep(100);

		let expected = fixture.stop();
		assert_eq!(expected, fixture.restart());
		assert_matches!(fixture.state(), Status::Running);

		sleep(100);

		almost_10ms(&fixture.restart(), 100);
	}
}
