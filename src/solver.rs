use input;
use std::vec::Vec;
use std::iter::FromIterator;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn solve_formulas(formulas: Vec<input::FORMULA>) {
    for formula in formulas {
        let ((positive, negative), successful) = solve(formula_and_variables(formula));
        if !successful {
            println!("unsat");
        } else {
            println!("{:?}", positive);
            println!("{:?}", negative);
        }
        println!("\n");
    }
}

pub fn solve((formula, mut used_vars, new_vars): (input::FORMULA, Vec<i32>, Vec<i32>)) -> ((Vec<i32>, Vec<i32>), bool) {
    let (formula, _) = pure_literal_elimination(propagate_units(formula));
    let clauses = formula.clauses.clone();
    let new_clauses = formula.clauses.clone();
    let copy_new_clauses = formula.clauses.clone();
    let units = formula.units.clone();
    let copy_units = formula.units.clone();
    let repeat_vars = used_vars.clone();
    {
        if some_repeats(repeat_vars) {
            return ((Vec::new(), Vec::new()), false); 
        }
    }
    {
        if some_empty_clause(clauses) { 
            return ((Vec::new(), Vec::new()), false); 
        }
    }
    {  
        let satisfiable_vars = used_vars.clone();
        if is_satisfiable(formula, satisfiable_vars)  && new_vars.is_empty() { 
            return (positives_and_negatives(used_vars), true); 
        }
    }
    {
        if !new_vars.is_empty() {
            let x = new_vars[0];
            let mut old_used = used_vars.clone();
            used_vars.push(x);
            let ((l, r), s) = solve((input::FORMULA {clauses: new_clauses, units: units}, used_vars, new_vars[1..].to_vec()));
            if s {
                return ((l, r), s);
            } else {
                old_used.push(-1 * x);
                let ((l, r), s) = solve((input::FORMULA {clauses: copy_new_clauses, units: copy_units}, old_used, new_vars[1..].to_vec()));
                if s {
                    return ((l, r), s)
                }
            }
        }
    }

    return ((Vec::new(), Vec::new()), false); 
}

fn some_repeats(vars: Vec<i32>) -> bool {
    let clone_vars = vars.clone();
    for input in vars {
        if clone_vars.contains(&(-1 * input)) && input != 0 {
            return true;
        }
    }
    return false;
}

fn positives_and_negatives(vars: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut positives = Vec::new();
    let mut negatives = Vec::new();
    for v in vars {
        if v >= 0 {
            if !positives.contains(&v) {
                positives.push(v);
            }
        } else if v < 0 {
            if !negatives.contains(&v) {
                negatives.push(v);
            }
        }
    }
    return (positives, negatives);
}

fn some_empty_clause(clauses: HashSet<Vec<i32>>) -> bool {
    if clauses.len() > 1 {
        for clause in clauses {
            if clause.is_empty() {
                return true;
            }
        }
    }
    return false;
}

fn is_satisfiable(formula: input::FORMULA, input_vars: Vec<i32>) -> bool {
    for clause in formula.clauses {
        let mut some_true = false;
        for variable in clause {
            if !(input_vars.contains(&variable) || input_vars.contains(&(-1 * variable))) {
                return false;
            }
            if input_vars.contains(&variable) {
                some_true = true;
            }
        }
        if !some_true {
            return false;
        }
    }

    let clone_units = formula.units.clone();
    for unit in formula.units {
        if !(input_vars.contains(&unit)) {
            return false;
        }
    }
    
    return true;
}

fn formula_and_variables(formula: input::FORMULA) -> (input::FORMULA, Vec<i32>, Vec<i32>) {
    let clauses_clone = formula.clauses.clone();
    let clause_vars = formula.clauses.clone();
    let unit_vars = formula.units.clone();
    let mut variables = Vec::new();
    for c in clause_vars {
        for v in c {
            if !variables.contains(&(-1 * v)) {
                variables.push(v);
            }
        }
    }
    for u in unit_vars {
        if !variables.contains(&(-1 * u)) {
            variables.push(u);
        }
    }
    variables.dedup();
    return (input::FORMULA {clauses: clauses_clone, units: formula.units }, Vec::new(), variables);
}

fn propagate_units(mut formula: input::FORMULA) -> input::FORMULA {

    fn not_in(u: &i32, clause: &Vec<i32>) -> bool {
        return !clause.iter().any(|v| (*v == *u));
    }

    let units = Vec::from_iter(formula.units.iter().cloned());
    for ref u in units {
        formula.clauses.retain(|ref k| not_in(u, k));
    }

    let new_units = Vec::from_iter(formula.units.iter().cloned());
    return input::FORMULA {clauses: formula.clauses, units: new_units};
}

fn pure_literal_elimination(formula: input::FORMULA) -> (input::FORMULA, Vec<i32>) {

    fn can_remove_cannot_remove(clauses: HashSet<Vec<i32>>) -> (Vec<i32>, Vec<i32>) {

        let mut do_not_remove: Vec<i32> = Vec::new();
        let return_clauses: HashSet<Vec<i32>> = clauses.iter().cloned().collect();
        let mut occurrences: HashMap<i32, i32> = HashMap::new();
        {
            for clause in clauses {
                for elem in clause {
                    if occurrences.contains_key(&((elem * -1))) && !do_not_remove.contains(&elem) {
                        do_not_remove.push(elem);
                        do_not_remove.push(-1 * elem);
                    } else {
                        let value = match occurrences.get(&elem) {
                            Some(v) => v + 1,
                            _ => 1,
                        };
                        occurrences.insert(elem, value);
                    } 
                }
            }
        }

        {
            let mut remove: Vec<i32> = Vec::new();
            for clause in return_clauses {
                for elem in clause {
                    let occurred_once = match occurrences.get(&elem) {
                        Some(v) => *v == 1,
                        _ => false,
                    };

                    if !do_not_remove.contains(&elem) && !occurred_once {
                        remove.push(elem);
                    }
                }
            }
            return (remove, do_not_remove);
        }
    }

    fn clauses_with_no_duplicate_values(mut clauses: HashSet<Vec<i32>>, (to_remove, cannot_remove): (Vec<i32>, Vec<i32>), units: Vec<i32>) -> (input::FORMULA, Vec<i32>) {
        
        let new_remove = Vec::from_iter(to_remove.iter().cloned());
        let return_remove = Vec::from_iter(to_remove.iter().cloned());
        let mut new_units = Vec::from_iter(units.iter().cloned());

        fn not_in(u: &i32, clause: &Vec<i32>) -> bool {
            return !clause.iter().any(|v| (*v == *u));
        }

        for ref r in to_remove {
            clauses.retain(|ref k| not_in(r, k));
        }

        new_units.extend(new_remove);
        new_units.dedup();

        return (input::FORMULA { clauses: clauses, units: new_units }, return_remove);
    }

    let clauses = formula.clauses.clone();
    let new_clauses = clauses.clone();
    return clauses_with_no_duplicate_values(new_clauses, can_remove_cannot_remove(clauses), formula.units);
}