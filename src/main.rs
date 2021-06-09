use rand::{Rng, thread_rng};

struct Gene {
    strategy: Vec<f64>,
    score: u32,
}



fn build_random_gene() -> Gene {
    let mut rng = thread_rng();
    let mut a: Vec<f64> = vec![0f64; 6 ];
    a = a.into_iter().map(|item| { rng.gen_range(0f64..1f64)}).collect();
    //let total : f64 = a.iter().fold(0f64, |acc,x| {acc+x});
    //for item in a.iter_mut(){
    //    *item = *item/total;
    //} 
    let mut gene = Gene {
        strategy: a,
        score: 0u32  
    };
    normalize(&mut gene);
    gene
}

fn normalize(gene : &mut Gene) -> () {
    let total : f64 = gene.strategy.iter().fold(0f64, |acc,x| {acc+x});
    for item in gene.strategy.iter_mut(){
        *item = *item/total;
    }
}

fn main() {
    const GOAL_PROBS : [f64; 6] = [ 0.75, 0.95, 0.85, 0.85, 1.0, 0.95];

    
    println!("Generating Random Kickers Popualtion");
    let mut kickers : Vec<Gene> = Vec::new();
    for i in 0..50 {
        kickers.push(build_random_gene());
        //println!("{:?}", kickers[i].strategy)
    }

    println!("Generating Random Keepers Popualtion");
    let mut keepers : Vec<Gene> = Vec::new();
    for i in 0..50 {
        keepers.push(build_random_gene());
        //println!("{:?}", kickers[i].strategy)
    }

    println!("Shuffling populations")
    keepers.sort_by_key(|item| -> {item.score} )



}
