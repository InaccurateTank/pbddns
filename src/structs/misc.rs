pub struct CommandStatus {
	changed: u8,
	skipped: u8,
	errored: u8
}
impl CommandStatus {
	pub fn add_changed(&mut self) {
		self.changed = self.changed.saturating_add(1)
	}

	pub fn add_skipped(&mut self) {
		self.skipped = self.skipped.saturating_add(1)
	}

	pub fn add_errored(&mut self) {
		self.errored = self.errored.saturating_add(1)
	}
}
impl Default for CommandStatus {
	fn default() -> Self {
		Self { changed: 0, skipped: 0, errored: 0 }
	}
}
impl std::fmt::Display for CommandStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} Changed, {} Skipped, {} Errored", self.changed, self.skipped, self.errored)
	}
}
