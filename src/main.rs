use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

// 1. Наш базовый словарь ассоциаций (Слово -> Байт)
// Сюда вы будете вписывать новые слова по мере расширения системы
fn get_base256_map() -> HashMap<&'static str, u8> {
    let mut map = HashMap::new();
    
    // Группа 1: Абсолютные крайности
    map.insert("око", 0x00);
    map.insert("лес", 0xFF);

    // Группа 2: Идеальная симметрия и Чередование
    map.insert("душ", 0x55); // 01010101
    map.insert("шум", 0xAA); // 10101010
    map.insert("бар", 0x3C); // 00111100
    map.insert("рог", 0xC3); // 11000011
    map.insert("боб", 0x66); // 01100110
    map.insert("уши", 0x99); // 10011001

    // Группа 3: Чистые Полубайты
    map.insert("лоб", 0x0F); // 00001111
    map.insert("бок", 0xF0); // 11110000

    // Группа 4: Флаги (Одинокие волки)
    map.insert("дед", 0x01); // 00000001
    map.insert("сын", 0x02); // 00000010
    map.insert("пик", 0x80); // 10000000

    // Технические слова для нашего минимального ELF-файла
    map.insert("шаг", 0x45);
    map.insert("гул", 0x46);
    map.insert("фон", 0x4C);
    map.insert("пар", 0x20);
    map.insert("газ", 0x40);
    map.insert("вес", 0xB0);
    map.insert("тик", 0x2A);
    map.insert("дар", 0xCD);

    map
}

// 2. Функция кодирования: бинарник -> слова
fn encode_file(input_path: &str) -> io::Result<()> {
    let mut file = File::open(input_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Создаем обратную карту для быстрого поиска Слова по Байту
    let base_map = get_base256_map();
    let mut byte_to_word = HashMap::new();
    for (word, byte) in base_map {
        byte_to_word.insert(byte, word);
    }

    for (i, &byte) in buffer.iter().enumerate() {
        // Если слова нет в словаре, выводим временную hex-заглушку (например, x1a)
        if let Some(word) = byte_to_word.get(&byte) {
            print!("{:<8}", word);
        } else {
            print!("{:<8}", format!("x{:02x}", byte));
        }

        // Форматируем по 4 слова в строке
        if (i + 1) % 4 == 0 {
            println!();
        }
    }
    println!();
    Ok(())
}

// 3. Функция декодирования: слова -> бинарник
fn decode_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let mut file = File::open(input_path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let base_map = get_base256_map();
    let mut binary_data = Vec::new();

    // Читаем текст построчно, чтобы можно было оставлять комментарии после ";"
    for line in text.lines() {
        // Отсекаем всё, что идет после точки с запятой (комментарии)
        let clean_line = match line.split_once(';') {
            Some((before, _after)) => before,
            None => line,
        };

        // Разбираем строку на отдельные слова
        for word in clean_line.split_whitespace() {
            let lower_word = word.to_lowercase();
            
            if let Some(&byte) = base_map.get(lower_word.as_str()) {
                binary_data.append(&mut vec![byte]);
            } else if lower_word.starts_with('x') && lower_word.len() == 3 {
                // Если встретили hex-заглушку вида "x1a", парсим её обратно в байт
                if let Ok(byte) = u8::from_str_radix(&lower_word[1..], 16) {
                    binary_data.push(byte);
                }
            } else {
                eprintln!("Ошибка: Слово '{}' не найдено в вашем Base256 словаре!", word);
                std::process::exit(1);
            }
        }
    }

    // Записываем собранный бинарный файл
    let mut out_file = File::create(output_path)?;
    out_file.write_all(&binary_data)?;
    println!("Успех! Скомпилирован файл: {}", output_path);
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Использование:");
        println!("  Кодирование:  {} --encode <входной_бинарник>", args[0]);
        println!("  Компиляция:   {} --decode <входной_текст> <выходной_бинарник>", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "--encode" => encode_file(&args[2])?,
        "--decode" => {
            if args.len() < 4 {
                println!("Ошибка: Для декодирования укажите имя выходного файла.");
                return Ok(());
            }
            decode_file(&args[2], &args[3])?;
        }
        _ => println!("Неизвестный флаг. Используйте --encode или --decode"),
    }

    Ok(())
}
