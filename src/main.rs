use rand::Rng;
use textplots::{Chart, Plot, Shape};

fn generate_temperatures(days: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..days)
        .map(|_| rng.gen_range(-15.0..35.0))
        .collect()
}

fn clean_data(data: &[f32]) -> Vec<f32> {
    data.iter().cloned().filter(|&t| t >= -30.0 && t <= 50.0).collect()
}

fn analyze(data: &[f32]) -> (f32, f32, f32) {
    let avg = data.iter().sum::<f32>() / data.len() as f32;
    let min = *data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max = *data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    (avg, min, max)
}

fn main() {
    let raw_temps = generate_temperatures(30);
    let temps = clean_data(&raw_temps);

    let (avg, min, max) = analyze(&temps);

    println!("Analysoitiin {} päivää", temps.len());
    println!("Keskiarvo: {:.1}°C", avg);
    println!("Minimi: {:.1}°C, Maksimi: {:.1}°C", min, max);

    println!("\nLämpötilojen jakautuma (päivä vs. lämpötila):");
    let chart_data: Vec<(f32, f32)> = temps
        .iter()
        .enumerate()
        .map(|(i, &t)| (i as f32, t))
        .collect();

    Chart::new(80, 20, 0.0, 30.0)
        .lineplot(&Shape::Lines(&chart_data))
        .display();
}
