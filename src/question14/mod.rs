pub fn solve_14() {
    let mut recipes = vec![3u32, 7u32];
    let mut first_elf_position = 0usize;
    let mut second_elf_position = 1usize;

    let end_recipe: Vec<u32> = vec![4, 0, 9, 5, 5, 1];
//    let end_recipe: Vec<u32> = vec![1, 4, 7, 0, 6, 1];
//    let end_recipe: Vec<u32> = vec![5, 9, 4, 1, 4];

//    while !ends_with(&recipes, &end_recipe) {
//        combine_recipes(&mut recipes, &mut first_elf_position, &mut second_elf_position);
//    }

    while !ends_with(&recipes, &end_recipe) {
        combine_recipes(&mut recipes, &mut first_elf_position, &mut second_elf_position);
    }

    println!("Recipes: {:?}", &recipes[recipes.len() - 10..]);
    println!("Recipes length: {}", recipes.len());
}

fn ends_with(v: &Vec<u32>, predicate: &Vec<u32>) -> bool {
    if v.len() < predicate.len() + 1 {
        return false;
    }

    for i in 1..=predicate.len() {
        if v[v.len() - i] != predicate[predicate.len() - i] && v[v.len() - i - 1] != predicate[predicate.len() - i] {
            break;
        }
        if i > 4 {
            println!("Matched: {}", predicate[predicate.len() - i]);
        }
    }

    for i in 1..=predicate.len() {
        if v[v.len() - i - 1] != predicate[predicate.len() - i] {
            return false;
        }
        if i > 4 {
            println!("Matched: {}", predicate[predicate.len() - i]);
        }
    }

    true
}

fn combine_recipes(recipes: &mut Vec<u32>, first_elf_position: &mut usize, second_elf_position: &mut usize) {
    let first_elf_recipe = recipes.get(*first_elf_position).unwrap().clone();
    let second_elf_recipe = recipes.get(*second_elf_position).unwrap().clone();

    let sum = first_elf_recipe + second_elf_recipe;
    let split: Vec<char> = sum.to_string().chars().collect();
//    println!("split: {:?}", split);

    recipes.push(split[0].to_digit(10).unwrap());
    if split.len() > 1 {
        recipes.push(split[1].to_digit(10).unwrap());
    }

//    println!("Recipes: {:?}", recipes);

    *first_elf_position = (*first_elf_position + (1 + first_elf_recipe as usize)) % recipes.len();
    *second_elf_position = (*second_elf_position + (1 + second_elf_recipe as usize)) % recipes.len();

//    println!("Positions: {} {} {}", *first_elf_position, *second_elf_position, recipes.len());
}