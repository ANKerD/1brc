use std::{env, fs::OpenOptions, io::Write};

use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};

fn measure(n: f32) -> f32 {
    let mut rng = thread_rng();
    let normal = Normal::new(n, 10.0).unwrap();
    let x = normal.sample(&mut rng);
    return (x * 10.0).round() / 10.0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        panic!("Wrong parameters provided")
    }

    let size: usize = args[1].trim().parse().unwrap();
    let filepath = args[2].trim();
    

    let stations =  [    
        ("Abha".to_string(), 18.0),
        ("Abidjan".to_string(), 26.0),
        ("Abéché".to_string(), 29.4),
        ("Accra".to_string(), 26.4),
        ("Addis Ababa".to_string(), 16.0),
        ("Adelaide".to_string(), 17.3),
        ("Aden".to_string(), 29.1),
        ("Ahvaz".to_string(), 25.4),
        ("Albuquerque".to_string(), 14.0),
        ("Alexandra".to_string(), 11.0),
        ("Alexandria".to_string(), 20.0),
        ("Algiers".to_string(), 18.2),
        ("Alice Springs".to_string(), 21.0),
        ("Almaty".to_string(), 10.0),
        ("Amsterdam".to_string(), 10.2),
        ("Anadyr".to_string(), -6.9),
        ("Anchorage".to_string(), 2.8),
        ("Andorra la Vella".to_string(), 9.8),
        ("Ankara".to_string(), 12.0),
        ("Antananarivo".to_string(), 17.9),
        ("Antsiranana".to_string(), 25.2),
        ("Arkhangelsk".to_string(), 1.3),
        ("Ashgabat".to_string(), 17.1),
        ("Asmara".to_string(), 15.6),
        ("Assab".to_string(), 30.5),
        ("Astana".to_string(), 3.5),
        ("Athens".to_string(), 19.2),
        ("Atlanta".to_string(), 17.0),
        ("Auckland".to_string(), 15.2),
        ("Austin".to_string(), 20.7),
        ("Baghdad".to_string(), 22.77),
        ("Baguio".to_string(), 19.5),
        ("Baku".to_string(), 15.1),
        ("Baltimore".to_string(), 13.1),
        ("Bamako".to_string(), 27.8),
        ("Bangkok".to_string(), 28.6),
        ("Bangui".to_string(), 26.0),
        ("Banjul".to_string(), 26.0),
        ("Barcelona".to_string(), 18.2),
        ("Bata".to_string(), 25.1),
        ("Batumi".to_string(), 14.0),
        ("Beijing".to_string(), 12.9),
        ("Beirut".to_string(), 20.9),
        ("Belgrade".to_string(), 12.5),
        ("Belize City".to_string(), 26.7),
        ("Benghazi".to_string(), 19.9),
        ("Bergen".to_string(), 7.7),
        ("Berlin".to_string(), 10.3),
        ("Bilbao".to_string(), 14.7),
        ("Birao".to_string(), 26.5),
        ("Bishkek".to_string(), 11.3),
        ("Bissau".to_string(), 27.0),
        ("Blantyre".to_string(), 22.2),
        ("Bloemfontein".to_string(), 15.6),
        ("Boise".to_string(), 11.4),
        ("Bordeaux".to_string(), 14.2),
        ("Bosaso".to_string(), 30.0),
        ("Boston".to_string(), 10.9),
        ("Bouaké".to_string(), 26.0),
        ("Bratislava".to_string(), 10.5),
        ("Brazzaville".to_string(), 25.0),
        ("Bridgetown".to_string(), 27.0),
        ("Brisbane".to_string(), 21.4),
        ("Brussels".to_string(), 10.5),
        ("Bucharest".to_string(), 10.8),
        ("Budapest".to_string(), 11.3),
        ("Bujumbura".to_string(), 23.8),
        ("Bulawayo".to_string(), 18.9),
        ("Burnie".to_string(), 13.1),
        ("Busan".to_string(), 15.0),
        ("Cabo San Lucas".to_string(), 23.9),
        ("Cairns".to_string(), 25.0),
        ("Cairo".to_string(), 21.4),
        ("Calgary".to_string(), 4.4),
        ("Canberra".to_string(), 13.1),
        ("Cape Town".to_string(), 16.2),
        ("Changsha".to_string(), 17.4),
        ("Charlotte".to_string(), 16.1),
        ("Chiang Mai".to_string(), 25.8),
        ("Chicago".to_string(), 9.8),
        ("Chihuahua".to_string(), 18.6),
        ("Chișinău".to_string(), 10.2),
        ("Chittagong".to_string(), 25.9),
        ("Chongqing".to_string(), 18.6),
        ("Christchurch".to_string(), 12.2),
        ("City of San Marino".to_string(), 11.8),
        ("Colombo".to_string(), 27.4),
        ("Columbus".to_string(), 11.7),
        ("Conakry".to_string(), 26.4),
        ("Copenhagen".to_string(), 9.1),
        ("Cotonou".to_string(), 27.2),
        ("Cracow".to_string(), 9.3),
        ("Da Lat".to_string(), 17.9),
        ("Da Nang".to_string(), 25.8),
        ("Dakar".to_string(), 24.0),
        ("Dallas".to_string(), 19.0),
        ("Damascus".to_string(), 17.0),
        ("Dampier".to_string(), 26.4),
        ("Dar es Salaam".to_string(), 25.8),
        ("Darwin".to_string(), 27.6),
        ("Denpasar".to_string(), 23.7),
        ("Denver".to_string(), 10.4),
        ("Detroit".to_string(), 10.0),
        ("Dhaka".to_string(), 25.9),
        ("Dikson".to_string(), -11.1),
        ("Dili".to_string(), 26.6),
        ("Djibouti".to_string(), 29.9),
        ("Dodoma".to_string(), 22.7),
        ("Dolisie".to_string(), 24.0),
        ("Douala".to_string(), 26.7),
        ("Dubai".to_string(), 26.9),
        ("Dublin".to_string(), 9.8),
        ("Dunedin".to_string(), 11.1),
        ("Durban".to_string(), 20.6),
        ("Dushanbe".to_string(), 14.7),
        ("Edinburgh".to_string(), 9.3),
        ("Edmonton".to_string(), 4.2),
        ("El Paso".to_string(), 18.1),
        ("Entebbe".to_string(), 21.0),
        ("Erbil".to_string(), 19.5),
        ("Erzurum".to_string(), 5.1),
        ("Fairbanks".to_string(), -2.3),
        ("Fianarantsoa".to_string(), 17.9),
        ("Flores,  Petén".to_string(), 26.4),
        ("Frankfurt".to_string(), 10.6),
        ("Fresno".to_string(), 17.9),
        ("Fukuoka".to_string(), 17.0),
        ("Gabès".to_string(), 19.5),
        ("Gaborone".to_string(), 21.0),
        ("Gagnoa".to_string(), 26.0),
        ("Gangtok".to_string(), 15.2),
        ("Garissa".to_string(), 29.3),
        ("Garoua".to_string(), 28.3),
        ("George Town".to_string(), 27.9),
        ("Ghanzi".to_string(), 21.4),
        ("Gjoa Haven".to_string(), -14.4),
        ("Guadalajara".to_string(), 20.9),
        ("Guangzhou".to_string(), 22.4),
        ("Guatemala City".to_string(), 20.4),
        ("Halifax".to_string(), 7.5),
        ("Hamburg".to_string(), 9.7),
        ("Hamilton".to_string(), 13.8),
        ("Hanga Roa".to_string(), 20.5),
        ("Hanoi".to_string(), 23.6),
        ("Harare".to_string(), 18.4),
        ("Harbin".to_string(), 5.0),
        ("Hargeisa".to_string(), 21.7),
        ("Hat Yai".to_string(), 27.0),
        ("Havana".to_string(), 25.2),
        ("Helsinki".to_string(), 5.9),
        ("Heraklion".to_string(), 18.9),
        ("Hiroshima".to_string(), 16.3),
        ("Ho Chi Minh City".to_string(), 27.4),
        ("Hobart".to_string(), 12.7),
        ("Hong Kong".to_string(), 23.3),
        ("Honiara".to_string(), 26.5),
        ("Honolulu".to_string(), 25.4),
        ("Houston".to_string(), 20.8),
        ("Ifrane".to_string(), 11.4),
        ("Indianapolis".to_string(), 11.8),
        ("Iqaluit".to_string(), -9.3),
        ("Irkutsk".to_string(), 1.0),
        ("Istanbul".to_string(), 13.9),
        ("İzmir".to_string(), 17.9),
        ("Jacksonville".to_string(), 20.3),
        ("Jakarta".to_string(), 26.7),
        ("Jayapura".to_string(), 27.0),
        ("Jerusalem".to_string(), 18.3),
        ("Johannesburg".to_string(), 15.5),
        ("Jos".to_string(), 22.8),
        ("Juba".to_string(), 27.8),
        ("Kabul".to_string(), 12.1),
        ("Kampala".to_string(), 20.0),
        ("Kandi".to_string(), 27.7),
        ("Kankan".to_string(), 26.5),
        ("Kano".to_string(), 26.4),
        ("Kansas City".to_string(), 12.5),
        ("Karachi".to_string(), 26.0),
        ("Karonga".to_string(), 24.4),
        ("Kathmandu".to_string(), 18.3),
        ("Khartoum".to_string(), 29.9),
        ("Kingston".to_string(), 27.4),
        ("Kinshasa".to_string(), 25.3),
        ("Kolkata".to_string(), 26.7),
        ("Kuala Lumpur".to_string(), 27.3),
        ("Kumasi".to_string(), 26.0),
        ("Kunming".to_string(), 15.7),
        ("Kuopio".to_string(), 3.4),
        ("Kuwait City".to_string(), 25.7),
        ("Kyiv".to_string(), 8.4),
        ("Kyoto".to_string(), 15.8),
        ("La Ceiba".to_string(), 26.2),
        ("La Paz".to_string(), 23.7),
        ("Lagos".to_string(), 26.8),
        ("Lahore".to_string(), 24.3),
        ("Lake Havasu City".to_string(), 23.7),
        ("Lake Tekapo".to_string(), 8.7),
        ("Las Palmas de Gran Canaria".to_string(), 21.2),
        ("Las Vegas".to_string(), 20.3),
        ("Launceston".to_string(), 13.1),
        ("Lhasa".to_string(), 7.6),
        ("Libreville".to_string(), 25.9),
        ("Lisbon".to_string(), 17.5),
        ("Livingstone".to_string(), 21.8),
        ("Ljubljana".to_string(), 10.9),
        ("Lodwar".to_string(), 29.3),
        ("Lomé".to_string(), 26.9),
        ("London".to_string(), 11.3),
        ("Los Angeles".to_string(), 18.6),
        ("Louisville".to_string(), 13.9),
        ("Luanda".to_string(), 25.8),
        ("Lubumbashi".to_string(), 20.8),
        ("Lusaka".to_string(), 19.9),
        ("Luxembourg City".to_string(), 9.3),
        ("Lviv".to_string(), 7.8),
        ("Lyon".to_string(), 12.5),
        ("Madrid".to_string(), 15.0),
        ("Mahajanga".to_string(), 26.3),
        ("Makassar".to_string(), 26.7),
        ("Makurdi".to_string(), 26.0),
        ("Malabo".to_string(), 26.3),
        ("Malé".to_string(), 28.0),
        ("Managua".to_string(), 27.3),
        ("Manama".to_string(), 26.5),
        ("Mandalay".to_string(), 28.0),
        ("Mango".to_string(), 28.1),
        ("Manila".to_string(), 28.4),
        ("Maputo".to_string(), 22.8),
        ("Marrakesh".to_string(), 19.6),
        ("Marseille".to_string(), 15.8),
        ("Maun".to_string(), 22.4),
        ("Medan".to_string(), 26.5),
        ("Mek'ele".to_string(), 22.7),
        ("Melbourne".to_string(), 15.1),
        ("Memphis".to_string(), 17.2),
        ("Mexicali".to_string(), 23.1),
        ("Mexico City".to_string(), 17.5),
        ("Miami".to_string(), 24.9),
        ("Milan".to_string(), 13.0),
        ("Milwaukee".to_string(), 8.9),
        ("Minneapolis".to_string(), 7.8),
        ("Minsk".to_string(), 6.7),
        ("Mogadishu".to_string(), 27.1),
        ("Mombasa".to_string(), 26.3),
        ("Monaco".to_string(), 16.4),
        ("Moncton".to_string(), 6.1),
        ("Monterrey".to_string(), 22.3),
        ("Montreal".to_string(), 6.8),
        ("Moscow".to_string(), 5.8),
        ("Mumbai".to_string(), 27.1),
        ("Murmansk".to_string(), 0.6),
        ("Muscat".to_string(), 28.0),
        ("Mzuzu".to_string(), 17.7),
        ("N'Djamena".to_string(), 28.3),
        ("Naha".to_string(), 23.1),
        ("Nairobi".to_string(), 17.8),
        ("Nakhon Ratchasima".to_string(), 27.3),
        ("Napier".to_string(), 14.6),
        ("Napoli".to_string(), 15.9),
        ("Nashville".to_string(), 15.4),
        ("Nassau".to_string(), 24.6),
        ("Ndola".to_string(), 20.3),
        ("New Delhi".to_string(), 25.0),
        ("New Orleans".to_string(), 20.7),
        ("New York City".to_string(), 12.9),
        ("Ngaoundéré".to_string(), 22.0),
        ("Niamey".to_string(), 29.3),
        ("Nicosia".to_string(), 19.7),
        ("Niigata".to_string(), 13.9),
        ("Nouadhibou".to_string(), 21.3),
        ("Nouakchott".to_string(), 25.7),
        ("Novosibirsk".to_string(), 1.7),
        ("Nuuk".to_string(), -1.4),
        ("Odesa".to_string(), 10.7),
        ("Odienné".to_string(), 26.0),
        ("Oklahoma City".to_string(), 15.9),
        ("Omaha".to_string(), 10.6),
        ("Oranjestad".to_string(), 28.1),
        ("Oslo".to_string(), 5.7),
        ("Ottawa".to_string(), 6.6),
        ("Ouagadougou".to_string(), 28.3),
        ("Ouahigouya".to_string(), 28.6),
        ("Ouarzazate".to_string(), 18.9),
        ("Oulu".to_string(), 2.7),
        ("Palembang".to_string(), 27.3),
        ("Palermo".to_string(), 18.5),
        ("Palm Springs".to_string(), 24.5),
        ("Palmerston North".to_string(), 13.2),
        ("Panama City".to_string(), 28.0),
        ("Parakou".to_string(), 26.8),
        ("Paris".to_string(), 12.3),
        ("Perth".to_string(), 18.7),
        ("Petropavlovsk-Kamchatsky".to_string(), 1.9),
        ("Philadelphia".to_string(), 13.2),
        ("Phnom Penh".to_string(), 28.3),
        ("Phoenix".to_string(), 23.9),
        ("Pittsburgh".to_string(), 10.8),
        ("Podgorica".to_string(), 15.3),
        ("Pointe-Noire".to_string(), 26.1),
        ("Pontianak".to_string(), 27.7),
        ("Port Moresby".to_string(), 26.9),
        ("Port Sudan".to_string(), 28.4),
        ("Port Vila".to_string(), 24.3),
        ("Port-Gentil".to_string(), 26.0),
        ("Portland (OR)".to_string(), 12.4),
        ("Porto".to_string(), 15.7),
        ("Prague".to_string(), 8.4),
        ("Praia".to_string(), 24.4),
        ("Pretoria".to_string(), 18.2),
        ("Pyongyang".to_string(), 10.8),
        ("Rabat".to_string(), 17.2),
        ("Rangpur".to_string(), 24.4),
        ("Reggane".to_string(), 28.3),
        ("Reykjavík".to_string(), 4.3),
        ("Riga".to_string(), 6.2),
        ("Riyadh".to_string(), 26.0),
        ("Rome".to_string(), 15.2),
        ("Roseau".to_string(), 26.2),
        ("Rostov-on-Don".to_string(), 9.9),
        ("Sacramento".to_string(), 16.3),
        ("Saint Petersburg".to_string(), 5.8),
        ("Saint-Pierre".to_string(), 5.7),
        ("Salt Lake City".to_string(), 11.6),
        ("San Antonio".to_string(), 20.8),
        ("San Diego".to_string(), 17.8),
        ("San Francisco".to_string(), 14.6),
        ("San Jose".to_string(), 16.4),
        ("San José".to_string(), 22.6),
        ("San Juan".to_string(), 27.2),
        ("San Salvador".to_string(), 23.1),
        ("Sana'a".to_string(), 20.0),
        ("Santo Domingo".to_string(), 25.9),
        ("Sapporo".to_string(), 8.9),
        ("Sarajevo".to_string(), 10.1),
        ("Saskatoon".to_string(), 3.3),
        ("Seattle".to_string(), 11.3),
        ("Ségou".to_string(), 28.0),
        ("Seoul".to_string(), 12.5),
        ("Seville".to_string(), 19.2),
        ("Shanghai".to_string(), 16.7),
        ("Singapore".to_string(), 27.0),
        ("Skopje".to_string(), 12.4),
        ("Sochi".to_string(), 14.2),
        ("Sofia".to_string(), 10.6),
        ("Sokoto".to_string(), 28.0),
        ("Split".to_string(), 16.1),
        ("St. John's".to_string(), 5.0),
        ("St. Louis".to_string(), 13.9),
        ("Stockholm".to_string(), 6.6),
        ("Surabaya".to_string(), 27.1),
        ("Suva".to_string(), 25.6),
        ("Suwałki".to_string(), 7.2),
        ("Sydney".to_string(), 17.7),
        ("Tabora".to_string(), 23.0),
        ("Tabriz".to_string(), 12.6),
        ("Taipei".to_string(), 23.0),
        ("Tallinn".to_string(), 6.4),
        ("Tamale".to_string(), 27.9),
        ("Tamanrasset".to_string(), 21.7),
        ("Tampa".to_string(), 22.9),
        ("Tashkent".to_string(), 14.8),
        ("Tauranga".to_string(), 14.8),
        ("Tbilisi".to_string(), 12.9),
        ("Tegucigalpa".to_string(), 21.7),
        ("Tehran".to_string(), 17.0),
        ("Tel Aviv".to_string(), 20.0),
        ("Thessaloniki".to_string(), 16.0),
        ("Thiès".to_string(), 24.0),
        ("Tijuana".to_string(), 17.8),
        ("Timbuktu".to_string(), 28.0),
        ("Tirana".to_string(), 15.2),
        ("Toamasina".to_string(), 23.4),
        ("Tokyo".to_string(), 15.4),
        ("Toliara".to_string(), 24.1),
        ("Toluca".to_string(), 12.4),
        ("Toronto".to_string(), 9.4),
        ("Tripoli".to_string(), 20.0),
        ("Tromsø".to_string(), 2.9),
        ("Tucson".to_string(), 20.9),
        ("Tunis".to_string(), 18.4),
        ("Ulaanbaatar".to_string(), -0.4),
        ("Upington".to_string(), 20.4),
        ("Ürümqi".to_string(), 7.4),
        ("Vaduz".to_string(), 10.1),
        ("Valencia".to_string(), 18.3),
        ("Valletta".to_string(), 18.8),
        ("Vancouver".to_string(), 10.4),
        ("Veracruz".to_string(), 25.4),
        ("Vienna".to_string(), 10.4),
        ("Vientiane".to_string(), 25.9),
        ("Villahermosa".to_string(), 27.1),
        ("Vilnius".to_string(), 6.0),
        ("Virginia Beach".to_string(), 15.8),
        ("Vladivostok".to_string(), 4.9),
        ("Warsaw".to_string(), 8.5),
        ("Washington, D.C.".to_string(), 14.6),
        ("Wau".to_string(), 27.8),
        ("Wellington".to_string(), 12.9),
        ("Whitehorse".to_string(), -0.1),
        ("Wichita".to_string(), 13.9),
        ("Willemstad".to_string(), 28.0),
        ("Winnipeg".to_string(), 3.0),
        ("Wrocław".to_string(), 9.6),
        ("Xi'an".to_string(), 14.1),
        ("Yakutsk".to_string(), -8.8),
        ("Yangon".to_string(), 27.5),
        ("Yaoundé".to_string(), 23.8),
        ("Yellowknife".to_string(), -4.3),
        ("Yerevan".to_string(), 12.4),
        ("Yinchuan".to_string(), 9.0),
        ("Zagreb".to_string(), 10.7),
        ("Zanzibar City".to_string(), 26.0),
        ("Zürich".to_string(), 9.3)
    ];

    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(filepath)
        .unwrap();

    let mut rng = rand::thread_rng();
    for _i in 0..size {
        let index = rng.gen_range(0..stations.len());
        let data = format!("{};{}\n", stations[index].0, measure(stations[index].1));
        f.write_all(data.as_bytes()).expect("Unable to write");
    }
}