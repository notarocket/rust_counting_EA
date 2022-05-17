use core::num;
use std::cmp;

use rand::Rng;

const population: usize = 200;
const mutate_rate: f64 = 0.05;
const mutate_change: f64 = 0.1;
const num_genes: usize = 3; //may need to edit player code if increased

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

    
    for x in 0..400{
        let mut selected_individuals = select_population(population_gene, fitness);
        let mut children = breed_population(selected_individuals[0],selected_individuals[1]);
        let values_to_be_replaced = replace_population(children, population_gene, fitness);
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
    let mut recomend_number_1 = 0;
    let mut recomend_number_2 = 16;
    for y in 0..num_genes{
        
        if recomend_number_1 > recomend_number_2{
            recomend_number_1 = recomend_number_2 - 7;
        }
        if recomend_number_2 > 19 {
            recomend_number_2 = 19;
        }
        arr[y] = rng.gen_range(recomend_number_1..recomend_number_2);
        recomend_number_1+=6;
        recomend_number_2+=4;

    }
    return  arr;

}


fn evaluate_population(population_gene: [[i32; num_genes];population]) -> [f64;population]{

    let mut fitness_arr = [0.0; population];
    for x in 0..population{

        for y in 0..population{
            if !run(population_gene[x], population_gene[y]){
                fitness_arr[x] += 1.0;
            }
        }
    }
    for x in 0..population{
        fitness_arr[x] = fitness_arr[x]/population as f64;
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

fn replace_population(children:[[i32;num_genes];2], population_gene: [[i32; num_genes];population], fitness: [f64; population]) -> [f64;2]{
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

fn run(p1: [i32; num_genes], p2: [i32; num_genes]) -> bool {

    let mut player_state = "player1";
    let mut goal = 0;

    while goal < 100 {
        let g2 = goal.clone();
        if player_state == "player1"{
            goal += player1(g2, p1);
            player_state = "player2";
            if goal == 99{
                return true
            }
                
            


        }else if player_state == "player2" {
            goal += player2(g2, p2);
            player_state = "player1";
            if goal == 99{
                return false;
            }
        



        }

        
        


    }
    return false; 
}

    fn player1(goal_pos: i32, gene_data: [i32;num_genes]) -> i32{
        return if goal_pos < gene_data[0] {
            3
        } else if (goal_pos < gene_data[2]) & (goal_pos > gene_data[0]-1) //max is 19
        {
            2
        } else {
            1
        }
    }

    fn player2(goal_pos: i32, gene_data: [i32;num_genes]) -> i32{
        return if goal_pos < gene_data[0] {
            3
        } else if (goal_pos < gene_data[2]) & (goal_pos > gene_data[0]-1) //max is 19
        {
            2
        } else {
            1
        }
    }




