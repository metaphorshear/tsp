use ecm::ECM;
use rand::{thread_rng, Rng};

pub struct ACO {
    pub pheromones : Vec< Vec<f32> >, //represents the strength of pheromone trails between edges
    pub pweight : f32, //importance of pheromones in decision-making (larger value = more important)
    pub hweight : f32, //importance of cost in decision-making (larger value = less important)
    pub decay : f32
}

impl ACO {
    pub fn new(grid_size: usize, initial_strength: f32, pweight : f32, hweight : f32,
               decay : f32) -> ACO {
        /// grid_size : the number of nodes in the graph
        /// initial strength : every edge in the pheromone matrix is initialized to this value
        /// try num_cities/path_cost, where path_cost is the total cost of the first path

        ACO { pheromones: vec![ vec![initial_strength; grid_size]; grid_size ],
              pweight: pweight, hweight : hweight, decay : decay }
    }

    fn calculate_choices(&self, ecm : &ECM, last_city : u32, left: &Vec<u32>) -> Vec<(usize, u32, f32)>
    {
        let mut choices : Vec<(usize, u32, f32)> = Vec::with_capacity(left.len());
        let mut idx = 0;
        while idx < left.len() {
            let p = self.pheromones[last_city as usize][left[idx] as usize].powf(self.pweight);
            let h = (ecm.ecm[last_city as usize][left[idx] as usize] as f32).powf(self.hweight);
            choices.push( (idx, left[idx], p/h) );
            idx += 1;
        }
        choices
    }

    fn select_city(&self, choices : &Vec< (usize, u32, f32) >) -> (usize, u32)
    {
        //create a probability distribution from the choices calculated earlier
        //that is, divide each "probability" by the sum of all of them
        let sum = choices.iter().map(|x| x.2.clone()).fold(0f32, |acc, x| acc + x);

        //eprintln!("probability of choosing each city:");
        //for city in choices {
        //    eprintln!("{} : {}", city.1, city.2/sum);
        //}
        let mut rng = thread_rng();
        if sum == 0f32 { let choice = rng.choose(&choices[..]).unwrap(); return (choice.0, choice.1) }
        let mut v = rng.next_f32();
        //keep subtracting probabilities from v until v <= 0; this way cities with higher probabilities
        //are more likely, but any city can be chosen
        for choice in choices {
            v -= choice.2/sum;
            if v <= 0f32 { return (choice.0, choice.1) }
        }
        //it shouldn't be possible to get here without returning a choice
        //but just pick something
        (choices[0].0, choices[0].1)
    }

    pub fn select_path(&self, ecm : &ECM) -> Vec<u32> {
        let cities = &ecm.ecm;
        let mut path : Vec<u32> = Vec::with_capacity(cities.len()+1);
        let mut strike : Vec<u32> = (1u32..(cities.len() as u32) ).collect(); 
        path.push(0); //home node is 0
        while strike.len() > 0 {
            let choices = self.calculate_choices(ecm, *path.last().unwrap(), &mut strike);
            let (idx, city) = self.select_city(&choices);
            path.push(city);
            strike.swap_remove(idx);
        }
        path.push(0);
        path
    }

    pub fn decay(&mut self) {
        for i in 0..self.pheromones.len() {
            for j in 0..self.pheromones.len() {
                self.pheromones[i][j] *= 1f32-self.decay;
            }
        }
    }

    pub fn update(&mut self, paths : &Vec<(Vec<u32>, f32)> ) {
        for pc in paths {
            let &(ref path, ref cost) = pc;
            let strength = 1f32/(*cost as f32);
            let mut i = 0; let mut j = 1;
            while j < path.len() {
                self.pheromones[path[i] as usize][path[j] as usize] += strength;
                self.pheromones[path[j] as usize][path[i] as usize] += strength;
                i += 1;
                j += 1;
            }
        }
    }
}


