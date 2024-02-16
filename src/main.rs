const SECONDS_PER_MINUTE: u32 = 60;

fn main() {
    let total = 30;
    let total_em_segundos = total * SECONDS_PER_MINUTE;
    println!("Trabalhou {} segundos", total_em_segundos);
}
