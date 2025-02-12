Ось приклад робочого коду на Rust, який виконує базову обробку даних - читання з файлу, сортування елементів масиву, знаходження максимуму та мінімуму, та запис результатів у файл.

```rust
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::OpenOptions;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let display = path.display();
    let mut numbers: Vec<i32> = Vec::new();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let number: i32 = line.trim().parse()
            .expect("Expected a number");
        numbers.push(number);
    }

    numbers.sort();

    let min = match numbers.first() {
        Some(min) => min,
        None => panic!("No numbers found"),
    };

    let max = match numbers.last() {
        Some(max) => max,
        None => panic!("No numbers found"),
    };

    let path = Path::new("output.txt");
    let display = path.display();

    let file = match OpenOptions::new().write(true).create(true).open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut writer = io::BufWriter::new(&file);

    write!(writer, "Sorted numbers:\n")?;

    for num in numbers {
        write!(writer, "{}\n", num)?;
    }

    write!(writer, "\nMin: {}\n", min)?;
    write!(writer, "Max: {}\n", max)?;

    Ok(())
}
```

Цей код виконує наступні дії:
- Він відкриває вхідний файл `input.txt`, зчитує з нього числа, а потім закриває файл.
- Потім він сортує числа в порядку зростання.
- Знаходить мінімальне і максимальне число в списку.
- Він відкриває вихідний файл `output.txt`, записує в нього відсортовані числа, мінімальне і максимальне число, а потім закриває файл.