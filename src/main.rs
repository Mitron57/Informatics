use {
    rand::Rng,
    signal_hook::{consts::*, iterator::Signals},
    std::io::{stdin, Error, ErrorKind, Result},
};

fn read_core(number: &mut String) -> Result<()> {
    match stdin().read_line(number) {
        Ok(bytes) => {
            if bytes == 0 {
                println!("Программа получила Ctrl+D, поток ввода отключен.");
                return Err(Error::from(ErrorKind::UnexpectedEof));
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn read_number() -> Result<usize> {
    let mut buffer = String::new();
    while read_core(&mut buffer).is_ok() {
        match buffer.trim().parse::<usize>() {
            Ok(value) => {
                if (2..=5).contains(&value) {
                    return Ok(value);
                }
                println!("Число должно быть в диапазоне [2, 5], введите число снова:");
            }
            _ => println!("Невозможно преобразовать введенные вами данные в неотрицательное число, введите число:"),
        }
        buffer.clear();
    }
    Err(Error::from(ErrorKind::UnexpectedEof))
}

fn read_answer() -> Result<bool> {
    let mut buffer = String::new();
    while read_core(&mut buffer).is_ok() {
        let trimmed = buffer.trim();
        if trimmed.len() == 1 {
            match trimmed {
                "Y" => return Ok(true),
                "N" => return Ok(false),
                _ => {}
            }
        }
        buffer.clear();
        println!("Введите либо Y, либо N:")
    }
    Err(Error::from(ErrorKind::UnexpectedEof))
}

fn read_string() -> Result<String> {
    let mut buffer = String::new();
    println!("Введите 4-х символьное слово, состоящее из английских букв:");
    while read_core(&mut buffer).is_ok() {
        let trimmed = buffer.trim();
        if trimmed.len() == 4
            && trimmed
                .as_bytes()
                .iter()
                .all(|&e| e.is_ascii_lowercase() || e.is_ascii_uppercase())
        {
            return Ok(buffer.trim().to_string());
        }
        buffer.clear();
        println!("Вы ввели не 4-х символьное слово, введите слово заново:")
    }
    Err(Error::from(ErrorKind::UnexpectedEof))
}

fn random_string() -> Result<String> {
    let mut random = rand::thread_rng();
    let mut word = String::with_capacity(4);
    const ALPHABET: &[u8] = b"QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm";
    for _ in 0..4 {
        word.push(ALPHABET[random.gen_range(0..52)] as char)
    }
    Ok(word)
}

fn read_letter() -> Result<char> {
    let mut buffer = String::new();
    while read_core(&mut buffer).is_ok() {
        let trimmed = buffer.trim();
        if trimmed.len() == 1
            && trimmed
                .as_bytes()
                .iter()
                .all(|&e| e.is_ascii_lowercase() || e.is_ascii_uppercase())
        {
            return Ok(buffer.trim().as_bytes()[0] as char);
        }
        buffer.clear();
        println!("Вы ввели не букву, введите букву:");
    }
    Err(Error::from(ErrorKind::UnexpectedEof))
}

fn main() {
    const CONSONANTS: &[u8] = b"pbkfvmzhtdlnPBKFVMZHTDLN";
    let mut signals = Signals::new([SIGINT, SIGTSTP])
        .expect("Ошибка: невозможно проинициализировать обработчик сигналов");
    std::thread::spawn(move || {
        for signal in signals.forever() {
            match signal {
                SIGINT => println!("\nПрограмма получила Ctrl+C! Программа продолжает работу..."),
                SIGTSTP => println!("\nПрограмма получила Ctrl+Z! Программа продолжает работу..."),
                _ => unreachable!(),
            }
        }
    });
    println!("Введите размер матрицы (целое число в диапазоне [2, 5]):");
    let size = if let Ok(size) = read_number() {
        size
    } else {
        return;
    };
    let mut matrix = vec![Vec::with_capacity(size); size];
    println!(
        "Если вы хотите внести слова самостоятельно, введите в консоль Y, в противном случае - N:"
    );
    let manual = if let Ok(mode) = read_answer() {
        mode
    } else {
        return;
    };
    let fill = if manual { read_string } else { random_string };
    for line in matrix.iter_mut() {
        for _ in 0..size {
            let word = if let Ok(word) = fill() { word } else { return };
            line.push(word)
        }
    }
    println!("Введите букву, которая должна отсутствовать в слове:");
    let letter = if let Ok(letter) = read_letter() {
        letter
    } else {
        return;
    };
    let mut result = Vec::new();
    for line in matrix.iter() {
        for word in line.iter() {
            if !word.contains(letter) {
                result.push(word);
            }
        }
    }
    result.sort_by(|&a, &b| {
        let (a, b) = (a.as_bytes(), b.as_bytes());
        let (mut cnt_a, mut cnt_b) = (0, 0);
        for i in 0..4 {
            if CONSONANTS.contains(&a[i]) {
                cnt_a += 1;
            }
            if CONSONANTS.contains(&b[i]) {
                cnt_b += 1;
            }
        }
        cnt_a.cmp(&cnt_b).reverse()
    });
    println!("Изначальная матрица:");
    for line in matrix.iter() {
        println!("{line:?}");
    }
    println!("\nРезультат: {result:?}");
}
