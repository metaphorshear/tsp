extern crate tsp;
extern crate time; 
use tsp::ecm::*;
use tsp::tsp::*;

fn print_ecm(ecm : &ECM) {
    let n = ecm.ecm.len();
    for i in 0..n {
        eprintln!("{:?}", ecm.ecm[i]);
    }
}

fn time_fn(f : &mut FnMut() ) -> (f64) {
    let start = time::precise_time_s();
    f();
    let end = time::precise_time_s();
    end-start
}

fn print_info(tsp : &mut TSP){
    let t = time_fn(&mut || tsp.brute_force());

    println!("best path: {:?}, cost: {}", tsp.brute_info.borrow().best_path.as_ref().unwrap(),
                                          tsp.brute_info.borrow().best_cost);
    println!("brute force paths checked: {}", tsp.brute_info.borrow().paths_checked);
    println!("Time taken (seconds): {}\n", t);

    let t = time_fn(&mut || tsp.greedy());

    println!("greedy path: {:?}, cost: {}", tsp.greedy_info.borrow().best_path.as_ref().unwrap(),
                                            tsp.greedy_info.borrow().best_cost);
    println!("Time taken (seconds): {}\n", t);

    time_fn(&mut || tsp.monte_carlo(20));
    println!("best monte carlo path: {:?}, cost: {}", tsp.monte_info.borrow().best_path.as_ref().unwrap(),
                                                      tsp.monte_info.borrow().best_cost);
    println!("monte carlo paths checked: {}", tsp.monte_info.borrow().paths_checked);
    println!("Time taken (seconds): {}\n", t);

    let t = time_fn(&mut || tsp.aco(2));
    println!("best path found by ant system: {:?}, cost: {}", tsp.ant_system_info.borrow().best_path
                                                                 .as_ref().unwrap(),
             tsp.ant_system_info.borrow().best_cost);
    println!("paths checked by ant system: {}", tsp.ant_system_info.borrow().paths_checked);
    println!("Time taken (seconds): {}\n", t);

}


fn quick_test(num_cities: usize) {
    let ecm = generate_ecm(num_cities); //12 = 1.5 sec brute force
    let n = num_cities;
    print_ecm(&ecm);
    println!("");
    let mut tsp = TSP::new(ecm);
    print_info(&mut tsp);

}

fn bad_greedy(){
    let greedy_bad_ecm = ECM { ecm: vec![ vec![0, 2000000000, 371, 111, 933, 360, 434, 779],
                                    vec![2000000000, 0, 630, 178, 947, 540, 215, 853],
                                    vec![371, 630, 0, 462, 884, 354, 722, 413],
                                    vec![111, 178, 462, 0, 811, 828, 142, 323],
                                    vec![933, 947, 884, 811, 0, 404, 146, 153],
                                    vec![360, 540, 354, 828, 404, 0, 120, 531],
                                    vec![434, 215, 722, 142, 146, 120, 0, 59],
                                    vec![779, 853, 413, 323, 153, 531, 59, 0] ] };
    print_ecm(&greedy_bad_ecm);
    println!("");
    let mut tsp = TSP::new(greedy_bad_ecm);
    print_info(&mut tsp);
    
}


fn part_a(){
    for n in 1..15 {
        let ecm = generate_ecm(n);
        print_ecm(&ecm);
        let mut tsp = TSP::new(ecm);
        let t = time_fn(&mut || tsp.brute_force() );
        eprintln!("best path: {:?}, cost: {}", tsp.brute_info.borrow().best_path.as_ref().unwrap(),
                  tsp.brute_info.borrow().best_cost);
        println!("{} {}", n, t);
    }
}

fn part_b_and_c(){

    let mut brute_times = vec![0f64; 9];
    let mut greedy_times  = vec![0f64; 9];
    let mut monte_carlo_times =  vec![0f64; 45];
    let mut ant_system_times = vec![0f64; 45];

    let mut total_greedy_q = vec![0f64; 9];
    let mut total_monte_carlo_q = vec![0f64; 45];
    let mut total_ant_system_q = vec![0f64; 45];

    for n in 4..13{
        for t in 0..10{

            let ecm = generate_ecm(n);
            //print_ecm(&ecm);
            let mut tsp = TSP::new(ecm);

            brute_times[n-4] += time_fn(&mut || tsp.brute_force());
            let best = tsp.brute_info.borrow().best_cost;

            greedy_times[n-4] += time_fn(&mut || tsp.greedy());
            total_greedy_q[n-4] += (tsp.greedy_info.borrow().best_cost as f64)/(best as f64);
            
            for i in 0..5 {
                let k = 10u32.pow(i as u32 +1) as usize;
                {
                    let idx = (n-4)*5+i;
                    monte_carlo_times[idx] += time_fn(&mut || tsp.monte_carlo(k));
                    total_monte_carlo_q[idx] += (tsp.monte_info.borrow().best_cost as f64)/(best as f64);

                    ant_system_times[idx] += time_fn(&mut || tsp.aco(k));
                    total_ant_system_q[idx] += (tsp.ant_system_info.borrow().best_cost as f64)/(best as f64);
                } //immutable borrows should now be out of scope
                //reset everything here, since the functions don't assume you want a clean slate
                let mut mi = tsp.monte_info.borrow_mut();
                mi.best_cost = 0;
                mi.paths_checked = 0;
                mi.best_path = None; 

                let mut ac = tsp.ant_system_info.borrow_mut();
                ac.best_cost = 0;
                ac.paths_checked = 0;
                ac.best_path = None;
            }
            eprintln!("Finished trial {} for n={}", t+1, n);
        }

    }

    //printing results

    println!("Brute Force Peformance");
    println!("Num Cities\tTime in seconds");
    for n in 4..13 {
        println!("{}\t{}", n, brute_times[n-4]/10f64);
    }
    println!("Greedy Performance");
    println!("Num Cities\tTime in seconds");
    for n in 4..13 {
        println!("{}\t{}", n, greedy_times[n-4]/10f64);
    }
    for i in 0..5 {
        println!("Monte Carlo Performance for {} iterations", 10u32.pow(i as u32 +1));
        println!("Num Cities\tTime in seconds");
        for n in 4..13 {
            println!("{}\t{}", n, monte_carlo_times[(n-4)*5+i]/10f64);
        }
    }
    for i in 0..5 {
        println!("Ant System Performance for {} iterations", 10u32.pow(i as u32 +1));
        println!("Num Cities\tTime in seconds");
        for n in 4..13 {
            println!("{}\t{}", n, ant_system_times[(n-4)*5+i]/10f64);
        }
    }
    println!("Average Greedy Quality (10 trials)");
    println!("Num Cities\tQuality");
    for n in 4..13 {
        println!("{}\t{}", n, total_greedy_q[n-4]/10f64);
    }

    for i in 0..5 {
        println!("Average Monte Carlo Quality for {} iterations", 10u32.pow(i as u32 +1));
        println!("Num Cities\tQuality");
        for n in 4..13 {
            println!("{}\t{}", n, total_monte_carlo_q[(n-4)*5+i]/10f64);
        }
    }
    for i in 0..5 {
        println!("Average Ant System Quality for {} iterations", 10u32.pow(i as u32 +1));
        println!("Num Cities\tQuality");
        for n in 4..13 {
            println!("{}\t{}", n, total_ant_system_q[(n-4)*5+i]/10f64);
        }
    }
}

fn part_d(trials : u64, monte_pow : usize){
    let mut greedy_times = vec![0f64; 100];
    let mut monte_carlo_times = vec![0f64; 100*monte_pow];
    let mut ant_system_times = vec![0f64; 100];
    
    let mut greedy_cost = vec![0u64; 100];
    let mut monte_carlo_cost = vec![0u64; 100*monte_pow];
    let mut ant_system_cost = vec![0u64; 100];

    let mut n = 10;
    let ns = 50; //n=10 to 500. no time for 1000 trials; ACO is too slow

    while n < 10*ns+1 {
        //here to help estimate when this will finish
        let start = time::precise_time_s();
        for t in 0..trials {
            let ecm = generate_ecm(n);
            let mut tsp = TSP::new(ecm);
            let idx = (n/10)-1;

            greedy_times[idx] += time_fn(&mut || tsp.greedy());
            greedy_cost[idx] += tsp.greedy_info.borrow().best_cost;

            //since num_ants = n, it takes forever to run many iterations of ant systems
            //running with small number of iterations (subject to change)
            ant_system_times[idx] += time_fn(&mut || tsp.aco(2));
            ant_system_cost[idx] += tsp.ant_system_info.borrow().best_cost;

            for i in 0..monte_pow{
                let k = 10u32.pow(i as u32 +1) as usize;
                {
                    let idx = ((n/10) -1)*monte_pow+i;
                    monte_carlo_times[idx] += time_fn(&mut || tsp.monte_carlo(k));
                    monte_carlo_cost[idx] += tsp.monte_info.borrow().best_cost;

                }
                let mut mi = tsp.monte_info.borrow_mut();
                mi.best_cost = 0;
                mi.paths_checked = 0;
                mi.best_path = None;
            }
        }
        let end = time::precise_time_s();
        eprintln!("Finished trials for n={} in {}s", n, end-start);
        //eprintln!("Ant Times: {}", ant_system_times[n/10 -1]/(trials as f64));
        //eprintln!("Ant Cost / Monte Cost: {}", (ant_system_cost[n/10 -1] as f32)/(monte_carlo_cost[(n/10)*monte_pow -1] as f32));
        //eprintln!("Ant Cost / Greedy Cost: {}", (ant_system_cost[n/10 -1] as f32)/(greedy_cost[n/10 -1] as f32));
        n += 10;
    }
    println!("Greedy Algorithm Performance");
    println!("Num Cities\t\tTime in seconds\t\tAverage Cost");
    n = 10;
    while n < 10*ns+1{
        println!("{}\t\t{}\t\t{}", n, greedy_times[n/10 -1]/(trials as f64), greedy_cost[n/10-1]/trials);
        n+=10;
    }
    for i in 0..3{
        println!("Monte Carlo Performance for {} iterations", 10u32.pow(i as u32 +1));
        println!("Num Cities\t\tTime in seconds\t\tAverage Cost");
        n = 10;
        while n < 10*ns+1{
            println!("{}\t\t{}\t\t{}", n, monte_carlo_times[(n/10 -1)*monte_pow+i]/(trials as f64),
                                  monte_carlo_cost[(n/10 -1)*monte_pow+i]/trials );
            n+=10;
        }
    }
    println!("Ant System Performance");
    println!("Num Cities\t\tTime in seconds\t\tAverage Cost");
    n = 10;
    while n < 10*ns+1{
        println!("{}\t\t{}\t\t{}", n, ant_system_times[n/10 -1]/(trials as f64), ant_system_cost[n/10-1]/trials);
        n+=10;
    }


}


fn main() {

    //quick_test(6);
    bad_greedy();
    //part_a();
    //part_b_and_c();
    //part_d(30, 3);

}
