use ecm::*;
use aco::*;
use rand::{thread_rng, Rng};
use std::cell::RefCell;

pub struct TSPInfo {
    pub best_path : Option< Vec<u32> >, //Option allows value to be None, or Some
    pub best_cost : u64,
    pub paths_checked : u64
}

pub struct TSP {
    pub ecm : ECM,
    pub brute_info : RefCell<TSPInfo>,
    pub greedy_info : RefCell<TSPInfo>,
    pub monte_info : RefCell<TSPInfo>,
    pub ant_system_info : RefCell<TSPInfo>
}

impl TSP {
    pub fn new(ecm : ECM) -> TSP {
        TSP{ ecm : ecm,
        brute_info : RefCell::new(TSPInfo{best_path: None, best_cost: 0, paths_checked: 0 }),
        greedy_info : RefCell::new(TSPInfo{best_path: None, best_cost: 0, paths_checked: 0 }),
        monte_info : RefCell::new(TSPInfo{best_path: None, best_cost: 0, paths_checked: 0 }),
        ant_system_info : RefCell::new(TSPInfo{best_path: None, best_cost: 0, paths_checked: 0 }),
        }
    }

    pub fn update_best(&self, path : &[u32], info: &RefCell<TSPInfo>) {
        let new_cost = self.ecm.path_cost(path);
        let mut info = info.borrow_mut();
        //eprintln!("path: {:?}, cost: {}", path, new_cost);
        info.paths_checked += 1;
        match info.best_path {
            None => { info.best_cost = new_cost; info.best_path = Some(path.to_vec());},
            _    => {
                        if new_cost < info.best_cost {
                            info.best_path = Some(path.to_vec());
                            info.best_cost = new_cost;
                        }
                    }
       }
    }

    fn brute(&mut self, path: &mut Vec<u32>, size: usize){
        //Heap's Algorithm, based on implementation at Geeks for Geeks
        //initial size should be one less than path size, to leave final num untouched
        //(assuming home node is at beginning and end of path)
        if size == 1 {
            self.update_best(&path[..], &self.brute_info);
            return;
        }
        for i in 1..size{ //this starts at 1 to skip the home node
            self.brute(path, size-1);
            if size % 2 == 1 {
                path.swap(1, size-1);
            }
            else {
                path.swap(i, size-1);
            }
        }
    }
    pub fn brute_force(&mut self){
        //wrapper to make things less confusing/more convenient
        let n = self.ecm.ecm.len();
        let mut path : Vec<u32> = (0u32..(n as u32) ).collect();
        path.push(0u32); //home node is 0, so it begins and ends path
        self.brute(&mut path, n);
    }

    pub fn greedy(&mut self){
        let n = self.ecm.ecm.len();
        let mut strike : Vec<u32> = (1u32..(n as u32) ).collect();
        let mut path : Vec<u32> = Vec::with_capacity(n);
        //let mut cost : u64 = 0;
        path.push(0u32);
        while strike.len() > 0 {
            let prev = *path.last().unwrap() as usize;
            let mut idx = 0;
            let next = { 
                let mut least = strike[0] as usize;
                for i in 1..strike.len(){
                    let cur = strike[i] as usize;
                    //check the cost to go from the last node to this one, vs last node to current winner
                    if self.ecm.ecm[prev][cur] < self.ecm.ecm[prev][least] { idx = i; least = cur; }
                }
                least
            };
            //cost += self.ecm.ecm[prev][next] as u64;
            path.push(next as u32);
            strike.swap_remove(idx);
        }
        //again, assuming 0 is the home node, and adding it to the end of the path
        //let end = *path.last().unwrap() as usize;
        //cost += self.ecm.ecm[end][0] as u64;
        path.push(0u32);
        self.update_best(&path[..], &self.greedy_info);
    }

    pub fn monte_carlo(&mut self, trials: usize){
        let mut rng = thread_rng();
        let n = self.ecm.ecm.len();
        let src : Vec<u32> = (1u32..(n as u32) ).collect();
        for _ in 0..trials {
            let mut path = Vec::with_capacity(n+1);
            path.extend(src.iter().cloned());
            rng.shuffle(&mut path);
            path.insert(0, 0);
            path.push(0);
            //eprintln!("monte carlo path: {:?}", &path);
            self.update_best(&path[..], &self.monte_info);
        }
    }
    pub fn ant_system(&mut self, num_ants : usize, pweight : f32, hweight : f32, 
                      decay : f32, iterations: usize)
    {
        ///Implementation of an Ant System, based on the code and descriptions in Clever Algorithms
        ///pweight corresponds to alpha (the history coefficient), usually set to 1
        ///hweight corresponds to beta (the heuristic coefficient), usually set between 2 and 5
        ///decay is usually set to 0.5
        ///number of ants should be number of cities

        let num_cities = self.ecm.ecm.len();

        let mut initial_path : Vec<u32>  = (0u32..(num_cities as u32) ).collect();
        initial_path.push(0);
        self.update_best(&initial_path[..], &self.ant_system_info); //note that one path is checked before the algo begins

        //since I need the cost of every path for ACO, I'll track the best in this fn and update later
        let mut best = (initial_path, self.ant_system_info.borrow().best_cost as f32);

        let initial_strength = (num_cities as f32)/(best.1);

        let mut pheromones = ACO::new(num_cities, initial_strength, pweight, hweight, decay);

        for n in 0..iterations {
            let mut paths : Vec<(Vec<u32>, f32)> = Vec::with_capacity(num_ants);
            for _ in 0..num_ants {
                //calculate probability of choosing each city based on pheromone strength and cost
                //use calculation as probability distribution to choose (randomly) the next path
                let path = pheromones.select_path(&self.ecm);
                let cost = self.ecm.path_cost(&path[..]) as f32;
                if best.1 > cost { best = (path.clone(), cost); }
                { self.ant_system_info.borrow_mut().paths_checked += 1; }
                paths.push((path, cost));
            }
            pheromones.decay();
            pheromones.update(&paths);
            //eprintln!("iteration {}, best path: {:?}, cost: {}", n+1, best.0, best.1);
        }
        self.update_best(&best.0[..], &self.ant_system_info);
        self.ant_system_info.borrow_mut().paths_checked -= 1;

    }

    pub fn aco(&mut self, iterations: usize) {
        ///This is a wrapper around ant_system, which sets sane values
        ///You can still use the other one if you want to tweak everything
        let pweight = 1f32;
        let hweight = 2.5f32;
        let num_ants = self.ecm.ecm.len();
        let decay = 0.5f32;
        self.ant_system(num_ants, pweight, hweight, decay, iterations);
    }
}




