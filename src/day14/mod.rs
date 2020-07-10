use std::collections::HashMap;

use crate::common::*;
use math;

type ReactionStorage = HashMap<String, Reaction>;

#[derive(Clone)]
struct Chemical {
    name: String,
    amount: i64,
}

impl Chemical {
    /*
    7 A
    */
    pub fn parse(input: &str) -> Chemical {
        let tokens = input.split(" ").collect::<Vec<_>>();

        Chemical {
            name: tokens[1].trim().to_string(),
            amount: tokens[0]
                .trim()
                .parse::<i64>()
                .expect("Chemical parse error"),
        }
    }
}

impl std::fmt::Display for Chemical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.amount)
    }
}

#[derive(Clone)]
struct Reaction {
    target_chemical: Chemical,
    source_chemicals: Vec<Chemical>,
}

impl std::fmt::Display for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t<-\t", self.target_chemical).unwrap();
        for source in &self.source_chemicals {
            write!(f, "{} ", source).unwrap();
        }
        std::fmt::Result::Ok(())
    }
}

impl Reaction {
    /*
    7 A, 1 B => 1 C\n
    */
    pub fn parse(input: &str) -> Reaction {
        let tokens = input.split("=>").collect::<Vec<_>>();

        Reaction {
            target_chemical: Chemical::parse(tokens[1].trim()),
            source_chemicals: tokens[0]
                .split(',')
                .map(|t| Chemical::parse(t.trim()))
                .collect(),
        }
    }

    /*
     *   This applies the original amounts multiple times until there at least amount of the target checmical
     *   and returns a new reaction with the new aamunts
     */
    pub fn match_target_amount(self, amount: i64) -> Reaction {
        let mut result = self.clone();

        while result.target_chemical.amount < amount {
            result.target_chemical.amount += self.target_chemical.amount;
            for (index, mut source) in result.source_chemicals.iter_mut().enumerate() {
                source.amount += self.source_chemicals[index].amount;
            }
        }
        result
    }
}

fn parse_reactions(input: &str) -> ReactionStorage {
    input
        .split('\n')
        .map(|token| {
            let r = Reaction::parse(token);
            (r.target_chemical.name.clone(), r)
        })
        .collect::<ReactionStorage>()
}

fn print_chemical_storage(chemicals: &HashMap<String, i64>) {
    println!("---");
    for chemical in chemicals {
        if *chemical.1 != 0 {
            println!("{} {}", chemical.0, chemical.1);
        }
    }
    println!("---");
}

fn process(input: &str, fuel_amount: i64) -> i64 {
    let reactions = parse_reactions(input);

    // for reaction in reactions.values() {
    //     println!("{}", reaction);
    // }

    let mut target_chemicals: HashMap<String, i64> = HashMap::new();
    target_chemicals.insert("FUEL".to_string(), fuel_amount);

    let mut chemical_storage: HashMap<String, i64> = HashMap::new();

    let mut ore_required = 0;

    //print_chemical_storage(&target_chemicals);

    while target_chemicals.values().any(|&amount| amount != 0) {
        let mut additional_chemicals: HashMap<String, i64> = HashMap::new();

        for target_chemical in &mut target_chemicals {
            if target_chemical.0 == "ORE" {
                ore_required += *target_chemical.1;
                *target_chemical.1 = 0;
            } else {
                // Lookup any surplus chemical we might still have in storage
                let stored_chemical_amount = chemical_storage
                    .entry(target_chemical.0.clone())
                    .or_insert(0);

                // If that is enough, remove some and were done with the target chemical
                if target_chemical.1 <= stored_chemical_amount {
                    *stored_chemical_amount -= *target_chemical.1;
                    *target_chemical.1 = 0;
                } else {
                    // If its not enough let some reaction process until we have enough (keeping the surplus we might have in mind)
                    let reaction = reactions
                        .get(target_chemical.0)
                        .expect("Weird, no reaction found")
                        .clone()
                        .match_target_amount(*target_chemical.1 - *stored_chemical_amount);

                    //println!("Reaction procesed: {}", reaction);

                    // Add the processed chemicals to the stored ones. These should be picked up in the next iteration
                    *stored_chemical_amount += reaction.target_chemical.amount;

                    for chemical in reaction.source_chemicals {
                        let c = additional_chemicals.entry(chemical.name).or_insert(0);
                        *c += chemical.amount
                    }
                }
            }
        }

        for chemical in additional_chemicals {
            let target_chemical = target_chemicals.entry(chemical.0).or_insert(0);
            *target_chemical += chemical.1;
        }

        //print_chemical_storage(&target_chemicals);
    }

    ore_required
}

pub fn test() {
    let ore = process("10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL",1);
    println!("ORE required : {}", ore);
    let ore = process("9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL",1);
    println!("ORE required : {}", ore);

    let ore = process("157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",1);
    println!("ORE required : {}", ore);

    let ore = process(
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF",
        1,
    );
    println!("ORE required : {}", ore);

    let ore = process(
        "171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX",
        1,
    );
    println!("ORE required : {}", ore);
}

pub fn part1() {
    let input = import_lines("src/day14/input.txt");
    let ore = process(&input, 1);
    println!("ORE required : {}", ore);
    assert_eq!(ore, 892207);
}

pub fn part2() {
    let input = import_lines("src/day14/input.txt");

    let mut min_fuel_amount: i64 = 1935202; // Empirical, < needs 1 trillion ore
    let mut max_fuel_amount: i64 = 1935330; // Empirical, > needs 1 trillion ore

    let mut fuel_amount: i64 = (max_fuel_amount + min_fuel_amount) / 2;
    loop {
        let ore = process(&input, fuel_amount);

        println!(
            "ORE required for {} fuel : {}\t(min:{} max:{})",
            fuel_amount, ore, min_fuel_amount, max_fuel_amount
        );
        if ore > 1000000000000 {
            max_fuel_amount = fuel_amount;
        } else {
            min_fuel_amount = fuel_amount;
        }

        if min_fuel_amount >= max_fuel_amount - 1 {
            assert_eq!(ore, 999999660128);
            assert_eq!(fuel_amount, 1935265);

            break;
        }
        fuel_amount = (min_fuel_amount + max_fuel_amount) / 2;
    }
}
