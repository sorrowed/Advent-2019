use permute;

use crate::cpu::*;

fn input() -> &'static str {
    "3,8,1001,8,10,8,105,1,0,0,21,42,67,88,105,114,195,276,357,438,99999,3,9,101,4,9,9,102,3,9,9,1001,9,2,9,102,4,9,9,4,9,99,3,9,1001,9,4,9,102,4,9,9,101,2,9,9,1002,9,5,9,1001,9,2,9,4,9,99,3,9,1001,9,4,9,1002,9,4,9,101,2,9,9,1002,9,2,9,4,9,99,3,9,101,4,9,9,102,3,9,9,1001,9,5,9,4,9,99,3,9,102,5,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,99"
}

fn find_max_output(input: &str, phase_settings: Vec<i64>) -> i64 {
    // Clean program state
    let program = read(input);

    let mut max_output = std::i64::MIN;

    // Phase input permutations for phase settings
    let phase_inputs = permute::permute(phase_settings);

    for phase_input in phase_inputs {
        // Initialize and execute first phase, set input signal to 0.
        let mut phase = program.clone();
        phase.interactive = false;

        phase.add_input(phase_input[0]);
        phase.add_input(0);
        execute(&mut phase);

        // Execute the rest of the phases, set input signal to output of previous phase.
        for ix in 1..phase_input.len() {
            let mut next_phase = program.clone();
            next_phase.interactive = false;

            next_phase.add_input(phase_input[ix]);
            next_phase.add_input(phase.get_output(0).expect("Error, No output for phase"));
            execute(&mut next_phase);

            // Remember for next phase or final output
            phase = next_phase;
        }

        max_output = std::cmp::max(
            max_output,
            phase.get_output(0).expect("Error, no program output"),
        );
    }

    max_output
}

pub fn test() {
    //let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";

    println!(
        "Maximum output : {}",
        find_max_output(input, vec![0, 1, 2, 3, 4])
    );
}

pub fn part1() {
    println!(
        "Maximum output : {}",
        find_max_output(input(), vec![0, 1, 2, 3, 4])
    );
}

pub fn part2() {
    // let input =
    // 	"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";

    let program = read(input());
    let phase_inputs = permute::permute(vec![9, 8, 7, 6, 5]);

    let mut max_output = std::i64::MIN;

    // Try all permutations of the phase settings
    for phase_input in phase_inputs {
        // Create programs for each amplifier
        let mut amplifiers = [
            program.clone(),
            program.clone(),
            program.clone(),
            program.clone(),
            program.clone(),
        ];

        // Prepare all amplifier programs with 1 input signal, the phase input setting
        for i in 0..amplifiers.len() {
            let amplifier = &mut amplifiers[i];
            amplifier.interactive = false;
            amplifier.add_input(phase_input[i]);
        }

        // Execute the amplifier programs until they are all finished
        let mut last_output = 0;
        loop {
            for i in 0..amplifiers.len() {
                let amplifier = &mut amplifiers[i];

                amplifier.add_input(last_output); // Add the output from the previous amplifier as input signal
                execute(amplifier);

                // Store the output so we can use it as input for the next program
                last_output = amplifier.get_output(0).expect("Error, no output produced");

                // At this point the program is either finished or paused, waiting for input
                // We now reset the input queue, any input needed on subsequent runs is added from the output
                // of the previous amplifier program
                amplifier.flush();
            }

            if amplifiers.iter().all(|p| p.is_finished()) {
                break;
            }
        }

        // We can use last_output here because that will always be the output of amplifier E at this point
        max_output = std::cmp::max(max_output, last_output);
    }

    println!("Maximum output produced by amplifier E: {}", max_output);
}
