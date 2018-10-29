pub struct Hex{
    x_val: f32,
    y_val: f32,
}

impl Hex {
    fn distance(&self, harold: &Hex) -> f32 {
        match determine_diagonal(self.x_val, self.y_val, harold.x_val, harold.y_val){
            Diagonal::UpRight => {
                match greater_distance(self, harold) {
                    DistanceType::Xdist => {return (self.x_val - harold.x_val).abs();},
                    DistanceType::Ydist => {return (self.y_val - harold.y_val).abs();},
                }
            },
            Diagonal::UpLeft => {
                return (self.x_val - harold.x_val).abs() + (self.y_val - harold.y_val).abs();
            }
        }

        /* old implementation        if (self.x_val - harold.x_val).abs() > (self.y_val - harold.y_val).abs(){
            (self.x_val - harold.x_val).abs()
        } else {
            (harold.y_val - self.y_val).abs()
        } */          
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

    fn regions_within_two_turns(&self, robert: &Vec<Region>) -> String {
        //let mut out: i32 = 0;
        let mut names: String = String::new();
        for r in robert{
            if self.shortest_path(r) <= 4.0{
          //      out = out + 1;
                names.push_str(&r.name);
                names.push_str(", ");
            }
        }
        names 
    }
}

fn build_hex(x: f32, y: f32) -> Hex {
    Hex{
        x_val: x,
        y_val: y,
    }
}

// determines whether the distance between x1 and x2 or the distancce between y1 and y2 is greater.
fn greater_distance(h1: &Hex, h2: &Hex) -> DistanceType{
    if (h1.x_val - h2.x_val).abs() > (h1.y_val - h2.y_val).abs() {
        return DistanceType::Xdist;
    } else {
        return DistanceType::Ydist;
    }
}

enum DistanceType{
    Xdist,
    Ydist,
}

enum Diagonal{
    UpLeft,
    UpRight,
}

fn determine_diagonal(x1: f32, y1: f32, x2: f32, y2: f32) -> Diagonal {
    if x1 <= x2 && y1 <= y2 {
        return Diagonal::UpRight;
    } else if x1 < x2 && y1 > y2 {
        return Diagonal::UpLeft;
    } else if x1 > x2 && y1 < y2 {
        return Diagonal::UpLeft;
    } else if x1 >= x2 && y1 >= y2 {
        return Diagonal::UpRight;
    } else {
        panic!("shit's fucked in the diagonalization check, yo");
    }
}

pub struct Region{
    name: String,
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
        let topbound: usize;
        let bottombound: usize;
        if i < 5 {
            topbound = 5 + i;
            bottombound = 0;
        } else {
            topbound = 9;
            bottombound = i - 4;
        }
        for j in bottombound..topbound {
            map.push(build_hex(j as f32, i as f32));
        }
    }

    // make the bay
    let mut bay = Region {
        name: String::from("bay"),
        hexes: Vec::new(),
    };
    bay.new_hex(5, 1);
    bay.new_hex(5, 2);
    bay.new_hex(6, 2);

    //make the desert
    let mut desert = Region {
        name: String::from("desert"),
        hexes: Vec::new(),
    };
    desert.new_hex(0, 1);
    desert.new_hex(0, 2);
    desert.new_hex(0, 3);
    desert.new_hex(1, 2);
    desert.new_hex(1, 3);

    //make the forest
    let mut forest = Region {
        name: String::from("forest"),
        hexes: Vec::new(),
    };
    forest.new_hex(3,4);
    forest.new_hex(4,5);

    //make the marsh
    let mut marsh = Region {
        name: String::from("marsh"),
        hexes: Vec::new(),
    };
    marsh.new_hex(6, 6);
    marsh.new_hex(5, 7);
    marsh.new_hex(6, 7);
    marsh.new_hex(7, 8);

    // make the convent
    let mut convent = Region {
        name: String::from("convent"),
        hexes: Vec::new(),
    };
    convent.new_hex(2,1);

    //make the castle
    let mut castle = Region {
        name: String::from("castle"),
        hexes: Vec::new(),
    };
    castle.new_hex(3,6);

    //make the library
    let mut library = Region {
        name: String::from("library"),
        hexes: Vec::new(),
    };
    library.new_hex(7,5);

    let regions = vec![bay, desert, marsh, forest, convent, castle, library];


    for h in map{
        let avg = mean(h.region_path_distances(&regions));;
        if h.y_val == 0.0  || h.y_val == 4.0  || h.y_val == 8.0  {
            if h.x_val == 0.0 || h.x_val == 8.0 || (h.y_val == 0.0 && h.x_val == 4.0) {
                println!("{}, {}, {}, {}", h.x_val, h.y_val, avg, h.regions_within_two_turns(&regions));
            }
        }
    }
}
