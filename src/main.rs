use rand::{Rng, thread_rng};

struct Gene {
    strategy: Vec<f64>,
    score: u32,
}

impl Gene {
    fn build_random_gene() -> Gene {
        let mut rng = thread_rng();
        let mut a: Vec<f64> = vec![0f64; 6 ];
        a = a.into_iter().map(|_item| { rng.gen_range(0f64..1f64)}).collect();
        let mut gene = Gene {
            strategy: a,
            score: 0u32  
        };
        gene.normalize();
        gene
    }
    fn normalize(&mut self){
        let total : f64 = self.strategy.iter().fold(0f64, |acc,x| {acc+x});
        for item in self.strategy.iter_mut(){
            *item = *item/total;
        }
    }
    fn choose(&self) -> usize {
        //let total : f64 = self.strategy.iter().fold(0f64, |acc, x|{ acc+x});
        let mut rng = thread_rng();
        let mut val = rng.gen_range(0f64..1f64);
        let mut a = 0;
        while a<6 {
            if self.strategy[a] > val {
                break;
            } 
            val-=self.strategy[a];
            a+=1;
        }
        return a
    }
    //fn match(&mut self, &mut other){
    //
    // }
}

fn shuffle_pop(pop : &mut Vec<Gene>){
    pop.sort_by_key(|item| {rand::thread_rng().gen_range(0..1000)});
}





fn main() {
    const GOAL_PROBS : [f64; 6] = [ 0.75, 0.95, 0.85, 0.85, 1.0, 0.95];

    
    println!("Generating Random Kickers Popualtion");
    let mut kickers : Vec<Gene> = Vec::new();
    for i in 0..50 {
        kickers.push(Gene::build_random_gene());
        //println!("{:?}", kickers[i].strategy)
    }

    println!("Generating Random Keepers Popualtion");
    let mut keepers : Vec<Gene> = Vec::new();
    for i in 0..50 {
        keepers.push(Gene::build_random_gene());
        //println!("{:?}", kickers[i].strategy)
    }

    println!("Shuffling populations");
    shuffle_pop(&mut keepers);
    shuffle_pop(&mut kickers);


    for (kicker, keeper) in kickers.iter_mut().zip(keepers.iter_mut()){
        for _i in 0..10000{
            let kick_choice = kicker.choose();
            let keep_choice = keeper.choose();
            if kick_choice == keep_choice {
                keeper.score += 1;
            }
            else { 
                let prob : f64;
                match kick_choice {
                    0 => prob = 0.75,
                    1 => prob = 0.90,
                    2 => prob = 0.85,
                    3 => prob = 0.85,
                    4 => prob = 1.0,
                    _ => prob = 0.9
    
                }
                if rand::thread_rng().gen_range(0f64..1f64) < prob {
                    kicker.score += 1;
                }
            }
               
        }
        println!("{} x {}", kicker.score, keeper.score);
    }



}
