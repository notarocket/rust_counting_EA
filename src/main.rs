
use std::cmp;


use rand::Rng;

const population: usize = 200;
const mutate_divide: usize = 2;
const mutate_change: i32 = 4;
const num_genes: usize = 6; //may need to edit player code if increased
const goalNum : i32 = 100;


fn main(){
    

    evolutionary_algorithm();
}



fn evolutionary_algorithm(){


    let mut population_gene = [[0; num_genes];population];
    for x in 0..population{
        population_gene[x] = create_individual();
        
    }
    println!("Population created");

    let mut fitness = evaluate_population(population_gene);

    
    for x in 0..1000{
        let selected_individuals = select_population(population_gene, fitness);
        let mut children = breed_population(selected_individuals[0],selected_individuals[1]);
        children = mutate_offspring(children[0], children[1]);
        let values_to_be_replaced = replace_population(fitness);
        let mut y=0;
        for x in values_to_be_replaced{
            let value = fitness.iter().position(|&r| r == x).unwrap();
            population_gene[value] = children[y];
            fitness[value] = 0.0;
            y+=1;
            
        }
        fitness = [0.0;population];
        fitness = evaluate_population(population_gene);
        let mut low_value = 1.0;
        for x in fitness{
            if x < low_value{
                low_value = x;
            }
        }
        println!("Best fitness: {}", low_value);
    }

    println!("done!");

}

fn create_individual() -> [i32; num_genes]{
    let mut arr = [0; num_genes];
    let mut rng = rand::thread_rng();
    //assign increment number
    arr[0] = rng.gen_range(4..20);
    //assign decreatment number
    println!("{}", arr[0]/2);
    arr[1] = rng.gen_range(1..arr[0]/2);

    //assing lowering areas
    for x in 2..num_genes{
        if arr[x-1] == goalNum-1 {
            arr[x] = 99;
        }
        else{
            arr[x] = rng.gen_range(arr[x-1]..goalNum);
        }

        if arr[x] < arr[x-1]{
            let temp = arr[x];
            arr[x] = arr[x-1];
            arr[x-1] = temp;
        }
    }
    return arr;

}


fn evaluate_population(population_gene: [[i32; num_genes];population]) -> [f64;population]{

    let mut fitness_arr = [0.0; population];
    for x in 0..population{

        for y in 0..population{
            if !run(population_gene[x], population_gene[y]){
                fitness_arr[x] += 1.0;
                fitness_arr[y] -= 1.0;
            }else{
                fitness_arr[y] +=1.0;
                fitness_arr[x] -=1.0;
            }
        }
    }
    for x in 0..population{
        fitness_arr[x] = fitness_arr[x];
    }
    return fitness_arr;


}


fn select_population(population_gene: [[i32; num_genes];population], fitness: [f64; population]) -> [[i32;num_genes];2]{
    let mut low_value = 1.1;
    let mut return_pop: [[i32;num_genes];2] =  [[0;num_genes];2];
    let mut rand = rand::thread_rng();
    for x in fitness{
        if x < low_value{
            low_value = x;
        }
    }
    let low_index = fitness.iter().position(|&r| r == low_value).unwrap();
    return_pop[0] = population_gene[low_index];
    return_pop[1] = population_gene[rand.gen_range(0..population)];
    return return_pop; 
}

fn breed_population(indvidual1: [i32;num_genes], individual2: [i32;num_genes]) -> [[i32;num_genes];2]{
    let mut child1 = [0;num_genes];
    let mut child2 = [0;num_genes];
    for x in 0..num_genes{
        child1[x] = cmp::min(indvidual1[x], individual2[x]);
        child2[x] = cmp::max(indvidual1[x], individual2[x]);
    }
    return [child1,child2];

}

fn mutate_offspring(child1: [i32;num_genes], child2: [i32;num_genes]) -> [[i32;num_genes];2]{
    let mut child1 = child1.clone();
    let mut child2 = child2.clone();
    let mut rng = rand::thread_rng();
    let mut mut_num = rng.gen_range(0..num_genes);
    mut_num = mut_num/mutate_divide;
    for x in 0..mut_num{
        let c = rng.gen_range(1..mutate_change);
        let g = rng.gen_range(1..num_genes);
        let pm: i32 = rng.gen_range(1..10);
        if pm % 2 == 0{
            if child1[g] + c < goalNum-1{
                child1[g] += c;
            }
        }else{
            if child1[g] - c > 0 {
                child1[g] -= c;
            }
        }
    }
    let mut mut_num = rng.gen_range(0..num_genes);
    mut_num = mut_num / mutate_divide;
    for x in 0..mut_num{
        let c = rng.gen_range(1..mutate_change);
        let g = rng.gen_range(1..num_genes);
        let pm: i32 = rng.gen_range(1..10);
        if pm %2 == 0{
            if child2[g] + c < goalNum-1{
                child2[g] += c;
            }
        }else{
            if child2[g] - c > 0 {
                child2[g] -= c;
            }
        }
    }
    return [child1, child2];


}

fn replace_population(fitness: [f64; population]) -> [f64;2]{
    let mut remove_values = [0.0;2];
    let mut temp_fitness = fitness.clone();
    for y in 0..2{
        let mut high_value = 0.0;
        for x in temp_fitness{
            if x > high_value{
                high_value = x;
            }
        }
        let high_index = temp_fitness.iter().position(|&r| r == high_value).unwrap();
        remove_values[y] = temp_fitness[high_index];
        temp_fitness[high_index] = 0.0;
    }
    return remove_values;
} 
    
fn run(p2: [i32; num_genes], p1: [i32; num_genes]) -> bool {

    let mut player_state = "player1";
    let mut goal = 0;

    while goal < 100 {
        if player_state == "player1"{
            goal += player(goal, p1, 99);
            player_state = "player2";
            if goal == 99{
                return false;
            }

        }else if player_state == "player2" {
            goal += player(goal, p2, 99);
            player_state = "player1";
            if goal == 99{
                return true;
            }

        }

    }
    return false; 
}
fn player(goal_pos: i32, gene_data: [i32;num_genes], goal: i32) -> i32{
    let mut rval = gene_data[0]; //gene data 0 = default return value
    for x in 2..num_genes{ //gene data >=2 used for areas where player lowers their total entry
        if rval < gene_data[x]{
            if rval > (goal - goal_pos){
                return 1;
            }
            return rval;
        }
        else{
            rval = rval - gene_data[1]; //gene_data1 = default takeaway value
            if rval <= 0{
                rval = 1;
            }
        }

    } 
    return 1;      
 
}
    

        
    


