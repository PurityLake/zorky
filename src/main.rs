use zorky::console;
use zorky::game;

fn should_quit(val: &String) -> bool {
    val == "#quit#"
}

fn main() {
    let mut console = console::Console::new(String::from("> "));
    let quit_options = [String::from("q"), String::from("quit")];

    let stateMachineController = game::initialise_state();
    let (name, curr) = &stateMachineController[0];

    loop {
        console.read().unwrap();
        let val = curr.invoke(&console.last_result);
        if should_quit(&val) {
            break
        }
        println!("{}", val);
    }
}