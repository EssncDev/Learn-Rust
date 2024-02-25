struct Bike {
    name: String,
    brand: String,
    color: String,
    gears: u8,
    current_gear: u8,
    speed: f32,
    vmax: f32
}

impl Bike {
    fn get_bike_stats(&self) {
        // function to parse object values into key - value Vec
        fn parse_key_values_into_vec(bike: &Bike) -> Vec<(&str, &dyn std::fmt::Display)> {
            vec![
                ("Name", &bike.name as &dyn std::fmt::Display),
                ("Brand", &bike.brand as &dyn std::fmt::Display),
                ("Color", &bike.color as &dyn std::fmt::Display),
                ("Gears", &bike.gears as &dyn std::fmt::Display),
                ("Current Gear", &bike.current_gear as &dyn std::fmt::Display),
                ("Speed", &bike.speed as &dyn std::fmt::Display),
                ("Vmax", &bike.vmax as &dyn std::fmt::Display),
            ]
        }

        let bike_fields = parse_key_values_into_vec(self);
        println!("Bike Stats:");
        for (key, value) in &bike_fields {
            println!("  {}: {}", key, value);
        }
    }

    fn get_current_speed_kph(&self) {
        if !(self.speed == 0.0) {
            println!("{} is traveling at {} kph", self.name, self.speed);
            return;
        }

        println!("{} is parked", self.name);
    }

    fn get_current_speed_mph(&self) {
        let mph_calc: f32 = 5.0/3.0;
        let mph = self.speed / mph_calc;
        println!("{} is traveling at {} mph", self.name, mph);
    }

    fn set_new_color(&mut self, color: String) {
        self.color = color;
    }

    fn accelerate(&mut self) {
        let speed_shifts = 255.0 / 12.0;
        let current_gear_float: f32 = self.current_gear as f32;
        if !(self.speed + 10.0 <= self.vmax) {
            return;
        }

        self.speed += 10.0;
        if self.speed > (current_gear_float*speed_shifts) {
            self.shift_gear();
        }
    }

    fn shift_gear(&mut self) {
        if !(self.current_gear < self.gears) {
            println!("Cannot shift gear: current gear exceeds maximum");
            return;
        }
        self.current_gear += 1;
        println!("{} shifted into {} gear @ a speed of {} kph", self.name, self.current_gear, self.speed);
    }

    fn stop(&mut self) {
        if self.speed > 0.0 {
            self.speed = 0.0;
            self.current_gear = 1
        }
    }
}


fn main() {
    let mut bike1 = Bike {
        name: String::from("Bike1"),
        brand: String::from("Cozy"),
        color: String::from("White-Blue"),
        gears: 12,
        current_gear: 1,
        speed: 0.0,
        vmax: 255.0
    };

    bike1.get_current_speed_kph();
    bike1.accelerate();
    bike1.accelerate();
    bike1.accelerate();
    bike1.get_current_speed_kph();
    bike1.set_new_color(String::from("Metallic-Ocean-Blue"));
    bike1.stop();
}
