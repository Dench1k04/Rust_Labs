# Пояснення  
> У цьому файлі представлена програма на Rust, яка демонструє роботу з **файловою системою**: створення, копіювання, пошук, перегляд, видалення файлів і папок.

---

## 1. Створення текстового файлу — `create_text_file(path, content)`

```rust
fn create_text_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)?;
    println!("Створено файл: {}", path);
    Ok(())
}
```

> Функція створює текстовий файл з вказаним шляхом `path` і вмістом `content`.  
> Якщо файл вже існує — він буде перезаписаний.

---

## 2. Створення каталогу — `create_directory(path)`

```rust
fn create_directory(path: &str) -> io::Result<()> {
    fs::create_dir(path)?;
    println!("Створено каталог: {}", path);
    Ok(())
}
```

> Створює нову директорію (папку) за вказаним шляхом.  
> Якщо така директорія вже існує — виникне помилка.

---

## 3. Копіювання файлу — `copy_file(src, dest)`

```rust
fn copy_file(src: &str, dest: &str) -> io::Result<()> {
    fs::copy(src, dest)?;
    println!("Скопійовано файл {} до {}", src, dest);
    Ok(())
}
```

> Копіює файл із `src` до `dest`.  
> Якщо файл-джерело не існує — буде помилка.

---

## 4. Копіювання каталогу — `copy_directory(src, dest)`

```rust
fn copy_directory(src: &str, dest: &str) -> io::Result<()> {
    fs::create_dir_all(dest)?;
    ...
}
```

> Функція копіює директорію рекурсивно:  
> - створює всі вкладені папки,
> - копіює всі файли, зберігаючи структуру.  
> Застосовується до будь-якої вкладеності.

---

## 5. Пошук файлів — `search_files(dir, pattern)`

```rust
fn search_files(dir: &str, pattern: &str) -> io::Result<Vec<PathBuf>> {
    ...
}
```

> Шукає файли в каталозі `dir` (і підкаталогах), назви яких містять підрядок `pattern`.  
> Повертає список відповідних шляхів (`PathBuf`).

---

## 6. Видалення файлу — `delete_file(path)`

```rust
fn delete_file(path: &str) -> io::Result<()> {
    fs::remove_file(path)?;
    println!("Видалено файл: {}", path);
    Ok(())
}
```

> Видаляє файл за вказаним шляхом.

---

## 7. Видалення каталогу — `delete_directory(path)`

```rust
fn delete_directory(path: &str) -> io::Result<()> {
    fs::remove_dir_all(path)?;
    println!("Видалено каталог: {}", path);
    Ok(())
}
```

> Видаляє папку разом з усім її вмістом рекурсивно.

---

## 8. Видалення кількох каталогів — `delete_multiple_directories(paths)`

```rust
fn delete_multiple_directories(paths: &[&str]) -> io::Result<()> {
    for path in paths {
        delete_directory(path)?;
    }
    Ok(())
}
```

> Приймає список шляхів до каталогів і видаляє кожен із них, викликаючи `delete_directory()`.

---

## 9. Перегляд вмісту каталогу — `list_directory(dir)`

```rust
fn list_directory(dir: &str) -> io::Result<()> {
    ...
}
```

> Виводить список файлів і папок у каталозі `dir`, позначаючи їх як `DIR` або `FILE`.

---

## 10. Властивості файлу/каталогу — `file_properties(path)`

```rust
fn file_properties(path: &str) -> io::Result<()> {
    ...
}
```

> Показує:
- тип (файл чи папка),
- розмір у байтах,
- дозволи,
- дату останнього редагування.

---

## 11. Головна функція — `main()`

```rust
fn main() -> io::Result<()> {
    ...
}
```

> В `main()` виконується демонстрація роботи всіх функцій:
- створення файлу та папки,
- копіювання файлу в папку,
- створення вкладеної папки з файлом,
- рекурсивне копіювання каталогу,
- пошук усіх файлів з `"test"` у назві,
- перегляд вмісту,
- перегляд властивостей файлу й каталогу,
- видалення файлу,
- видалення кількох каталогів.
