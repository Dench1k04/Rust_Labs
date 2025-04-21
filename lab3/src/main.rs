use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn create_text_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)?;
    println!("Створено файл: {}", path);
    Ok(())
}

fn create_directory(path: &str) -> io::Result<()> {
    fs::create_dir(path)?;
    println!("Створено каталог: {}", path);
    Ok(())
}

fn copy_file(src: &str, dest: &str) -> io::Result<()> {
    fs::copy(src, dest)?;
    println!("Скопійовано файл {} до {}", src, dest);
    Ok(())
}

fn copy_directory(src: &str, dest: &str) -> io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = Path::new(dest).join(entry.file_name());

        if entry.file_type()?.is_dir() {
            copy_directory(
                src_path.to_str().unwrap(),
                dest_path.to_str().unwrap()
            )?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }
    println!("Скопійовано каталог {} до {}", src, dest);
    Ok(())
}

fn search_files(dir: &str, pattern: &str) -> io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            let sub_results = search_files(path.to_str().unwrap(), pattern)?;
            results.extend(sub_results);
        } else if path.to_str().unwrap().contains(pattern) {
            results.push(path);
        }
    }
    Ok(results)
}

fn delete_file(path: &str) -> io::Result<()> {
    fs::remove_file(path)?;
    println!("Видалено файл: {}", path);
    Ok(())
}

fn delete_directory(path: &str) -> io::Result<()> {
    fs::remove_dir_all(path)?;
    println!("Видалено каталог: {}", path);
    Ok(())
}

fn delete_multiple_directories(paths: &[&str]) -> io::Result<()> {
    for path in paths {
        delete_directory(path)?;
    }
    Ok(())
}

fn list_directory(dir: &str) -> io::Result<()> {
    println!("Вміст каталогу {}:", dir);
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = if entry.file_type()?.is_dir() { "DIR" } else { "FILE" };
        println!("{} - {}", file_type, path.display());
    }
    Ok(())
}

fn file_properties(path: &str) -> io::Result<()> {
    let path = Path::new(path);
    let metadata = path.metadata()?;

    println!("Властивості файлу {}:", path.display());
    println!("Тип: {}", if metadata.is_file() { "Файл" } else { "Каталог" });
    println!("Розмір: {} байт", metadata.len());
    println!("Дозволи: {:?}", metadata.permissions());
    if let Ok(time) = metadata.modified() {
        println!("Останнє редагування: {:?}", time);
    }
    Ok(())
}

fn main() -> io::Result<()> {

    create_text_file("test.txt", "Привіт, світ!")?;
    create_directory("test_dir")?;
    copy_file("test.txt", "test_dir/test_copy.txt")?;
    create_directory("test_dir/sub_dir")?;
    create_text_file("test_dir/sub_dir/file.txt", "Тест")?;
    copy_directory("test_dir", "test_dir_copy")?;
    println!("\nПошук файлів з 'test' у назві:");
    let results = search_files(".", "test")?;
    for path in results {
        println!("Знайдено: {}", path.display());
    }
    println!("\nПерегляд вмісту:");
    list_directory("test_dir")?;
    println!("\nВластивості файлу:");
    file_properties("test.txt")?;
    println!("\nВластивості каталогу:");
    file_properties("test_dir")?;
    delete_file("test_dir/test_copy.txt")?;
    delete_multiple_directories(&["test_dir", "test_dir_copy"])?;

    Ok(())
}