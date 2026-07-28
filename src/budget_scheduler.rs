pub struct BudgetScheduler { step: usize, total_samples: usize }
impl BudgetScheduler {
    pub fn new() -> Self { Self { step: 0, total_samples: 0 } }
    pub fn tick(&mut self) { self.step += 1; self.total_samples += 1; }
    pub fn should_restructure(&self) -> bool { self.step % 50 == 0 && self.total_samples > 0 }
}
