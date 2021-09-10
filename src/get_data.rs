pub mod temps {
    use sysinfo::{ComponentExt, System, SystemExt};
    
    use indexmap::IndexMap;
    use std::{collections::HashMap, ops::Deref};

// used to aquire, format, and store temperature data
    pub struct TempMaps {
        pub x_minmax: [f64; 2],
        pub y_minmax: [f64; 2],
        last_pair: HashMap< String, (f64, f64) >, // last added X Y pair of each line
        min_pos: (f64, f64), // valley positions
        max_pos: (f64, f64), // peak positions
        pub maps: IndexMap< String, Vec<(f64, f64)> >,
        rolling_average: HashMap< String, Vec<f64> >,
        system: sysinfo::System,
    }
    
    impl TempMaps {
        pub fn new(system: System) -> TempMaps {
            let mut output = TempMaps {
                x_minmax: [0.0; 2],
                y_minmax: [0.0; 2],
                last_pair: HashMap::new(),
                max_pos: (0.0, 0.0),
                min_pos: (0.0, 0.0),
                maps: IndexMap::new(),
                rolling_average: HashMap::new(),
                system: system,
            };
            output.refresh_temps();
            output
        }
        // refreshes the sysinfo instance with temp data,
        // if its a core's temperature, push it into smooth_and_append
        pub fn refresh_temps(&mut self) {
            self.system.refresh_components_list();
            let mut comp_vec: Vec<(String, f64)> = vec![];
            for core in self.system.components() {
                comp_vec.push((core.label().to_owned(), core.temperature().to_owned() as f64));
                // println!("{:?}", core);
            }
            for (comp_name, comp_temp) in comp_vec {
                if !comp_name.contains("Core") {
                    continue
                }
                let next_val: (f64, f64);
                let last_val = self.last_pair.get(&comp_name);
                if last_val.is_none() {
                    next_val = (1.0, comp_temp);
                    self.maps.insert(comp_name.to_owned(), vec![ next_val ]);
                    self.last_pair.insert(comp_name.to_owned(), next_val);
                } else {
                    let last_val = last_val.unwrap().to_owned();
                    next_val = (
                        last_val.0 + 1.0,
                        comp_temp
                    );
                    self.last_pair.insert(comp_name.to_owned(), next_val);
                    self.smooth_and_append(last_val.to_owned(), next_val.to_owned(), comp_name.to_owned());
                }
                
            }
        }
        fn smooth_and_append(&mut self, last_val: (f64, f64), next_val: (f64, f64), name: String) { // fix this aaaaaaaaaa
            let position_vec = self.maps.get_mut(&name).unwrap();
            let rolled_vec = self.rolling_average.entry(name).or_insert(vec![last_val.1]);
    
            let rate_of_change_x = (last_val.1 - next_val.1).abs();
            for multiplier in calculate_filler_dots(rate_of_change_x as f64) {
                let filler_pos = (
                    ((next_val.0 - last_val.0) * multiplier) + last_val.0,
                    ((next_val.1 - last_val.1) * multiplier) + last_val.1,                    
                );
                let rolled_coords = calculate_rolled_entry(rolled_vec, filler_pos, 30);
                position_vec.push(rolled_coords.to_owned());
            }
            self.check_min_max();
        }
        fn check_min_max( &mut self ) {
            for (name, coord_vec) in &mut self.maps {
                // if newly initialized, set minmax to the last value added
                let last_pair = self.last_pair[name];
                let mut update_y: bool = false;
                if self.y_minmax == [0.0; 2] {
                    self.y_minmax = [last_pair.1; 2];
                    self.max_pos = (last_pair.0, last_pair.1 + 1.0);
                    self.min_pos = (last_pair.0, last_pair.1 - 1.0);
                } else {
                    // updates the X value bounds, shrinks the vector if it is above maximum size
                    let max_x_val = coord_vec.last().unwrap().0;
                    let mut min_x_val = coord_vec.first().unwrap().0;
                    
                    while (max_x_val - min_x_val) > 120.0 {
                        coord_vec.remove(0);
                        min_x_val = coord_vec[0].0;
                        // if the min or max goes off screen, update the Y min max
                        if (self.max_pos.0 <= min_x_val) || (self.min_pos.0 < min_x_val) {
                            update_y = true;
                        }
                    }
                    self.x_minmax = [min_x_val, max_x_val];
                    if update_y {
                        // the temp will never be above 200, thus the minimum will always be set
                        self.y_minmax = [200.0, 0.0]; 
                        for position in coord_vec {
                            if position.1 <= self.y_minmax[0] {
                                self.y_minmax[0] = position.1;
                                self.min_pos = *position;
                            }
                            if position.1 >= self.y_minmax[1] {
                                self.y_minmax[1] = position.1;
                                self.max_pos = *position;
                            }
    
                        }
                    } else {
                        // if the Y min/max value is still on screen, check the newly appended values to see if they have a new min/max
                        if last_pair.1 < self.y_minmax[0] {
                            self.min_pos = last_pair;
                            self.y_minmax[0] = last_pair.1;
                        }
                        if last_pair.1 > self.y_minmax[1] {
                            self.y_minmax[1] = last_pair.1;
                            self.max_pos = last_pair;
                        }
                    }
                }
            }
        }
    }
    fn calculate_rolled_entry(rolled_vec: &mut Vec<f64>, coords:(f64, f64), buffer_len: usize) -> (f64, f64) {
        let (x_val, y_val) = coords;
        rolled_vec.push(y_val);
        while rolled_vec.len() > buffer_len {
            rolled_vec.remove(0);
        }
        let mut all_added:f64 = 0.0;
        for entry in rolled_vec.clone() {
            all_added += &entry
        }
        (x_val, all_added / rolled_vec.len() as f64)
    }
    
    fn calculate_filler_dots(rate_of_change: f64) -> Vec<f64> {
        // let filler_dots = if 6.0 > rate_of_change { ((rate_of_change * rate_of_change) * 1.5) + 2.0 } else { 36.0 };
        // let filler_dots = if rate_of_change > 5.0 { 36.0 } else { 5.0 };
        let filler_dots:f64;
        match rate_of_change {
            0.0..=1.5 => filler_dots = 4.0,
            1.5..=3.0 => filler_dots = 6.0,
            3.0..=6.0 => filler_dots = 10.0,
            _ => filler_dots = 15.0,
        }
        
        let dot_distance:f64 = 1.0 / filler_dots;
        let mut output: Vec<f64> = vec![];
        for x in 1..filler_dots as usize + 1 {
            output.push(dot_distance * x as f64);
        }
        output
    }
}