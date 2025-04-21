fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

#[derive(Debug, Clone)]
struct Engine {
    engine_type: String,
    power: f32,
    rpm: u32,
    weight: f32,
}

impl Engine {
    fn new(engine_type: &str, power: f32, rpm: u32, weight: f32) -> Engine {
        Engine {
            engine_type: engine_type.to_string(),
            power,
            rpm,
            weight,
        }
    }

    fn specific_power(&self) -> f32 {
        self.power / self.weight
    }

    fn set_rpm(&mut self, new_rpm: u32) {
        self.rpm = new_rpm;
    }

    fn is_high_power(&self) -> bool {
        self.power > 100.0
    }

    fn get_type(&self) -> String {
        self.engine_type.clone()
    }
}

fn main() {
    println!("1. Транспонування матриці:");
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];
    println!("Оригінальна матриця: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("Транспонована матриця: {:#?}", transposed);

    println!("\n2. Робота зі структурою Engine:");
    let mut engine1 = Engine::new("V8", 150.0, 5000, 250.0);
    
    println!("Двигун: {:?}", engine1);
    println!("Питома потужність: {:.2} кВт/кг", engine1.specific_power());
    println!("Високопотужний: {}", engine1.is_high_power());
    println!("Тип двигуна: {}", engine1.get_type());
    
    engine1.set_rpm(6000);
    println!("Оновлені оберти: {} об/хв", engine1.rpm);

    println!("\n3. Сортування масиву двигунів:");
    let mut engines = vec![
        Engine::new("V6", 120.0, 5500, 200.0),
        Engine::new("Inline-4", 80.0, 6000, 150.0),
        Engine::new("V8", 200.0, 5000, 300.0),
    ];

    println!("До сортування:");
    for engine in &engines {
        println!("{:?}", engine);
    }

    engines.sort_by(|a, b| a.power.partial_cmp(&b.power).unwrap());
    
    println!("\nПісля сортування за потужністю:");
    for engine in &engines {
        println!("{:?}", engine);
    }
}