// // enum Item{
// //     Weapon { damage: i32 },
// //     Potion { healing: i32 },
// //     QuestItem { description: String },
// // }

// // struct Characeter{
// //     name: String,
// //     inventory: Vec<Item>,
// // }

// // impl Characeter {
// //     fn new(name: String) -> Characeter{
// //         Characeter {
// //             name,
// //             inventory: Vec::new(),
// //         }
// //     }
// //     fn add_item(&mut self, item: Item){
// //         self.inventory.push(item);
// //     }
    
// //     fn show_inventory(&self){
// //         println!("{}'s inventory:", self.name);
// //         for item in &self.inventory{
// //             match item {
// //                 Item::Weapon { damage } => println!("Weapon with damage: {}", damage),
// //                 Item::Potion { healing } => println!("Potion with healing: {}", healing),
// //                 Item::QuestItem { description } => println!("Quest item: {}", description),
// //             }
// //         }
// //     }
// // }

// enum Order{
//     Pizza{srednica :i32},
//     Burger{frytki: bool},
//     Napoj{nazwa :String}
// }

// struct Paragon{
//     table_number: i32,
//     list_of_orders: Vec<Order>,
// }

// impl Paragon{
//     fn new(table_number: i32) -> Paragon{
//         Paragon{
//             table_number,
//             list_of_orders: Vec::new(),
//         }
//     }
//     fn add_order(&mut self, order: Order){
//         self.list_of_orders.push(order);
//     }

//     fn show_paragon(&self){
//         println!("Paragon for table number: {}", self.table_number);
//         for order in &self.list_of_orders{
//             match order{
//                 Order::Pizza{srednica} => println!("Pizza with diameter: {}", srednica),
//                 Order::Burger{frytki} => println!("Burger with fries: {}", frytki),
//                 Order::Napoj{nazwa} => println!("Drink: {}", nazwa),
//             }
//         }
//     }
// }

// fn main(){
//     // let mut character = Characeter::new(String::from("Gandalf"));
//     // character.add_item(Item::Weapon { damage: 50 });
//     // character.add_item(Item::Potion { healing: 30 });
//     // character.add_item(Item::QuestItem { description: String::from("Magiczny Przedmiot") });
    
//     // character.show_inventory();    

//     let mut paragon = Paragon::new(5);  
//     paragon.add_order(Order::Pizza { srednica: (30) });
//     paragon.add_order(Order::Burger { frytki: true });
//     paragon.add_order(Order::Napoj { nazwa: (String::from("cola")) });

//     paragon.show_paragon();   
// }

// // enum Urzadzenia{
// //     Lampa{brightness: i32},
// //     Termostat{temperature: f32},
// //     Glosnik{nazwa: String},
// // }

// // struct Pokoj{
// //     nazwa: String,
// //     lista_urzadzen: Vec<Urzadzenia>,
// // }

// // impl Pokoj{
// //     fn new(nazwa: String) -> Pokoj{
// //         Pokoj{
// //             nazwa,
// //             lista_urzadzen: Vec::new()
// //         }
// //     }
// //     fn add_Urzadzenie(&mut self, urzadzenie: Urzadzenia){
// //         self.lista_urzadzen.push(urzadzenie);
        
// //     }
// //     fn show_stan_pokoju(&self){
// //         println!("Raport dla pomieszczenia: {}", self.nazwa);

// //         for urzadzenie in &self.lista_urzadzen {
// //             match urzadzenie{
// //                 Urzadzenia::Lampa{brightness} => println!("Poziom jasnosci lampy: {}", brightness),
// //                 Urzadzenia::Termostat{temperature} => println!("temperatura to: {}", temperature),
// //                 Urzadzenia::Glosnik{nazwa} => println!("nazwa muzyki to: {}", nazwa),
// //             }
// //         }
// //     }
// // }

// // fn main(){
// //     let mut pokoj = Pokoj::new(String::from("salon"));

// //     pokoj.add_Urzadzenie(Urzadzenia::Lampa{brightness: 50});
// //     pokoj.add_Urzadzenie(Urzadzenia::Termostat{temperature:23.0});
// //     pokoj.add_Urzadzenie(Urzadzenia::Glosnik{nazwa:String::from("eloelo320")});

// //     pokoj.show_stan_pokoju();
    
// // }

// fn pobierz_rabat(klient: &str) -> Option<f64>{
//     if klient == "VIP" {
//         Some(20.0)
//     } else {
//         None
//     }
// }

// fn zrealizuj_platnosc(kwota: f64, srodki: f64) -> Result<f64, String>{
//     if srodki < kwota {
//         Err(String::from("Odrzucono: Niewsystarczajace srodki"))
//     } else {
//         let nowe_saldo = srodki - kwota;
//         Ok(nowe_saldo)
//     }
// }

// fn main() {
//     let mut cena = 100.0;
//     let budzet = 90.0;

//     let rabat = pobierz_rabat("VIP");

//     match rabat {
//         Some(rabat) => {
//             cena = cena - (cena * rabat / 100.0);
//         }
//         None => {}
//     }

//     match zrealizuj_platnosc(cena, budzet) {
//         Ok(nowe_saldo) => println!("Platnosc zrealizowana. Nowe saldo: {}", nowe_saldo),
//         Err(e) => println!("{}", e),
//     }
// }

// fn pobierz_stan(produkt: &str) -> Option<u32>{
//     if produkt == "klawiatura"{
//         Some(15)}
//         else if produkt == "myszka"{
//             Some(40)
//          } else {
//              None
//          }
// }

// fn przetworz_zamowienie(produkt: &str, ilosc: u32) -> Result<u32, String>{
//     let stan = pobierz_stan(produkt);

//     match stan {
//         Some(dostepne) => {
//             if dostepne >= ilosc{
//                 Ok(dostepne - ilosc)
//             } else {
//                 Err(String::from("Odrzucono: Niewystarczajacy stan magazynowy"))
//             } 
            
//     }
//     None => {
//                 Err(String::from("Odrzucono: Produkt niedostepny"))
//         }
    
//     }
// }

// fn main() {
//     match przetworz_zamowienie("klawiatura", 5){
//         Ok(nowy_stan) => println!("Zamowienie zrealizowane. Nowy stan magazynowy: {}", nowy_stan),
//         Err(e) => println!("{}", e),
//     }
//     match przetworz_zamowienie("klawiatura", 50) {
//         Ok(pozostalo) => println!("Zamówienie ok! Zostało w magazynie: {}", pozostalo),
//         Err(e) => println!("Błąd: {}", e),
//     }

//     match przetworz_zamowienie("monitor", 1) {
//         Ok(pozostalo) => println!("Zamówienie ok! Zostało w magazynie: {}", pozostalo),
//         Err(e) => println!("Błąd: {}", e),
//     }
// }

// fn znajdz_wiek(email: &str) -> Option<u32>{
//     if email == "anna@wp.pl" {
//         Some(21)
//     } else if email == "piotr@wp.pl" {
//         Some(15)
//     } else {
//         None
//     }
// }

// fn sprawdz_wiek(wiek: u32, wymagany_wiek: u32) -> Result<String, String>{
//     if wiek >= wymagany_wiek{
//         Ok(String::from("Dostep przyznany"))
//     } else {
//         Err(String::from("wydarzenie od 18 roku zycia, za niski wiek"))
//     }
// }

// fn zarezerwuj(email: &str, wymagany_wiek: u32) -> Result<String, String> {
//     let wiek_opcja = znajdz_wiek(email);
    
//     match wiek_opcja {
//         Some(wiek) => {
//             sprawdz_wiek(wiek, wymagany_wiek)
//         }
//         None => {
//             Err(String::from("Nieznany klient w bazie"))
//         }
//     }
// }

        
// fn main(){
//     let wymagany = 18;
    
//     match zarezerwuj("anna@wp.pl", wymagany){
//         Ok(a) => println!("Suckes {}", a),
//         Err(err) => println!("blad {}", err),
//     }
//     match zarezerwuj("piotr@wp.pl", wymagany) {
//         Ok(msg) => println!("[Piotr] Sukces: {}", msg),
//         Err(err) => println!("[Piotr] Błąd: {}", err),
//     }

//     match zarezerwuj("nieznany@wp.pl", wymagany) {
//         Ok(msg) => println!("[Nieznany] Sukces: {}", msg),
//         Err(err) => println!("[Nieznany] Błąd: {}", err),
//     }
// }


// enum RodzajPojazdu{
//     Osobowy{miejsca: u32},
//     Dostawczy{ladownosc_kg: f32},
//     Elektryk{zasieg_km: u32},
// }

// struct Samochod{
//     id: u32,
//     model: String,
//     rodzaj: RodzajPojazdu,
//     dostepny: bool,
// }

// struct Wypozyczalnia{
//     nazwa: String,
//     flota: Vec<Samochod>,
// }

// impl Wypozyczalnia{
//     fn new(nazwa: String) -> Self{
//         Wypozyczalnia{
//             nazwa,
//             flota: Vec::new(),
//         }
//     }
//     fn dodaj_pojazd(&mut self, auto: Samochod){
//         self.flota.push(auto);
//     }
//     fn znajdz_pojazd(&self, id: u32) -> Option<&Samochod>{
//         for auto in &self.flota{
//             if auto.id == id{
//                 return Some(auto);
//             }
//         }
//         None
//     }
//     fn wypozycz(&mut self, id: u32) -> Result<String, String>{
//         for auto in &mut self.flota{
//             if auto.id == id{
//                 if auto.dostepny{
//                     auto.dostepny = false;
//                     return Ok(String::from("Pojazd został wypożyczony"));
//                 } else {
//                     return Err(String::from("Pojazd jest niedostępny"));
//                 }
//             }
//         }
//         Err(String::from("Pojazd o ID nie został znaleziony"))
//     }
// }

// fn main(){
//     let mut wypozyczalnia = Wypozyczalnia::new(String::from("CityCars"));

//     wypozyczalnia.dodaj_pojazd(Samochod{
//         id: 1,
//         model: String::from("Toyota Corolla"),
//         rodzaj: RodzajPojazdu::Osobowy{miejsca: 5},
//         dostepny: true,
//     });

//     wypozyczalnia.dodaj_pojazd(Samochod {
//         id: 2,
//         model: String::from("Tesla Model 3"),
//         rodzaj: RodzajPojazdu::Elektryk { zasieg_km: 450 },
//         dostepny: true,
//     });

//     let _ = wypozyczalnia.wypozycz(1);

//     // Test 1: Próba wypożyczenia już zajętego auta (ID 1) -> Oczekujemy Err
//     println!("--- Test ponownego wypożyczenia (ID 1) ---");
//     match wypozyczalnia.wypozycz(1) {
//         Ok(msg) => println!("Zaskoczenie, poszło: {}", msg),
//         Err(err) => println!("Zgodnie z planem błąd: {}", err),
//     }

//     // Test 2: Wypożyczenie poprawnego, dostępnego auta (ID 2) -> Oczekujemy Ok
//     println!("--- Test poprawnego wypożyczenia (ID 2) ---");
//     match wypozyczalnia.wypozycz(2) {
//         Ok(msg) => println!("Sukces! Info: {}", msg),
//         Err(err) => println!("Błąd: {}", err),
//     }
// }


// enum Ocena {
//     Cyfrowa(f64),
//     Zwolnienie,
// }

// struct OcenaWpis {
//     przedmiot: String,
//     ocena: Ocena,
//     waga: f64,
// }

// struct Dziennik {
//     student: String,
//     wpisy: Vec<OcenaWpis>,
// }

// impl Dziennik {
//     fn new(student: String) -> Self {
//         Dziennik {
//             student,
//             wpisy: Vec::new(),
//         }
//     }
    
//     fn dodaj_wpis(&mut self, wpis: OcenaWpis) {
//         self.wpisy.push(wpis);
//     }

//     fn oblicz_srednia_wazona(&self) -> Result<f64, String> {
//         let mut suma_wazona = 0.0;
//         let mut suma_wag = 0.0;

//         for wpis in &self.wpisy {
//             match wpis.ocena {
//                 Ocena::Cyfrowa(val) => {
//                     suma_wazona += val * wpis.waga;
//                     suma_wag += wpis.waga;
//                 }
//                 Ocena::Zwolnienie => {
//                     // Ignorujemy zwolnienie
//                 }
//             }
//         } // <-- Pętla kończy się tutaj!

//         // Sprawdzenie robimy PO zakończeniu pętli, gdy mamy zsumowane wszystko
//         if suma_wag == 0.0 {
//             return Err(String::from("Brak ocen do wyliczenia średniej"));
//         }

//         // Dzielenie zamiast odejmowania i poprawna nazwa zmiennej suma_wag
//         Ok(suma_wazona / suma_wag)
//     }
// }

// fn main() {
//     // Spójna nazwa zmiennej: dziennik
//     let mut dziennik = Dziennik::new(String::from("Jan Kowalski"));

//     dziennik.dodaj_wpis(OcenaWpis {
//         przedmiot: String::from("math"),
//         ocena: Ocena::Cyfrowa(4.5), // poprawiono przecinek na kropkę
//         waga: 3.0,
//     });

//     dziennik.dodaj_wpis(OcenaWpis {
//         przedmiot: String::from("Physics"),
//         ocena: Ocena::Cyfrowa(3.0),
//         waga: 2.0,
//     });

//     dziennik.dodaj_wpis(OcenaWpis {
//         przedmiot: String::from("Art"),
//         ocena: Ocena::Zwolnienie,
//         waga: 1.0,
//     });
    
//     println!("=== Test 1: Pełny dziennik ===");
//     match dziennik.oblicz_srednia_wazona() {
//         Ok(srednia) => println!("Średnia ważona dla studenta {}: {:.2}", dziennik.student, srednia),
//         Err(e) => println!("Błąd obliczeń: {}", e),
//     }

//     // --- Test 2: Pusty dziennik ---
//     let pusty_dziennik = Dziennik::new(String::from("Anna Nowak"));

//     println!("\n=== Test 2: Pusty dziennik ===");
//     match pusty_dziennik.oblicz_srednia_wazona() {
//         Ok(srednia) => println!("Średnia: {}", srednia),
//         Err(e) => println!("Zgodnie z planem błąd: {}", e),
//     }
// }

// mod test;

// fn main(){
//     test::kwadrat(5);
//     println!("{}", test::kwadrat(5));
// }

// fn analizuj_liczbe(mut n: u32) -> (u32, u32) {
//     let mut suma = 0;
//     let mut odwrocona = 0;

//     while n > 0 {
//         let ostatnia_cyfra = n % 10;
//         suma += ostatnia_cyfra;
//         odwrocona = (odwrocona * 10) + ostatnia_cyfra;
//         n /= 10;
//     }

//     // Użyj while n > 0. 
//     // Pamiętaj: odwrocona = (odwrocona * 10) + ostatnia_cyfra

//     (suma, odwrocona)
// }

// enum Status{
//     DoZrobienia,
//     WTrakcie,
//     Zakonczone,
// }

// struct Zadanie{
//     tytul: String,
//     tresc: String,
//     aktualny_status: Status,
// }

// struct ListaZadan{
//     elementy: Vec<Zadanie>,
// }

// impl ListaZadan{
//     fn new() -> Self{
//         ListaZadan{
//             elementy: Vec::new(),
//         }
//     }
    
//     fn dodaj_zadanie(&mut self, zadanie: Zadanie){
//         self.elementy.push(zadanie);
//     }

//     fn znajdz_zadanie(&self, szukany_tytul: &str) -> Option<&Zadanie>{
//         for z in &self.elementy {
//             if z.tytul == szukany_tytul{
//                 return Some(z);
//             }
//         }
//         None
//     }
// }

// fn main(){
//     let mut lista = ListaZadan::new();

//     lista.dodaj_zadanie(Zadanie{
//         tytul: String::from("nauka rusta"),
//         tresc: String::from("powtorzyc petle"),
//         aktualny_status: Status::DoZrobienia,
//     });

//     let wynik = lista.znajdz_zadanie("nauka rusta");

//     match wynik{
//         Some(z) => println!("znalazlem: {}", z.tytul),
//         None => println!("nie znalazlem zadania"),
//     }
// }

// impl Magazyn{
//     fn new() -> Self{
//         Magazyn{
//             towary : Vec::new(),
//         }
//     }
//     fn dodaj_produkt(&mut self, produkt: Produkt ){
//         self.towary.push(produkt);
//     }
// }

// enum Gatunek{
//     Fantastyka,
//     Kryminal,
//     IT,
// }

// struct Ksiazka{
//     tytul: String,
//     autor: String,
//     gatunek: Gatunek,
// }

// struct Biblioteka{
//     ksiazki: Vec<Ksiazka>,
// }

// impl Biblioteka{
//     fn new() -> Self{
//         Biblioteka{
//             ksiazki: Vec::new(),
//         }
//     }
    
//     fn dodaj_ksiazke(&mut self, ksiazka: Ksiazka){
//         self.ksiazki.push(ksiazka);
//     }

//     fn szukaj_po_tytule(&self, szukany_tytul: &str) -> Option <&Ksiazka>{
//         for z in &self.ksiazki{
//             if z.tytul == szukany_tytul{
//             return Some(z);
//             }
//         }
//         None
//     }
// }
    
// fn main(){
//     let mut biblioteka = Biblioteka::new();

//     biblioteka.dodaj_ksiazke(Ksiazka{
//         tytul: String::from("lalka"),
//         autor: String::from("prus"),
//         gatunek: Gatunek::IT,                   
//     });

//     let wynik = biblioteka.szukaj_po_tytule("lalka");

//     match wynik {
//         Some(z) => println!("tytul tej ksiazki to: {}", z.tytul),
//         None => println!("nie znaleziono ksiazki"),
//     }
// }


// fn sprawdz_pelnoletnosc() -> Result<bool, String>{
//     let wiek = pobierz_wiek()?;

//     if wiek >= 18 {
//         Ok(true)
//     } else {
//         Ok(false)
//     }

    
// }


// trait Pojazd{
//     fn predkosc_max(&self) -> u32;
// }

// struct Rower;

// impl Pojazd for Rower{
//     fn predkosc_max(&self) -> u32 {
//         25
//     }
// }


// let liczby = vec![5, 12, 8, 20, 3];

// let wynik: Vec<i32> = liczby.iter().filter(|&x| x > 10).map(|x| x + 1).collect();

// use std::ops::Add;

// struct Wynik {
//     gole_nasze: u32,
//     gole_ich: u32,
// }

// impl Add for Wynik{
//     type Output = Wynik;

//     fn add(self, other: Wynik) -> Wynik{
//         Wynik{
//             gole_nasze: self.gole_nasze + other.gole_nasze,
//             gole_ich: self.gole_ich + other.gole_ich,
//         }
//     }
// }

// fn mniejsza_wartosc<T: PartialOrd>(a: T, b: T) -> T{
    
// }
// use std::ops::Add;

// struct Towar{
//     nazwa: String,
//     cena: u32,
// }

// impl Add for Towar{
//     type Output = Towar;

//     fn add(self, other: Towar) -> Towar{
//         Towar{
//             nazwa: String::from("Zestaw"),
//             cena: self.cena + other.cena,
//         }
//     }
// }

// struct Koszyk{
//     lista_zakupow: Vec<Towar>,
// }

// impl Koszyk{
//     fn new() -> Self{
//         Koszyk{
//             lista_zakupow: Vec::new()
//         }
//     }

//     fn dodaj_towar(&mut self, towar: Towar){
//         self.lista_zakupow.push(towar);
//     }

//     fn ceny_premium(&self) -> Vec<u32>{
//         self.lista_zakupow.iter().filter(|t| t.cena > 100).map(|t| t.cena).collect()
//     }
// // }

// fn czy_parzysta(liczba: i32) -> bool {
//     liczba % 2 == 0
// }

// #[cfg(test)]
// mod tests{
//     use super::*;

//     #[test]
//     fn test_parzystosci(){
//         assert_eq!(czy_parzysta(4), true);
//     }
// }

// trait Figura{
//     fn pole(&self) -> u32;
// }

// struct Kwadrat {
//     bok: u32
// }

// impl Figura for Kwadrat {
//     fn pole(&self) -> u32 {
//         self.bok * self.bok
//     }
// }

// struct Prostokat{
//     a: u32,
//     b: u32
// }

// impl Figura for Prostokat{
//     fn pole(&self) -> u32{
//         self.a * self.b
//     }
// }

// fn main(){
//     let mut figury: Vec<Box<dyn Figura>> = Vec::new();

//     figury.push(Box::new(Kwadrat{bok: 4}));
//     figury.push(Box::new(Prostokat{a:2, b:5}));

//     for f in figury{
//         println!("Pole wynosi: {}", f.pole());
//     }
    
// }


// fn suma_kwadratow(n: u32, cache: &mut(u32, u32)) -> u32{
//     if cache.0 == n{
//         return cache.1;
//     }
//     if n == 0{
//         return 0;
//     }
//     let wynik = n * n + suma_kwadratow(n - 1, cache);

//     cache.0 = n;
//     cache.1 = wynik;

//     wynik
// }

// fn ocne_haslo(haslo: &str) -> String{
//     let dlugosc = haslo.chars().count();

//     let czesc1 = if dlugosc < 8{
//         "krotkie"
//     } else if dlugosc <=12 {
//         "srednie"
//     } else {
//         "dlugie"
//     }
//     let ma_cyfre = haslo.chars().any(|c| c.is_ascii_digit());
        
//     let czesc2 = if ma_cyfre{
//         "silne"
//     } else {
//         "slabe"
//     };

//     format!("{},{}", czesc1, czesc2)
// }

// fn szukaj_parzystych(n: u32, k: u32) -> u32{
//     let mut szukana = n;

//     loop{
        
//     }
// }

// fn main() {
//     let mut pamiec = (0, 0); 
//     println!("Liczymy: {}", suma_kwadratow(3, &mut pamiec)); 
//     println!("Z cache: {}", suma_kwadratow(3, &mut pamiec)); 
// }

//     println!("{}", ocen_haslo("haslo123")); 
//     println!("{}", ocen_haslo("krotkie"));


// #[derive(Debug)]
// enum Stan{
//     Dostepny,
//     Wypozyczony,
//     WNaprawie,
// }
// #[derive(Debug)]
// enum Blad{
//     NieZnaleziono,
//     NieDostepny,
//     ZaMlody,
// }

// struct Pojazd{
//     nr_rej: String,
//     rok_produkcji: i32,
//     stan_pojazdu: Stan
// }

// struct Flota{
//     auta: Vec<Pojazd>,
// }

// impl Flota{
//     fn new() -> Self {
//         Self { 
//             auta: Vec::new() 
//         } 
//     }
    
//     fn dodaj_pojazd(&mut self, auto: Pojazd){
//         self.auta.push(auto);
//     }

//     fn znajdz_mut(&mut self, nr_rej: &str) -> Option<&mut Pojazd>{
//         self.auta.iter_mut().find(|x| x.nr_rej == nr_rej)
//     }

//     fn wypozycz(&mut self, nr_rej: &str, wiek: u8) -> Result<(), Blad>{
//         if wiek < 18{
//             return Err(Blad::ZaMlody);
//         }
//         let auto = self.znajdz_mut(nr_rej).ok_or(Blad:: NieZnaleziono)?;

//         match auto.stan_pojazdu{
//             Stan::Dostepny => {
//                 auto.stan_pojazdu = Stan::Wypozyczony;
//                 Ok(())
//             }
//             _ => Err(Blad::NieDostepny),
//         }
//     }
// }

// fn main(){
//     let mut oscars = Flota::new();

//     oscars.dodaj_pojazd(Pojazd{
//         nr_rej: String::from("LU996TL"),
//         rok_produkcji: 2016,
//         stan_pojazdu: Stan::Dostepny
//     });

//     oscars.dodaj_pojazd(Pojazd{
//         nr_rej: String::from("LU812SW"),
//         rok_produkcji: 2018,
//         stan_pojazdu: Stan::WNaprawie
//     });

//     println!("Test 1: {:?}", oscars.wypozycz("LU996TL", 16));

//     println!("Test 2: {:?}", oscars.wypozycz("LU812SW", 22));

//     println!("Test 3: {:?}", oscars.wypozycz("LU112SW", 22));

//     println!("Test 4: {:?}", oscars.wypozycz("LU996TL", 22));

//     println!("Test 5: {:?}", oscars.wypozycz("LU996TL", 22));
// }
