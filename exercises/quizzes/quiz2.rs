// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TYPES
    // Convienience type
    struct Pair {
        // field ordering should have String first to have better memory alignment
        arg: String,
        com: Command,
    }

    // FUNCTIONS
    fn kernel(pair: Pair) -> String {
        let str = pair.arg;
        let out = match pair.com {
            Command::Uppercase => str.to_uppercase(),
            Command::Trim => str.trim().to_string(),
            Command::Append(post) => str + &post.to_string(),
        };
        return out;
    }

    // call to a kernel function for modularity
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // convert vector of tuples to vector of Pairs
        let pair_vec: Vec<Pair> = input
            .iter()
            .map(|tup| -> Pair {
                Pair {
                    arg: tup.0,
                    com: tup.1,
                }
            })
            .collect();

        pair_vec.iter().map(kernel)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::Command;
    use super::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
