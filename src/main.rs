
mod utils;

fn main() {
    let program = utils::read_program("src/day1/p1.txt");
    let input = utils::read_input("src/day1/input.txt").into_iter().map(|x| String::from("K") +&utils::convert_numbers_unary(&x,'a')).collect();
    
    let output = utils::run_with_bumped_input(
        &input,
        program,
        String::from("SttttttttttttttttttttttttttttttttttttttttttttttttttE")
    );
    
    println!("{}", output)
}
