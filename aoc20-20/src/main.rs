use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

static NESSIE : [[bool; 20]; 3]=[[false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,true,false],
[true,false,false,false,false,true,true,false,false,false,false,true,true,false,false,false,false,true,true,true],
[false,true,false,false,true,false,false,true,false,false,true,false,false,true,false,false,true,false,false,false]];

struct Tile {
    id : u32,
    tile : [[bool; 10]; 10],
    edges : [i32; 4]
}

impl Tile { 
    fn rotate(& mut self) { //clockwise
        self.edges = [-self.edges[3], self.edges[0], -self.edges[1], self.edges[2]];
        let old_tile = self.tile;
        for x in 0..10 {
            for y in 0..10 {
                self.tile[x][y] = old_tile[9-y][x];
            }
        }
    }

    fn flip_lr(& mut self) {
        self.edges = [-self.edges[0], self.edges[3], -self.edges[2], self.edges[1]];
        let old_tile = self.tile;
        for x in 0..10 {
            for y in 0..10 {
                self.tile[x][y] = old_tile[x][9-y];
            }
        }
    }

    fn flip_tb(& mut self) {
        self.edges = [self.edges[2], -self.edges[1], self.edges[0], -self.edges[3]];
        let old_tile = self.tile;
        for x in 0..10 {
            for y in 0..10 {
                self.tile[x][y] = old_tile[9-x][y];
            }
        }
    }
}

// create edge id from edge, negative if it requires reversal 
fn to_edge_id(edge: &[bool; 10]) -> i32 {
    let mut id1 = 0i32;
    let mut id2 = 0i32;
    for i in 0..10 {
        id1 <<= 1;
        id2 <<= 1;
        if edge[i] {
            id1 |= 1;
        }
        if edge[9-i] {
            id2 |= 1;
        }
    }
    if id1 <= id2 {
        id1
    }
    else {
        -id2
    }
}

// add abs of edge id to edge map, return edge id
fn add_edge(tile_id: u32, edge: &[bool; 10], edges: &mut HashMap<u32, Vec<u32>>) -> i32{
    let id = to_edge_id(edge);
    let pos_id = id.abs() as u32;
    if let Some(v) = edges.get_mut(&pos_id) {
        v.push(tile_id);
    }
    else {
        edges.insert(pos_id, vec!(tile_id));
    }
    id
}
// check if there is a given nessie at the given position.
// strike out all tiles, if they are part of a nessie
fn check_nessie(pic: &[[bool; 96]; 96], not_nessie: & mut [[bool; 96]; 96], x: usize, y: usize, horizontal: bool, nessie: &[[bool;20];3]) -> bool {
    if horizontal && y+20 >= pic[0].len() {
        return false;
    }
    else if !horizontal && x+20 >= pic.len() {
        return false;
    }
    for i in 0..20 {
        for j in 0..3 {
            if nessie[j][i] && !pic[if horizontal {x+j} else {x+i}][if horizontal {y+i} else {y+j}]{
                return false;
            }
        }
    }
    for i in 0..20 {
        for j in 0..3 {
            if nessie[j][i]{
                not_nessie[if horizontal {x+j} else {x+i}][if horizontal {y+i} else {y+j}] = false;
            }
        }
    }
    println!("NESSIE found, starting at {},{} - Horizontal: {}", x,y,horizontal);
    true
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut current_id = 0;
    let mut current_tile = [[false; 10]; 10];
    let mut current_line = 0;
    let mut tiles = HashMap::new();
    // parse input
    for l in contents.lines() {
        if l == "" {continue;}
        if l.starts_with("Tile ") {
            current_id = l[5..9].parse::<u32>().unwrap();
            current_line = 0;
            current_tile = [[false; 10]; 10];
        }
        else {
            for (i, c) in l.chars().enumerate() {
                if c == '#' {
                    current_tile[current_line][i] = true;
                }
            }
            current_line += 1;
            if current_line == 10 {
                tiles.insert(current_id, Tile{id: current_id, tile: current_tile, edges: [0; 4]});
            }
        }
    }
    // make a map of all edge ids and their corresponding tiles
    let mut edges = HashMap::new();
    for (_, t) in & mut tiles{
        t.edges[0] = add_edge(t.id, &t.tile[0], &mut edges);
        t.edges[2] = add_edge(t.id, &t.tile[9], &mut edges);
        let mut l = [false; 10];
        let mut r = [false; 10];
        for i in 0..10 {
            l[i] = t.tile[i][0];
            r[i] = t.tile[i][9];
        }
        t.edges[3] = add_edge(t.id, &l, &mut edges);
        t.edges[1] = add_edge(t.id, &r, &mut edges);
    }
    // find all tiles which have edges without any match
    // edges who have two such edges must be corners!
    let mut edge_tiles = HashSet::new();
    let mut corners = vec!();
    let mut result1 = 1u64;
    for (_k,v) in &edges {
        //println!("{}: {:?}", v.len(), v);
        if v.len() == 1 {
            if !edge_tiles.insert(v[0]) {
                result1 *= v[0] as u64;
                //println!("Found corner tile: {}", v[0]);
                corners.push(v[0]);
            }
        }
    }
    println!("Result1 = {}", result1);

    let top_left = corners[0];
    // bring tl into position so its unused edges are facing t and l
    if edges.get(&(tiles.get(&top_left).unwrap().edges[3].abs() as u32)).unwrap().len() == 2 {
        tiles.get_mut(&top_left).unwrap().flip_lr();
    }
    if edges.get(&(tiles.get(&top_left).unwrap().edges[0].abs() as u32)).unwrap().len() == 2 {
        tiles.get_mut(&top_left).unwrap().flip_tb();
    }
    // start arranging tiles
    let mut arranged_id = vec!(vec!(top_left));
    // arrange left column
    let mut last_tile_id = top_left;
    loop {
        let shared_edge = tiles.get(&last_tile_id).unwrap().edges[2];
        let next_tile_id = {
            let edge_neighbors = edges.get(&(shared_edge.abs() as u32)).unwrap();
            if edge_neighbors[0] != last_tile_id {edge_neighbors[0]} else {edge_neighbors[1]}
        };
        let next_tile = tiles.get_mut(&next_tile_id).unwrap();
        while next_tile.edges[0].abs() != shared_edge.abs() {
            next_tile.rotate();
        }
        if next_tile.edges[0] == -shared_edge {
            next_tile.flip_lr();
        }
        arranged_id.push(vec!(next_tile_id));
        if corners.contains(&next_tile_id) {
            break;
        }
        else {
            last_tile_id = next_tile_id;
        }
    }
    //println!("{:?}", arranged_id);
    //arrange rows
    let nb_tiles_per_row = tiles.len() / arranged_id.len();
    println!("{} x {} tiles", arranged_id.len(), nb_tiles_per_row);
    for row in & mut arranged_id {
        let mut last_tile_id = row[0];
        while row.len() < nb_tiles_per_row {
            let shared_edge = tiles.get(&last_tile_id).unwrap().edges[1];
            let next_tile_id = {
                let edge_neighbors = edges.get(&(shared_edge.abs() as u32)).unwrap();
                if edge_neighbors[0] != last_tile_id {edge_neighbors[0]} else {edge_neighbors[1]}
            };
            let next_tile = tiles.get_mut(&next_tile_id).unwrap();
            while next_tile.edges[3].abs() != shared_edge.abs() {
                next_tile.rotate();
            }
            if next_tile.edges[3] == -shared_edge {
                next_tile.flip_tb();
            }
            row.push(next_tile_id);
            last_tile_id = next_tile_id;
        }
    }
    //println!("{:?}", arranged_id);
    // compose the picture!
    let mut picture = [[false; 12*8]; 12*8];
    for x in 0..arranged_id.len() {
        for y in 0..arranged_id[0].len() {
            let tile = &tiles.get(&arranged_id[x][y]).unwrap().tile;
            for xx in 0..8 {
                for yy in 0..8 {
                    picture[x*8+xx][y*8+yy] = tile[xx+1][yy+1];
                }
            }
        }
    }
    // draw the swamp!
    for x in 0..picture.len() {
        let mut s = String::new();
        for y in 0..picture[x].len() {
            s.push(if picture[x][y] {'#'} else {' '});
        }
        println!("{}", s);
    }
    // let nb_trues = picture.iter().fold(0, |acc, row| acc + row.iter().fold(0, |acc, b| if *b {acc+1}else {acc}));
    // println!("There are {} points in the picture", nb_trues);

    // build all horizonal nessie variations
    let mut nessies = vec!();
    for reversed in &[false, true] {
        for upsidedown in &[false, true] {
            let mut nessie = [[false; 20]; 3];
            for x in 0..3 {
                for y in 0..20 {
                    nessie[x][y] = NESSIE[if *upsidedown {2-x} else {x}][if *reversed {19-y} else {y}];
                }
            }
            nessies.push(nessie);
        }
    }

    // points which are not part of nessie
    let mut not_part_of_nessie = picture;
    // strike out all nessies!
    for x in 0..picture.len()-2 {
        for y in 0..picture[0].len()-2 {
            for n in &nessies {
                check_nessie(&picture, & mut not_part_of_nessie, x, y, true, n);
                check_nessie(&picture, & mut not_part_of_nessie, x, y, false, n);
            }
        }
    }
    // WE DID IT! Lets count and be done!
    let nb_not_nessie = not_part_of_nessie.iter().fold(0, |acc, row| acc + row.iter().fold(0, |acc, b| if *b {acc+1}else {acc}));
    println!("There are {} points not part of nessie", nb_not_nessie);

    // Lets draw it again for celebration!
    for x in 0..picture.len() {
        let mut s = String::new();
        for y in 0..picture[x].len() {
            s.push(if picture[x][y] {if not_part_of_nessie[x][y] {'#'} else {'O'}} else {' '});
        }
        println!("{}", s);
    }
}
