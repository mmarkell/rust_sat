use encoder;
use std::vec::Vec;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn solve_formula((formula, width, height) : (encoder::FORMULA, i32, i32)) -> (Vec<i32>, Vec<i32>, i32, i32) {
    let ((positive, negative), successful) = solve(formula_and_variables(formula));
    if !successful {
        println!("unsat");
        return (Vec::new(), Vec::new(), width, height);
    } else {
        return (positive, negative, width, height);
    }
}

pub fn solve((mut formula, mut used_vars, mut new_vars): (encoder::FORMULA, Vec<i32>, Vec<i32>)) -> ((Vec<i32>, Vec<i32>), bool) {
    formula = propagate_units(formula);
    let (formula, _) = pure_literal_elimination(formula);
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
            let new_vars_copy = new_vars.clone();
            let (x, idx) = get_most_frequent(new_vars_copy);
            let mut old_used = used_vars.clone();
            used_vars.push(x);
            new_vars.remove((idx as usize));
            let left_new = new_vars.clone();
            let right_new = new_vars.clone();
            let ((l, r), s) = solve((encoder::FORMULA {clauses: new_clauses, units: units}, used_vars, left_new));
            if s == true || x == 0 {
                return ((l, r), s);
            } else {
                old_used.push(-1 * x);
                let ((l, r), s) = solve((encoder::FORMULA {clauses: copy_new_clauses, units: copy_units}, old_used, right_new));
                if s {
                    return ((l, r), s)
                }
            }
        }
    }

    return ((Vec::new(), Vec::new()), false);
}

fn get_most_frequent(vars: Vec<i32>) -> (i32, i32) {
    let mut seen: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut idx = 0;
    let mut most_common = 0;
    let mut most_occurrences = 0;
    for var in vars {
        match seen.get(&var) {
            Some(&(occurrences, last_seen)) => {
                seen.insert(var, (occurrences + 1, idx));
                if occurrences > most_occurrences {
                    most_occurrences = occurrences;
                    most_common = var;
                }
            },
            _ => {
                seen.insert(var, (1, idx));
                if 1 > most_occurrences {
                    most_occurrences = 1;
                    most_common = var;
                }
            },
        }
        idx += 1;
    }
    match seen.get(&most_common) {
        Some(&(_, location)) => return (most_common, location),
        _ => return (0, 0),
    }
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
    for clause in clauses {
        if clause.is_empty() {
            return true;
        }
    }
    return false;
}

fn is_satisfiable(formula: encoder::FORMULA, input_vars: Vec<i32>) -> bool {
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

    for unit in formula.units {
        if !(input_vars.contains(&unit)) {
            return false;
        }
    }

    return true;
}

fn formula_and_variables(formula: encoder::FORMULA) -> (encoder::FORMULA, Vec<i32>, Vec<i32>) {
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
    return (encoder::FORMULA {clauses: clauses_clone, units: formula.units }, Vec::new(), variables);
}

fn propagate_units(mut formula: encoder::FORMULA) -> encoder::FORMULA {

    fn contains(clause: Vec<i32>, u: i32) -> bool {
        return clause.iter().any(|v| (*v == u));
    }

    let clauses = formula.clauses.clone();
    for clause in clauses {
        let units = formula.units.clone();
        let mut copy_clause = clause.clone();
        let mut to_remove: Vec<i32> = Vec::new();
        for unit in units {
            let mut copy_clause_2 = clause.clone();
            let mut copy_clause_3 = clause.clone();
            if contains(copy_clause_2, unit) {
                formula.clauses.remove(&copy_clause_3);
                to_remove.clear();
                break;
            } else if contains(copy_clause_3, -1 * unit) {
                to_remove.push(-1 * unit);
            }
        }
        if !to_remove.is_empty() {
            formula.clauses.remove(&copy_clause);
            let length = copy_clause.len();
            for value_to_remove in to_remove {
                let index = match copy_clause.iter().position(|x| *x == value_to_remove) {
                    Some(v) => v,
                    _ => length + 1,
                };
                if index < length {
                    copy_clause.remove(index);
                }
            }
            if copy_clause.len() == 1 {
                formula.units.push(copy_clause[0]);
            } else {
                formula.clauses.insert(copy_clause);
            }
        }
    }
    return encoder::FORMULA {clauses: formula.clauses, units: formula.units};
}

fn pure_literal_elimination(formula: encoder::FORMULA) -> (encoder::FORMULA, Vec<i32>) {

    fn pure_clauses(clauses: HashSet<Vec<i32>>, units: Vec<i32>) -> Vec<i32> {

        let mut non_pure: Vec<i32> = Vec::new();
        let return_clauses = clauses.clone();
        let mut occurrences: HashMap<i32, i32> = HashMap::new();
        {
            for clause in clauses {
                for elem in clause {
                    if occurrences.contains_key(&((elem * -1))) && !(non_pure.contains(&elem)) {
                        non_pure.push(elem);
                        non_pure.push(-1 * elem);
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
                    if !non_pure.contains(&elem) && !units.contains(&(-1 * elem)) {
                        remove.push(elem);
                    }
                }
            }
            return remove;
        }
    }

    fn remove_pure_clauses(mut clauses: HashSet<Vec<i32>>, pure_literals: Vec<i32>, units: Vec<i32>) -> (encoder::FORMULA, Vec<i32>) {

        let new_pure_literals = pure_literals.clone();
        let return_pure_literals = pure_literals.clone();
        let mut new_units = units.clone();

        fn not_in(u: &i32, clause: &Vec<i32>) -> bool {
            return !clause.iter().any(|v| (*v == *u));
        }

        for ref r in pure_literals {
            clauses.retain(|ref k| not_in(r, k));
        }

        new_units.extend(new_pure_literals);
        return (encoder::FORMULA { clauses: clauses, units: new_units }, return_pure_literals);
    }

    let clauses = formula.clauses.clone();
    let units = formula.units.clone();
    let new_clauses = clauses.clone();
    return remove_pure_clauses(new_clauses, pure_clauses(clauses, units), formula.units);
}