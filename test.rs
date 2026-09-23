pub fn kwadrat(x: i32) -> i32{
    x*x
}

fn silnia(n: u32, cache: &mut (u32, u32)) -> u32{
    if n == cache.0 {
        return cache.1;
    } 
    let wynik = if n == 0 {
        1
    } else {
        n * silnia(n - 1, cache)
    };
    cache.0 = n;
    cache.1 = wynik;
    wynik
}

fn klasyfikuj(x: i32) -> String {
    let znak = if x > 0 {
        "Dodatnia"
    } else if x < 0 {
        "Ujemna"
    } else {
        "Zero"
    };

    let parzystosc = if x % 2 == 0 {
        "Parzysta"
    } else {
        "Nieparzysta"
    };

    format!("{}, {}", znak, parzystosc)
}

fn najmniejsza(n: u32, s: u32) -> u32 {
    let mut aktualna = n;
    
    loop {
        let mut kopia = aktualna;
        let mut suma_cyfr = 0;
        
        while kopia > 0 {
            suma_cyfr += kopia % 10;
            kopia /= 10;
        }
        
        if suma_cyfr >= s {
            return aktualna;
        }
        
        aktualna += 1;
    }
}
fn odleglosc_hamminga(s1: &str, s2: &str) -> f64 {
    s1.chars()
        .zip(s2.chars())
        .enumerate()
        .filter(|(_, (c1, c2))| c1 != c2)
        .map(|(i, _)| 1.0 / ((i + 1) as f64))
        .sum()
}
    fn main(){
    let mut cache = (0, 1);
    let n = 5;
    silnia(n, &mut cache);
    println!("Silnia z {} wynosi {}", n, cache.1);

    println!("Liczba 10 jest {}", klasyfikuj(10));

    
}