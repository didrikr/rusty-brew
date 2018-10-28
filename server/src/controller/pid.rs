use std::default::Default;

struct Reference(i32);

#[derive(Clone)]
pub struct PidParameters {
    kp: f32,
    ki: f32,
    kd: f32,
}

impl PidParameters {
    pub fn new(kp: f32, ki: f32, kd: f32) -> PidParameters {
        PidParameters{
            kp,
            ki,
            kd,
        }
    }

    pub fn get_parameters(&self) -> (f32, f32, f32) {
        (self.kp, self.ki, self.kd)
    }
}

impl Default for PidParameters {
    /// kp = 1, ki = 0, kd = 0
    fn default() -> PidParameters {
        PidParameters {
            kp: 1.0,
            ki: 0.0,
            kd: 0.0,
        }
    }
}

pub struct Pid {
    parameters: PidParameters,
    accumulator: f32,
    previous_input: f32,
}

impl Pid {
    pub fn new(parameters: &PidParameters) -> Pid {
        Pid {
            parameters: parameters.clone(),
            accumulator: 0.0,
            previous_input: 0.0,
        }
    }
    pub fn pid(&mut self, input: f32, reference: f32) -> f32 {
        let (kp, ki, kd) = self.parameters.get_parameters();
        let error = reference - input;
        self.accumulator += error;
        let derivative = input - self.previous_input;
        self.previous_input = input;
        kp*error + ki*self.accumulator + kd*derivative
    }
}

