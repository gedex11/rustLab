
fn liczba_wystapien(napis: &str, znak: char) -> usize {
        let mut licznik = 0;
        for c in napis.chars(){
            if c == znak {
            licznik += 1;
        }
    }
    licznik
}

fn wartosc_znaku(znak: char) -> i32 {
    match znak {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

fn rzymskie(napis: &str) -> i32 {
    let mut suma = 0;
    let mut poprzednia_wartosc = 0;
    
    for znak in napis.chars().rev(){
        let  aktualna_wartosc = wartosc_znaku(znak);
        
        if aktualna_wartosc < poprzednia_wartosc{
            suma-= aktualna_wartosc;
        } else {
            suma += aktualna_wartosc;
        }
        
        poprzednia_wartosc = aktualna_wartosc;
    
    }
    suma
}


fn main() {
    let tekst = "costam haha";
    let szukany_zank = 'a';
    
    let wynik = liczba_wystapien(tekst, szukany_zank);
    println!("zank '{}' wsytepuje {} razy", szukany_zank, wynik);

    
    println!("III = {}", rzymskie("III"));     // 3
    println!("IX = {}", rzymskie("IX"));       // 9
    println!("XIX = {}", rzymskie("XIX"));     // 19
    println!("MCMX = {}", rzymskie("MCMX"));
    
}