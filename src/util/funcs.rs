pub fn parse_passes(args: &[String], counter: usize) -> u32 {
    let next_arg = match args.get(counter + 1) {
        Some(arg) => arg,
        None => {
            println!(
                "srm: missing number of passes after '{}'\nTry 'srm --help' for more information.",
                args[counter]
            );
            std::process::exit(0);
        }
    };

    match next_arg.parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!(
                "srm: invalid number of passes '{}'\nTry 'srm --help' for more information.",
                next_arg
            );
            std::process::exit(0);
        }
    }
}
