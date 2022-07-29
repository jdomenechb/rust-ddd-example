use std::io::{BufRead, Write};

pub fn read_input<R>(mut input: R) -> String
where
    R: BufRead,
{
    let mut input_text: String = String::new();

    input
        .read_line(&mut input_text)
        .expect("Failed to read input");

    input_text.trim().to_string()
}

pub fn ask_question<R, W>(input: R, mut output: W, question: &str) -> String
where
    R: BufRead,
    W: Write,
{
    writeln!(output, "{}", question).unwrap();

    read_input(input)
}

pub fn menu<R, W, E>(input: R, mut output: W, mut error_output: E, options: Vec<&str>) -> u8
where
    R: BufRead,
    W: Write,
    E: Write,
{
    writeln!(output, "\nMENU").unwrap();
    writeln!(output, "----------------------------------------\n").unwrap();
    writeln!(output, "Please, select an option:").unwrap();

    let mut i = 1;

    for option in options {
        writeln!(output, "\t {}. {}", i, option).unwrap();
        i += 1;
    }

    writeln!(output, "\t 0. Exit").unwrap();
    write!(output, "\nOption: ").unwrap();

    output.flush().expect("Error flushing");

    let option = read_input(input).trim().parse();

    writeln!(output, "\n").unwrap();

    match option {
        Ok(o) => o,
        Err(_) => {
            writeln!(error_output, "ERROR: Please, type a number").unwrap();
            u8::MAX
        }
    }
}

#[cfg(test)]
mod test {
    use crate::presentation::prompt::*;

    #[test]
    fn read_input_ok() {
        let input = b"Something to read\n";

        let result = read_input(&input[..]);

        assert_eq!("Something to read", result)
    }

    #[test]
    fn ask_question_ok() {
        let question = "Something to ask";

        let input = b"Answer\n";
        let mut output = Vec::new();

        let result = ask_question(&input[..], &mut output, question);

        assert_eq!("Answer", result);
        assert_eq!(
            "Something to ask\n".to_string(),
            String::from_utf8(output).unwrap()
        )
    }

    #[test]
    fn menu_valid_option() {
        let input = b"1\n";
        let mut output = Vec::new();
        let mut err = Vec::new();

        let result = menu(&input[..], &mut output, &mut err, vec!["Option 1"]);

        assert_eq!(result, 1);
        assert_eq!(String::from_utf8(err).unwrap(), String::new());
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "
MENU
----------------------------------------

Please, select an option:
\t 1. Option 1
\t 0. Exit

Option: \n
"
            .to_string()
        );
    }

    #[test]
    fn menu_invalid_option() {
        let input = b"INVALID\n";
        let mut output = Vec::new();
        let mut err = Vec::new();

        let result = menu(
            &input[..],
            &mut output,
            &mut err,
            vec!["Option 1", "Option 2"],
        );

        assert_eq!(result, 255);
        assert_eq!(
            String::from_utf8(err).unwrap(),
            "ERROR: Please, type a number\n".to_string()
        );
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "
MENU
----------------------------------------

Please, select an option:
\t 1. Option 1
\t 2. Option 2
\t 0. Exit

Option: \n
"
            .to_string()
        );
    }
}
