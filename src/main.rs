use rand::Rng;

// Функція для генерації випадкового вектора з цілих чисел в діапазоні [10..99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    // Створення генератора випадкових чисел
    let mut rng = rand::thread_rng();
    // Створення вектора з випадковими числами в діапазоні від 10 до 99
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Функція для знаходження мінімальної пари сусідніх елементів у векторі
fn min_adjacent_sum(data: &[i32]) -> Option<(i32, (i32, i32), usize, usize)> {
    // Перевірка, чи є вектор достатньо великим для пошуку пари сусідніх елементів
    if data.len() < 2 {
        return None; // Якщо вектор має менше двох елементів, повертаємо None
    }

    // Ініціалізація змінних для мінімальної пари та її суми
    let mut min_sum = data[0] + data[1];
    let mut min_pair = (data[0], data[1]);
    let mut min_indexes = (0, 1);

    // Проходимо по вектору, шукаючи мінімальну суму сусідніх елементів
    for (i, window) in data.windows(2).enumerate() {
        let sum = window[0] + window[1]; // Обчислення суми двох сусідніх елементів
        if sum < min_sum { // Якщо поточна сума менша за мінімальну
            min_sum = sum;
            min_pair = (window[0], window[1]);
            min_indexes = (i, i + 1);
        }
    }

    // Повертаємо мінімальну пару, її суму та індекси
    Some((min_sum, min_pair, min_indexes.0, min_indexes.1))
}

// Функція для виведення результатів пошуку мінімальної пари сусідніх елементів
fn print_min_adjacent_sum(data: &[i32]) {
    // Якщо ми знайшли мінімальну пару, виводимо інформацію
    if let Some((sum, (a, b), idx1, idx2)) = min_adjacent_sum(data) {
        // Виведення вектора даних
        println!("data:  {:?}", data);

        // Виведення індексів з підкресленою мінімальною парою
        let mut highlighted_indexes = String::new();
        for i in 0..data.len() {
            // Додаємо спеціальне підкреслення для індексів мінімальної пари
            if i == idx1 {
                highlighted_indexes.push_str("\\__ ");
            } else if i == idx2 {
                highlighted_indexes.push_str("__/ ");
            } else {
                highlighted_indexes.push_str("    ");
            }
        }
        // Виведення рядка з індексами
        println!("indexes:{}", highlighted_indexes);

        // Виведення мінімальної пари та її суми
        println!("min adjacent sum={}+{}={} at indexes:{},{}", a, b, sum, idx1, idx2);
    } else {
        // Якщо немає достатньо елементів для пошуку мінімальної пари
        println!("Немає достатньо елементів для пошуку мінімальної пари.");
    }
}

// Головна функція
fn main() {
    // Генерація випадкового вектора довжиною 20 і виведення мінімальної пари
    let random_vector = gen_random_vector(20);
    print_min_adjacent_sum(&random_vector);

    // Генерація ще кількох випадкових векторів і виведення результатів
    let random_vector2 = gen_random_vector(20);
    print_min_adjacent_sum(&random_vector2);

    let random_vector3 = gen_random_vector(20);
    print_min_adjacent_sum(&random_vector3);

    let random_vector4 = gen_random_vector(20);
    print_min_adjacent_sum(&random_vector4);
}

/*
Функція gen_random_vector(n: usize) -> Vec<i32>:

Створює випадковий вектор довжиною n, заповнений випадковими числами в діапазоні від 10 до 99.
Для цього використовуємо метод gen_range з бібліотеки rand.
Функція min_adjacent_sum(data: &[i32]) -> Option<(i32, (i32, i32), usize, usize)>:

Приймає вектор чисел як вхід і повертає мінімальну пару сусідніх елементів, їх суму і індекси.
За допомогою методу windows(2) створюємо усі пари сусідніх елементів і перевіряємо їх суму.
Якщо сума пари менша за поточну мінімальну суму, оновлюємо мінімальну пару та її індекси.
Функція print_min_adjacent_sum(data: &[i32]):

Виводить вектор чисел (data).
Після цього підкреслює індекси мінімальної пари та виводить її суму разом з індексами.
Якщо вектор має менше ніж два елементи, виводиться повідомлення про помилку.
*/