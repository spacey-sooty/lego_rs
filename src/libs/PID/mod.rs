use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor};

pub struct PIDConfig {
  // immutable
  path: String,
  p: f64,
  i: f64,
  d: f64,
  // mutable
  izone: u16,
  error: f32
}

impl PIDConfig {
  // getters for fields
  fn get_path(&self) -> &String { &self.path }
  fn get_p(&self) -> f64 { self.p }
  fn get_i(&self) -> f64 { self.i }
  fn get_d(&self) -> f64 { self.d }
  fn get_izone(&self) -> u16 { self.izone }
  fn get_error(&self) -> f32 { self.error }

  // setters for mutable values
  fn set_izone(&mut self, val: u16) { self.izone = val }
  fn set_error(&mut self, val: f32) { self.error = val }
}

pub enum System {
  LargeMotor(LargeMotor),
  MediumMotor(MediumMotor),
}

impl System {}

pub struct PIDController {
  // mutable (for values that are mutable in PIDConfig type)
  config: PIDConfig,
  // the system that is controlled by this PID controller instance
  // immutable
  system: System
}

impl PIDController {
  // getters
  fn get_config(&self) -> &PIDConfig { &self.config }
  fn get_system(&self) -> &System { &self.system }
}
