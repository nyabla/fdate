pub struct RepublicanDate {
    year: i64,  // Year
    month: u8,  // Month: 1..13
    day: u8,    // Day: 1..30
    decade: u8, // Decade: 1..3 (0 if month = 13)
    weekday: u8,// Weekday: 1..10
    rural: u16, // Day of year: 1..360 (0 if month = 13)
    dyear: u16, // Day of year: 1..366
    time: (u8, u8, u8), // Decimal time (h, m, s)
}

impl RepublicanDate {
    // (year, month, day)
    pub fn new(date: (i64, u8, u8), time: (u8, u8, u8)) -> RepublicanDate {
        let m_u16 = date.1 as u16;
        let d_u16 = date.2 as u16;

        RepublicanDate {
            year: date.0,
            month: date.1,
            day: date.2,
            decade: if date.1 == 13 { 0 } else { date.2 / 10 + 1 },
            weekday: (date.2 - 1) % 10 + 1,
            rural: if date.1 == 13 { 0 } else { (m_u16 - 1) * 30 + d_u16 },
            dyear: (m_u16 - 1) * 30 + d_u16,
            time: time,
        }
    }

    pub fn format_str(&self, fmt_string: &str) -> String {
        let mut result = String::new();
        let mut percent = false;

        for c in fmt_string.chars() {
            if percent {
                match c {
                    '%' => result.push('%'),
                    'A' => if self.month != 13 {
                        result.push_str(WEEKDAYS[self.weekday as usize - 1])
                    } else {
                        result.push_str(SANSCULOTTIDES[self.weekday as usize - 1])
                    },
                    'B' => result.push_str(MONTHS[self.month as usize - 1]),
                    'd' => result.push_str(&format!("{}", self.day)),
                    'H' => result.push_str(&format!("{}", self.time.0)),
                    'j' => result.push_str(&format!("{}", self.dyear)),
                    'J' if self.month != 13 => {
                        result.push_str(RURAL[self.rural as usize - 1])
                    },
                    'm' => result.push_str(&format!("{}", self.month)),
                    'M' => result.push_str(&format!("{}", self.time.1)),
                    'n' => result.push('\n'),
                    'S' => result.push_str(&format!("{}", self.time.2)),
                    't' => result.push('\t'),
                    'u' => result.push_str(&format!("{}", self.weekday)),
                    'w' if self.decade != 0 => {
                        result.push_str(&format!("{}", self.decade))
                    },
                    'W' if self.decade != 0 => {
                        result.push_str(&roman_numerals(self.decade as u64))
                    },
                    'y' => result.push_str(&format!("{}", self.year)),
                    'Y' => {
                        if self.year < 0 { result.push('-'); }
                        result.push_str(&roman_numerals(self.year.abs() as u64))
                    },
                    _ => {},
                }

                percent = false;
            } else {
                percent = c == '%';
                if percent { continue; }
                result.push(c);
            }
        }

        return result;
    }
}

// Yoinked from Don Knuth's tex.web tyy<3
fn roman_numerals(mut number: u64) -> String {
    let numerals = ['M', 'D', 'C', 'L', 'X', 'V', 'I'];
    let quotient = [2, 5, 2, 5, 2, 5];

    let (mut j, mut k); // numeral, quotient
    let (mut u, mut v); // le numbers

    j = 0;
    v = 1000;

    let mut result = String::new();

    loop {
        while number >= v {
            // append numerals[j]
            result.push(numerals[j]);
            number -= v;
        }

        if number <= 0 {
            break;
        }

        k = j + 1;
        u = v / quotient[j];

        if quotient[k - 1] == 2 {
            k += 1;
            u = u / quotient[k - 1];
        }

        if number + u >= v {
            // append numerals[k]
            result.push(numerals[k]);
            number += u;
        } else {
            j += 1;
            v = v / quotient[j - 1];
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    #[test]
    fn fmt() {
        let format = "%%-%A-%B-%d-%H-%j-%J-%m-%M-%n-%S-%t-%u-%w-%W-%y-%Y-%k";
        let date = super::RepublicanDate::new((230, 3, 17), (5, 50, 99));
        assert_eq!(
            date.format_str(&format),
            "%-Septidi-Frimaire-17-5-77-Cypr??s-3-50-\n-99-\t-7-2-II-230-CCXXX-"
        );
    }

    #[test]
    fn roman_numerals() {
        let r = super::roman_numerals(1999);
        assert_eq!(r, "MCMXCIX");
    }
}

const WEEKDAYS: [&str; 10] = [
        "Primidi", "Duodi", "Tridi", "Quarti", "Quintidi",
        "Sextidi", "Septidi", "Octidi", "Nonidi", "D??cadi",
];

const MONTHS: [&str; 13] = [
        "Vend??miaire", "Brumaire", "Frimaire",
        "Niv??se", "Pluvi??se", "Vent??se",
        "Germinal", "Flor??al", "Prairial",
        "Messidor", "Thermidor", "Fructidor",
        "Sansculottides",
];

const SANSCULOTTIDES: [&str; 6] = [
    "La F??te de la Vertu",
    "La F??te du G??nie",
    "La F??te du Travail",
    "La F??te de l'Opinion",
    "La F??te des R??compenses",
    "La F??te de la R??volution",
];

const RURAL: [&str; 360] = [
    "Raisin",
    "Safran",
    "Ch??taigne",
    "Colchique",
    "Cheval",
    "Balsamine",
    "Carotte",
    "Amaranthe",
    "Panais",
    "Cuve",
    "Pomme de terre",
    "Immortelle",
    "Potiron",
    "R??s??da",
    "??ne",
    "Belle de nuit",
    "Citrouille",
    "Sarrasin",
    "Tournesol",
    "Pressoir",
    "Chanvre",
    "P??che",
    "Navet",
    "Amaryllis",
    "B??uf",
    "Aubergine",
    "Piment",
    "Tomate",
    "Orge",
    "Tonneau",
    "Pomme",
    "C??leri",
    "Poire",
    "Betterave",
    "Oie",
    "H??liotrope",
    "Figue",
    "Scorson??re",
    "Alisier",
    "Charrue",
    "Salsifis",
    "M??cre",
    "Topinambour",
    "Endive",
    "Dindon",
    "Chervis",
    "Cresson",
    "Dentelaire",
    "Grenade",
    "Herse",
    "Bacchante",
    "Azerole",
    "Garance",
    "Orange",
    "Faisan",
    "Pistache",
    "Macjonc",
    "Coing",
    "Cormier",
    "Rouleau",
    "Raiponce",
    "Turneps",
    "Chicor??e",
    "N??fle",
    "Cochon",
    "M??che",
    "Chou-fleur",
    "Miel",
    "Geni??vre",
    "Pioche",
    "Cire",
    "Raifort",
    "C??dre",
    "Sapin",
    "Chevreuil",
    "Ajonc",
    "Cypr??s",
    "Lierre",
    "Sabine",
    "Hoyau",
    "??rable ?? sucre",
    "Bruy??re",
    "Roseau",
    "Oseille",
    "Grillon",
    "Pignon",
    "Li??ge",
    "Truffe",
    "Olive",
    "Pelle",
    "Tourbe",
    "Houille",
    "Bitume",
    "Soufre",
    "Chien",
    "Lave",
    "Terre v??g??tale",
    "Fumier",
    "Salp??tre",
    "Fl??au",
    "Granit",
    "Argile",
    "Ardoise",
    "Gr??s",
    "Lapin",
    "Silex",
    "Marne",
    "Pierre ?? chaux",
    "Marbre",
    "Van",
    "Pierre ?? pl??tre",
    "Sel",
    "Fer",
    "Cuivre",
    "Chat",
    "??tain",
    "Plomb",
    "Zinc",
    "Mercure",
    "Crible",
    "Laur??ole",
    "Mousse",
    "Fragon",
    "Perce-neige",
    "Taureau",
    "Laurier-thym",
    "Amadouvier",
    "M??z??r??on",
    "Peuplier",
    "Coign??e",
    "Ell??bore",
    "Brocoli",
    "Laurier",
    "Avelinier",
    "Vache",
    "Buis",
    "Lichen",
    "If",
    "Pulmonaire",
    "Serpette",
    "Thlaspi",
    "Thimel??",
    "Chiendent",
    "Trainasse",
    "Li??vre",
    "Gu??de",
    "Noisetier",
    "Cyclamen",
    "Ch??lidoine",
    "Tra??neau",
    "Tussilage",
    "Cornouiller",
    "Violier",
    "Tro??ne",
    "Bouc",
    "Asaret",
    "Alaterne",
    "Violette",
    "Marceau",
    "B??che",
    "Narcisse",
    "Orme",
    "Fumeterre",
    "V??lar",
    "Ch??vre",
    "??pinard",
    "Doronic",
    "Mouron",
    "Cerfeuil",
    "Cordeau",
    "Mandragore",
    "Persil",
    "Cochl??aria",
    "P??querette",
    "Thon",
    "Pissenlit",
    "Sylvie",
    "Capillaire",
    "Fr??ne",
    "Plantoir",
    "Primev??re",
    "Platane",
    "Asperge",
    "Tulipe",
    "Poule",
    "Bette",
    "Bouleau",
    "Jonquille",
    "Aulne",
    "Couvoir",
    "Pervenche",
    "Charme",
    "Morille",
    "H??tre",
    "Abeille",
    "Laitue",
    "M??l??ze",
    "Cigu??",
    "Radis",
    "Ruche",
    "Gainier",
    "Romaine",
    "Marronnier",
    "Roquette",
    "Pigeon",
    "Lilas",
    "An??mone",
    "Pens??e",
    "Myrtille",
    "Greffoir",
    "Rose",
    "Ch??ne",
    "Foug??re",
    "Aub??pine",
    "Rossignol",
    "Ancolie",
    "Muguet",
    "Champignon",
    "Hyacinthe",
    "R??teau",
    "Rhubarbe",
    "Sainfoin",
    "B??ton d'or",
    "Chamerisier",
    "Ver ?? soie",
    "Consoude",
    "Pimprenelle",
    "Corbeille d'or",
    "Arroche",
    "Sarcloir",
    "Statice",
    "Fritillaire",
    "Bourrache",
    "Val??riane",
    "Carpe",
    "Fusain",
    "Civette",
    "Buglosse",
    "S??nev??",
    "Houlette",
    "Luzerne",
    "H??m??rocalle",
    "Tr??fle",
    "Ang??lique",
    "Canard",
    "M??lisse",
    "Fromental",
    "Martagon",
    "Serpolet",
    "Faux",
    "Fraise",
    "B??toine",
    "Pois",
    "Acacia",
    "Caille",
    "??illet",
    "Sureau",
    "Pavot",
    "Tilleul",
    "Fourche",
    "Barbeau",
    "Camomille",
    "Ch??vrefeuille",
    "Caille-lait",
    "Tanche",
    "Jasmin",
    "Verveine",
    "Thym",
    "Pivoine",
    "Chariot",
    "Seigle",
    "Avoine",
    "Oignon",
    "V??ronique",
    "Mulet",
    "Romarin",
    "Concombre",
    "??chalote",
    "Absinthe",
    "Faucille",
    "Coriandre",
    "Artichaut",
    "Girofle",
    "Lavande",
    "Chamois",
    "Tabac",
    "Groseille",
    "Gesse",
    "Cerise",
    "Parc",
    "Menthe",
    "Cumin",
    "Haricot",
    "Orcan??te",
    "Pintade",
    "Sauge",
    "Ail",
    "Vesce",
    "Bl??",
    "Chal??mie",
    "??peautre",
    "Bouillon blanc",
    "Melon",
    "Ivraie",
    "B??lier",
    "Pr??le",
    "Armoise",
    "Carthame",
    "M??re",
    "Arrosoir",
    "Panic",
    "Salicorne",
    "Abricot",
    "Basilic",
    "Brebis",
    "Guimauve",
    "Lin",
    "Amande",
    "Gentiane",
    "??cluse",
    "Carline",
    "C??prier",
    "Lentille",
    "Aun??e",
    "Loutre",
    "Myrte",
    "Colza",
    "Lupin",
    "Coton",
    "Moulin",
    "Prune",
    "Millet",
    "Lycoperdon",
    "Escourgeon",
    "Saumon",
    "Tub??reuse",
    "Sucrion",
    "Apocyn",
    "R??glisse",
    "??chelle",
    "Past??que",
    "Fenouil",
    "??pine vinette",
    "Noix",
    "Truite",
    "Citron",
    "Card??re",
    "Nerprun",
    "Tagette",
    "Hotte",
    "??glantier",
    "Noisette",
    "Houblon",
    "Sorgho",
    "??crevisse",
    "Bigarade",
    "Verge d'or",
    "Ma??s",
    "Marron",
    "Panier",
];

