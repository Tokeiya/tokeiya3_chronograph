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
	state: Status,
	pivot: Option<Instant>,
	duration: Duration,
}

impl Chronograph {
	pub fn new() -> Self {
		todo!()
	}

	pub fn start(&mut self) -> Result<Duration, StateError> {
		todo!()
	}

	pub fn stop(&mut self) -> Duration {
		todo!()
	}

	pub fn elapsed(&self) -> Duration {
		todo!()
	}

	pub fn reset(&mut self) -> Duration {
		todo!()
	}

	pub fn restart(&mut self) -> Duration {
		todo!()
	}

	pub fn state(&self) -> &Status {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::time::{Duration, Instant};

	macro_rules! assert_matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => assert!(true),
            _ => unreachable!()
        }
    };
}

	macro_rules! extract {
		($expression:expr,$pat:pat=>$result:expr) => {
			match $expression {
				$pat => $result,
				_ => unreachable!(),
			}
		};
	}
	fn almost_eq(actual: &Duration, expected: u64, err: u64) {
		assert!((*actual - Duration::from_millis(expected)) <= Duration::from_millis(err))
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
		assert_matches!(fixture.state, Status::Initial);
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
		fixture.start();

		sleep(100);

		let expected = fixture.stop();
		assert_eq!(fixture.reset(), expected);
		assert_matches!(fixture.state(), Status::Initial);

		sleep(100);

		just_eq(&fixture.reset(), 0);
		assert_matches!(fixture.state(), Status::Initial);

		fixture.start();
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
