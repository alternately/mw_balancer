pub struct Hex{
    x_val: f32,
    y_val: f32,
}

impl Hex {
    fn distance(&self, harold: &Hex) -> f32 {
        if (self.x_val - harold.x_val).abs() > (self.y_val - harold.y_val).abs(){
            (self.x_val - harold.x_val).abs()
        } else {
            (harold.y_val - self.y_val).abs()
        }
    }

    fn eq(&self, harold: &Hex) -> bool {
        if self.x_val == harold.x_val && self.y_val == harold.y_val {
            true
        } else {
            false
        }
    }

    fn shortest_path(&self, ronald: &Region) -> f32 {
        let mut track: f32 = 999.0;
        for h in &ronald.hexes {
            let d = self.distance(h);
            if d < track {
                track = d;
            }
        }
        track
    }

    fn region_path_distances(&self, romeo: &Vec<Region>) -> Vec<f32> {
        let mut victor: Vec<f32> = Vec::new();
        for r in romeo {
            let d = self.shortest_path(r);
            victor.push(d);
        }
        victor
    }
}

fn build_hex(x: f32, y: f32) -> Hex {
    Hex{
        x_val: x,
        y_val: y,
    }
}


pub struct Region{
    hexes: Vec<Hex>,
}

impl Region {
    fn contains(&self, harold: &Hex) -> bool {
        for h in &self.hexes {
            if h.eq(harold) {
                return true
            }
        }
        false
    }

    //NOTE this does not take a Hex, but instead two f32 coordinates!!!
    fn new_hex(&mut self, x: u32, y: u32){
        let h = build_hex(x as f32, y as f32);
        self.hexes.push(h);
    }
}

fn mean(values: Vec<f32>) -> f32{
    let mut sum: f32 = 0.0;
    let number: f32 = *&values.len() as f32;
    for v in values{
        sum = sum + v;
    }
    sum / number
}



fn main() {
    
    // create a blank map of hexes in the shape of a mys wiz board
    let mut map: Vec<Hex> = Vec::new();
    for i in 0..9{
        let k: usize;
        let l: usize;
        if i < 5 {
            k = 5 + i;
            l = 0;
        } else {
            k = 8;
            l = i - 4;
        }
        for j in l..k {
            map.push(build_hex(j as f32, i as f32));
        }
    }

    // make the bay
    let mut bay = Region {
        hexes: Vec::new(),
    };
    bay.new_hex(5, 1);
    bay.new_hex(5, 2);
    bay.new_hex(6, 2);

    //make the desert
    let mut desert = Region {
        hexes: Vec::new(),
    };
    desert.new_hex(0, 1);
    desert.new_hex(0, 2);
    desert.new_hex(0, 3);
    desert.new_hex(1, 2);
    desert.new_hex(1, 3);

    //make the forest
    let mut forest = Region {
        hexes: Vec::new(),
    };
    forest.new_hex(4,3);
    forest.new_hex(5,4);

    //make the marsh
    let mut marsh = Region {
        hexes: Vec::new(),
    };
    marsh.new_hex(4, 6);
    marsh.new_hex(2, 7);
    marsh.new_hex(3, 7);
    marsh.new_hex(3, 8);

    // make the convent
    let mut convent = Region {
        hexes: Vec::new(),
    };
    convent.new_hex(2,1);

    //make the castle
    let mut castle = Region {
        hexes: Vec::new(),
    };
    castle.new_hex(1,6);

    //make the library
    let mut library = Region {
        hexes: Vec::new(),
    };
    library.new_hex(6,5);

    let regions = vec![bay, desert, marsh, forest, convent, castle, library];


    for h in map{
        let avg = mean(h.region_path_distances(&regions));
        println!("{}, {}, {}", h.x_val, h.y_val, avg);
    }
    println!("Hello world!");
}
