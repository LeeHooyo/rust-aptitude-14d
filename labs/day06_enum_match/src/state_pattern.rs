use std::fmt;

#[derive(Debug)]
enum DeviceState {
    Off,
    On,
    Sleep,
}

trait PowerControl {
    fn toggle(&mut self);
    fn info(&self) -> String;
}

struct Device {
    name: String,
    state: DeviceState,
}

impl PowerControl for Device {
    fn toggle(&mut self) {
        self.state = match self.state {
            DeviceState::Off => DeviceState::On,
            DeviceState::On => DeviceState::Sleep,
            DeviceState::Sleep => DeviceState::Off, 
        };
    }

    fn info(&self) -> String {
        match &self.state {
            DeviceState::Off => format!("{}: 전원 꺼짐", self.name),
            DeviceState::On => format!("{}: 전원 켜짐", self.name),
            DeviceState::Sleep => format!("{}: 절전 모드", self.name),
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.info())
    }
}

pub fn run_demo() {
    let mut laptop = Device {
        name: "Laptop".into(),
        state: DeviceState::Off,
    };

    for _ in 0..4 {
        println!("{}", laptop);
        laptop.toggle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_toggle_cycle() {
        let mut d = Device {
            name: "Test".into(),
            state: DeviceState::Off,
        };

        d.toggle();
        assert!(matches!(d.state, DeviceState::On));

        d.toggle();
        assert!(matches!(d.state, DeviceState::Sleep));

        d.toggle();
        assert!(matches!(d.state, DeviceState::Off));
    }

    #[test]
    fn test_device_info_display() {
        let d = Device {
            name: "Lamp".into(),
            state: DeviceState::On,
        };
        assert_eq!(d.info(), "Lamp: 전원 켜짐");
    }
}
