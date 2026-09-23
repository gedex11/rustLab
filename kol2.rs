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

mod test;

fn main(){
    test::kwadrat(5);
    println!("{}", test::kwadrat(5));
}