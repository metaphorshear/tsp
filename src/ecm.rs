extern crate rand;
use rand::Rng;

pub struct ECM {
    pub ecm : Vec< Vec<u32> >
}

pub fn generate_ecm(n: usize) -> ECM{
    let mut ecm = vec![ vec![0;n]; n ];
    let mut rng = rand::thread_rng();
    for i in 0..n {
        for j in i+1..n {
            ecm[i][j] = rng.gen_range(1, 1001); //range [1, 1000]
            ecm[j][i] = ecm[i][j];
        }
    }
    ECM{ ecm : ecm }
}

impl ECM {
    pub fn new(n : usize) -> ECM{
        generate_ecm(n)
    }
    pub fn path_cost(&self, path: &[u32]) -> u64{
        //path is a list of vertices/indices into the ecm
        if path.len() < 2 { return 0; }
        let mut cost : u64 = 0;
        //indices into the path
        let mut i = 0;
        let mut j = 1;
        while j < path.len(){
            cost += self.ecm[path[i] as usize][path[j] as usize] as u64;
            i += 1;
            j += 1;
        }
        cost
    }
}
