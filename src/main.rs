use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_add_char_to_pangram({
        let ui_handle = ui.as_weak();
        move |ch| {
            let ui = ui_handle.unwrap();
            ui.set_pangram(format!("{}{ch}", ui.get_pangram()).into());
            ui.invoke_update();
        }
    });

    ui.on_update({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            match encrypt(format!("{}\n", ui.get_pangram().to_string()), ui.get_input().to_string(), ui.get_action() == 1) {
                Ok (v) => {
                    ui.set_has_error(false);
                    ui.set_output(SharedString::from(v));
                },
                Err (e) => {
                    ui.set_has_error(true);
                    ui.set_missing_char(SharedString::from(e.character));
                },
            }
        }
    });

    ui.run()
}

pub struct MissingCharacterError {
    pub character: char
}

fn encrypt(pangram: String, text: String, decrypt: bool) -> Result<String, MissingCharacterError> {
    Ok(
        text.chars()
            .map(|character| {
                let idx = pangram.chars().enumerate().find(|(_, ch)| *ch == character).ok_or(MissingCharacterError {character})?.0;
                Ok::<_, MissingCharacterError>(
                    if decrypt {
                        pangram.chars().nth(if idx >= 1 { idx - 1 } else { 0 }).unwrap()
                    } else {
                        pangram.chars().nth(idx + 1).unwrap_or(pangram.chars().nth(0).unwrap())
                    }
                )
            })
            .collect::<Result<String, _>>()?
    )
}
