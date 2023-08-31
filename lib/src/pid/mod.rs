use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor};

pub struct PIDConfig {
  // immutable
  path: String,
  p: f32,
  i: f32,
  d: f32,
  // mutable
  izone: f32,
  error: f32,
}

impl PIDConfig {
  // getters for fields
  pub fn get_path(&self) -> &String { &self.path }
  pub fn get_p(&self) -> f32 { self.p }
  pub fn get_i(&self) -> f32 { self.i }
  pub fn get_d(&self) -> f32 { self.d }
  pub fn get_izone(&self) -> f32 { self.izone }
  pub fn get_error(&self) -> f32 { self.error }

  // setters for mutable values
  pub fn set_izone(&mut self, val: f32) { self.izone = val }
  pub fn set_error(&mut self, val: f32) { self.error = val }

  pub fn from(path: String, p: f32, i: f32, d: f32, izone: f32, error: f32) -> PIDConfig {
    return PIDConfig {
        path: path,
        p: p,
        i: i,
        d: d,
        izone: izone,
        error: error
    };
  }
}

pub enum System {
  LargeMotor(LargeMotor),
  MediumMotor(MediumMotor),
}

impl System {}

pub struct PIDController {
  // mutable (for values that are mutable in PIDConfig type)
  config: PIDConfig,
  target: f32,
  dt: f32,
  // immutable
  // the system that is controlled by this PID controller instance
  system: System,
  c: f32,
  finished: bool
}

impl PIDController {
  // getters
  pub fn get_config(&self) -> &PIDConfig { &self.config }
  pub fn get_system(&self) -> &System { &self.system }

  // closed loop
  pub fn closed_loop_c(&mut self) {
      self.c = self.config.p + self.config.i / self.dt + self.config.d * self.dt;
  }

  pub fn on_update(&mut self) {
      self.closed_loop_c();
      match &self.system {
          System::LargeMotor(motor) => motor.set_duty_cycle_sp(50 + self.config.d as i32).unwrap(),
          System::MediumMotor(motor) => motor.set_duty_cycle_sp(50 + self.config.d as i32).unwrap(),
      };
      if self.config.izone > self.target - self.c {
          self.config.d -= 0.35;
      }

      if -self.config.error < self.target - self.c && self.target - self.c < self.config.error {
        self.finished = true;
      }
  }

  pub fn from(config: PIDConfig, target: f32, dt: f32, system: System, c: f32) -> PIDController {
    return PIDController {
        config: config,
        target: target,
        dt: dt,
        system: system,
        c: c,
        finished: false,
    };
  }
}

