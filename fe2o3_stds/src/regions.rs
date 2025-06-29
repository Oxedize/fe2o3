use oxedyne_fe2o3_core::{
    prelude::*,
    new_enum,
    rand::Rand,
};
use strum::Display;

/// Adapted from https://textlists.info/geography/countries-of-the-world/ 
/// See also https://en.wikipedia.org/wiki/ISO_3166-1
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[repr(usize)]
pub enum Country {
    Afghanistan,
    Albania,
    Algeria,
    Andorra,
    Angola,
    AntiguaAndBarbuda,
    Argentina,
    Armenia,
    Australia,
    Austria,
    Azerbaijan,
    TheBahamas,
    Bahrain,
    Bangladesh,
    Barbados,
    Belarus,
    Belgium,
    Belize,
    Benin,
    Bhutan,
    Bolivia,
    BosniaAndHerzegovina,
    Botswana,
    Brazil,
    Brunei,
    Bulgaria,
    BurkinaFaso,
    Burundi,
    Cambodia,
    Cameroon,
    Canada,
    CapeVerde,
    CentralAfricanRepublic,
    Chad,
    Chile,
    China,
    Colombia,
    Comoros,
    CostaRica,
    CotedIvoire,
    Croatia,
    Cuba,
    Cyprus,
    CzechRepublic,
    DemocraticRepublicOfTheCongo,
    DemocraticRepublicOfTimorLeste,
    Denmark,
    Djibouti,
    Dominica,
    DominicanRepublic,
    Ecuador,
    Egypt,
    ElSalvador,
    EquatorialGuinea,
    Eritrea,
    Estonia,
    Ethiopia,
    FederatedStatesOfMicronesia,
    Fiji,
    Finland,
    France,
    Gabon,
    TheGambia,
    Georgia,
    Germany,
    Ghana,
    Greece,
    Grenada,
    Guatemala,
    Guinea,
    GuineaBissau,
    Guyana,
    Haiti,
    Honduras,
    Hungary,
    Iceland,
    India,
    Indonesia,
    Iran,
    Iraq,
    Ireland,
    Israel,
    Italy,
    Jamaica,
    Japan,
    Jordan,
    Kazakhstan,
    Kenya,
    Kiribati,
    Kosovo,
    Kuwait,
    Kyrgyzstan,
    Laos,
    Latvia,
    Lebanon,
    Lesotho,
    Liberia,
    Libya,
    Liechtenstein,
    Lithuania,
    Luxembourg,
    Macedonia,
    Madagascar,
    Malawi,
    Malaysia,
    Maldives,
    Mali,
    Malta,
    MarshallIslands,
    Mauritania,
    Mauritius,
    Mexico,
    Moldova,
    Monaco,
    Mongolia,
    Montenegro,
    Morocco,
    Mozambique,
    Myanmar,
    Namibia,
    Nauru,
    Nepal,
    Netherlands,
    NewZealand,
    Nicaragua,
    Niger,
    Nigeria,
    NorthKorea,
    Norway,
    Oman,
    Pakistan,
    Palau,
    Panama,
    PapuaNewGuinea,
    Paraguay,
    Peru,
    Philippines,
    Poland,
    Portugal,
    Qatar,
    RepublicOfTheCongo,
    Romania,
    Russia,
    Rwanda,
    SaintKittsAndNevis,
    SaintLucia,
    SaintVincentAndTheGrenadines,
    Samoa,
    SanMarino,
    SaoTomeAndPrincipe,
    SaudiArabia,
    Senegal,
    Serbia,
    Seychelles,
    SierraLeone,
    Singapore,
    Slovakia,
    Slovenia,
    SolomonIslands,
    Somalia,
    SouthAfrica,
    SouthKorea,
    SouthSudan,
    Spain,
    SriLanka,
    Sudan,
    Suriname,
    Swaziland,
    Sweden,
    Switzerland,
    Syria,
    Taiwan,
    Tajikistan,
    Tanzania,
    Thailand,
    Togo,
    Tonga,
    TrinidadAndTobago,
    Tunisia,
    Turkey,
    Turkmenistan,
    Tuvalu,
    Uganda,
    Ukraine,
    UnitedArabEmirates,
    UnitedKingdom,
    UnitedStatesOfAmerica,
    Uruguay,
    Uzbekistan,
    Vanuatu,
    VaticanCity,
    Venezuela,
    Vietnam,
    Yemen,
    Zambia,
    Zimbabwe,
}

new_enum!(Country;
    Afghanistan,
    Albania,
    Algeria,
    Andorra,
    Angola,
    AntiguaAndBarbuda,
    Argentina,
    Armenia,
    Australia,
    Austria,
    Azerbaijan,
    TheBahamas,
    Bahrain,
    Bangladesh,
    Barbados,
    Belarus,
    Belgium,
    Belize,
    Benin,
    Bhutan,
    Bolivia,
    BosniaAndHerzegovina,
    Botswana,
    Brazil,
    Brunei,
    Bulgaria,
    BurkinaFaso,
    Burundi,
    Cambodia,
    Cameroon,
    Canada,
    CapeVerde,
    CentralAfricanRepublic,
    Chad,
    Chile,
    China,
    Colombia,
    Comoros,
    CostaRica,
    CotedIvoire,
    Croatia,
    Cuba,
    Cyprus,
    CzechRepublic,
    DemocraticRepublicOfTheCongo,
    DemocraticRepublicOfTimorLeste,
    Denmark,
    Djibouti,
    Dominica,
    DominicanRepublic,
    Ecuador,
    Egypt,
    ElSalvador,
    EquatorialGuinea,
    Eritrea,
    Estonia,
    Ethiopia,
    FederatedStatesOfMicronesia,
    Fiji,
    Finland,
    France,
    Gabon,
    TheGambia,
    Georgia,
    Germany,
    Ghana,
    Greece,
    Grenada,
    Guatemala,
    Guinea,
    GuineaBissau,
    Guyana,
    Haiti,
    Honduras,
    Hungary,
    Iceland,
    India,
    Indonesia,
    Iran,
    Iraq,
    Ireland,
    Israel,
    Italy,
    Jamaica,
    Japan,
    Jordan,
    Kazakhstan,
    Kenya,
    Kiribati,
    Kosovo,
    Kuwait,
    Kyrgyzstan,
    Laos,
    Latvia,
    Lebanon,
    Lesotho,
    Liberia,
    Libya,
    Liechtenstein,
    Lithuania,
    Luxembourg,
    Macedonia,
    Madagascar,
    Malawi,
    Malaysia,
    Maldives,
    Mali,
    Malta,
    MarshallIslands,
    Mauritania,
    Mauritius,
    Mexico,
    Moldova,
    Monaco,
    Mongolia,
    Montenegro,
    Morocco,
    Mozambique,
    Myanmar,
    Namibia,
    Nauru,
    Nepal,
    Netherlands,
    NewZealand,
    Nicaragua,
    Niger,
    Nigeria,
    NorthKorea,
    Norway,
    Oman,
    Pakistan,
    Palau,
    Panama,
    PapuaNewGuinea,
    Paraguay,
    Peru,
    Philippines,
    Poland,
    Portugal,
    Qatar,
    RepublicOfTheCongo,
    Romania,
    Russia,
    Rwanda,
    SaintKittsAndNevis,
    SaintLucia,
    SaintVincentAndTheGrenadines,
    Samoa,
    SanMarino,
    SaoTomeAndPrincipe,
    SaudiArabia,
    Senegal,
    Serbia,
    Seychelles,
    SierraLeone,
    Singapore,
    Slovakia,
    Slovenia,
    SolomonIslands,
    Somalia,
    SouthAfrica,
    SouthKorea,
    SouthSudan,
    Spain,
    SriLanka,
    Sudan,
    Suriname,
    Swaziland,
    Sweden,
    Switzerland,
    Syria,
    Taiwan,
    Tajikistan,
    Tanzania,
    Thailand,
    Togo,
    Tonga,
    TrinidadAndTobago,
    Tunisia,
    Turkey,
    Turkmenistan,
    Tuvalu,
    Uganda,
    Ukraine,
    UnitedArabEmirates,
    UnitedKingdom,
    UnitedStatesOfAmerica,
    Uruguay,
    Uzbekistan,
    Vanuatu,
    VaticanCity,
    Venezuela,
    Vietnam,
    Yemen,
    Zambia,
    Zimbabwe,
);

/// Population data reference year and sources.
///
/// All population data in this type is based on:
/// - Reference Year: 2023
/// - Primary Sources:
///   - UN World Population Prospects 2022 (country totals)
///   - World Bank Urban Population % (2022 data)
///   - UN World Urbanization Prospects 2018 (city populations)
///   - National census data where available (2020-2023)
///
/// Notes:
/// - City populations represent metropolitan/urban agglomeration areas
/// - Rural populations are calculated as (total - sum of urban)
/// - Areas are approximate and may not sum exactly to country total
/// - Some small countries use simplified single-region models
///
/// IMPORTANT LIMITATIONS:
/// - Generated coordinates use circular distributions around population centres
/// - Does NOT respect actual borders, coastlines, or water bodies
/// - Island nations (e.g., Philippines, Indonesia) will generate many ocean points
/// - Coastal cities often generate 30-50% of points in water
/// - Border regions may generate points in neighbouring countries
/// - For accurate land-only points, post-process with actual GIS boundary data
impl Country {
    /// Generates a random location within a country, weighted by population density.
    /// 
    /// This function returns a latitude and longitude pair that reflects actual
    /// population distribution, including both urban centres and rural areas.
    /// 
    /// # Arguments
    /// 
    /// * `country` - The country to generate a location for.
    /// 
    /// # Returns
    /// 
    /// A `LocationPopulationRegion` containing latitude, longitude in decimal degrees and the region name.
    pub fn random_population_location(&self) -> LocationPopulationRegion {
        let country_data = self.get_country_population_data();
        
        // Create weighted distribution based on population.
        let weights: Vec<f64> = country_data.regions.iter().map(|r| r.population).collect();
        let total_weight: f64 = weights.iter().sum();
        
        // Select a region based on population weights.
        let mut selection_value = Rand::value::<f64>() * total_weight;
        let mut selected_region = country_data.regions[0].clone();
        
        for (i, region) in country_data.regions.iter().enumerate() {
            selection_value -= weights[i];
            if selection_value <= 0.0 {
                selected_region = region.clone();
                break;
            }
        }
        
        // Generate a random point within the selected region.
        let (latitude, longitude) = selected_region.random_point_in_region();

        LocationPopulationRegion {
            latitude,
            longitude,
            region: selected_region,
        }
    }

    /// Returns population data for a given country.
    /// 
    /// Each country is divided into non-overlapping regions (both urban and rural)
    /// with accurate population and area data.
    pub fn get_country_population_data(&self) -> CountryPopulationData {
        match self {
            Self::Afghanistan => {
                // Afghanistan: 40.1M population, 652,860 km² area, ~26% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(34.5553, 69.2075, 4_600_000.0, 275.0, AreaType::Urban, "Kabul"),
                    PopulationRegion::new(31.6100, 65.7100, 614_000.0, 54.0, AreaType::Urban, "Kandahar"),
                    PopulationRegion::new(36.7050, 67.1100, 548_000.0, 50.0, AreaType::Urban, "Mazar-i-Sharif"),
                    PopulationRegion::new(34.3480, 62.2040, 506_000.0, 45.0, AreaType::Urban, "Herat"),
                    PopulationRegion::new(34.5000, 69.1800, 30_000_000.0, 651_500.0, AreaType::Rural, "Rural Afghanistan"),
                ], 652_860.0)
            },
            
            Self::Albania => {
                // Albania: 2.8M population, 28,748 km² area, ~63% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(41.3275, 19.8187, 610_000.0, 42.0, AreaType::Urban, "Tirana"),
                    PopulationRegion::new(40.4587, 19.4901, 140_000.0, 28.0, AreaType::Urban, "Durrës"),
                    PopulationRegion::new(40.7264, 19.5687, 95_000.0, 20.0, AreaType::Urban, "Elbasan"),
                    PopulationRegion::new(41.0, 20.0, 1_100_000.0, 28_600.0, AreaType::Rural, "Rural Albania"),
                ], 28_748.0)
            },
            
            Self::Algeria => {
                // Algeria: 44.7M population, 2,381,741 km² area, ~74% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(36.7538, 3.0588, 2_988_000.0, 363.0, AreaType::Urban, "Algiers"),
                    PopulationRegion::new(35.6969, -0.6331, 541_000.0, 64.0, AreaType::Urban, "Oran"),
                    PopulationRegion::new(36.3650, 6.6147, 507_000.0, 80.0, AreaType::Urban, "Constantine"),
                    PopulationRegion::new(36.1900, 5.4100, 415_000.0, 127.0, AreaType::Urban, "Sétif"),
                    PopulationRegion::new(35.4000, 8.1200, 380_000.0, 40.0, AreaType::Urban, "Biskra"),
                    PopulationRegion::new(35.4028, 0.1320, 335_000.0, 85.0, AreaType::Urban, "Tiaret"),
                    PopulationRegion::new(36.7525, 5.0556, 320_000.0, 75.0, AreaType::Urban, "Béjaïa"),
                    PopulationRegion::new(36.9050, 7.7633, 295_000.0, 68.0, AreaType::Urban, "Annaba"),
                    PopulationRegion::new(35.3714, 1.3225, 280_000.0, 62.0, AreaType::Urban, "Tlemcen"),
                    PopulationRegion::new(28.0, 3.0, 10_600_000.0, 2_380_000.0, AreaType::Rural, "Rural Algeria"),
                ], 2_381_741.0)
            },
            
            Self::Andorra => {
                // Andorra: 79k population, 468 km² area, ~88% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(42.5063, 1.5218, 23_000.0, 12.0, AreaType::Urban, "Andorra la Vella"),
                    PopulationRegion::new(42.5, 1.5, 56_000.0, 456.0, AreaType::Rural, "Rural Andorra"),
                ], 468.0)
            },
            
            Self::Angola => {
                // Angola: 34.5M population, 1,246,700 km² area, ~67% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-8.8390, 13.2894, 8_330_000.0, 116.0, AreaType::Urban, "Luanda"),
                    PopulationRegion::new(-12.5700, 13.4100, 810_000.0, 40.0, AreaType::Urban, "Lubango"),
                    PopulationRegion::new(-11.2000, 13.5400, 778_000.0, 35.0, AreaType::Urban, "Huambo"),
                    PopulationRegion::new(-5.9200, 12.4100, 550_000.0, 30.0, AreaType::Urban, "Cabinda"),
                    PopulationRegion::new(-11.0, 16.0, 11_400_000.0, 1_245_000.0, AreaType::Rural, "Rural Angola"),
                ], 1_246_700.0)
            },
            
            Self::AntiguaAndBarbuda => {
                // Antigua and Barbuda: 98k population, 442 km² area, ~24% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(17.1175, -61.8456, 24_000.0, 10.0, AreaType::Urban, "St. John's"),
                    PopulationRegion::new(17.1333, -61.7833, 11_000.0, 5.0, AreaType::Urban, "All Saints"),
                    PopulationRegion::new(17.0667, -61.8667, 5_000.0, 3.0, AreaType::Urban, "Liberta"),
                    PopulationRegion::new(17.1, -61.8, 36_000.0, 280.0, AreaType::Rural, "Rural Antigua"),
                    PopulationRegion::new(17.6333, -62.0167, 22_000.0, 144.0, AreaType::Rural, "Rural Barbuda"),
                ], 442.0)
            },
            
            Self::Argentina => {
                // Argentina: 45.8M population, 2,780,400 km² area, ~92% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-34.6037, -58.3816, 15_370_000.0, 203.0, AreaType::Urban, "Buenos Aires Metro"),
                    PopulationRegion::new(-31.4201, -64.1888, 1_577_000.0, 576.0, AreaType::Urban, "Córdoba"),
                    PopulationRegion::new(-32.8908, -68.8272, 1_200_000.0, 57.0, AreaType::Urban, "Mendoza"),
                    PopulationRegion::new(-26.8083, -65.2176, 1_014_000.0, 91.0, AreaType::Urban, "Tucumán"),
                    PopulationRegion::new(-34.9205, -57.9536, 937_000.0, 25.0, AreaType::Urban, "La Plata"),
                    PopulationRegion::new(-24.7859, -65.4117, 792_000.0, 60.0, AreaType::Urban, "Salta"),
                    PopulationRegion::new(-32.9468, -60.6393, 743_000.0, 117.0, AreaType::Urban, "Rosario"),
                    PopulationRegion::new(-34.5711, -58.4232, 715_000.0, 52.0, AreaType::Urban, "San Justo"),
                    PopulationRegion::new(-27.7834, -64.2642, 554_000.0, 44.0, AreaType::Urban, "Santiago del Estero"),
                    PopulationRegion::new(-38.0, -63.0, 1_750_000.0, 2_778_000.0, AreaType::Rural, "Rural Argentina"),
                ], 2_780_400.0)
            },
            
            Self::Armenia => {
                // Armenia: 3.0M population, 29,743 km² area, ~63% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(40.1792, 44.4991, 1_090_000.0, 227.0, AreaType::Urban, "Yerevan"),
                    PopulationRegion::new(40.7899, 43.8473, 123_000.0, 25.0, AreaType::Urban, "Gyumri"),
                    PopulationRegion::new(40.1833, 44.5167, 74_000.0, 18.0, AreaType::Urban, "Vanadzor"),
                    PopulationRegion::new(40.5, 44.5, 1_100_000.0, 29_400.0, AreaType::Rural, "Rural Armenia"),
                ], 29_743.0)
            },
            
            Self::Australia => {
                // Australia: 25.9M population, 7,692,024 km² area, ~86% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-33.8688, 151.2093, 5_367_000.0, 12_368.0, AreaType::Urban, "Sydney Metro"),
                    PopulationRegion::new(-37.8136, 144.9631, 5_078_000.0, 9_992.0, AreaType::Urban, "Melbourne Metro"),
                    PopulationRegion::new(-27.4698, 153.0251, 2_568_000.0, 15_842.0, AreaType::Urban, "Brisbane Metro"),
                    PopulationRegion::new(-31.9505, 115.8605, 2_118_000.0, 6_418.0, AreaType::Urban, "Perth Metro"),
                    PopulationRegion::new(-34.9285, 138.6007, 1_376_000.0, 3_258.0, AreaType::Urban, "Adelaide Metro"),
                    PopulationRegion::new(-28.0167, 153.4000, 693_000.0, 1_353.0, AreaType::Urban, "Gold Coast-Tweed"),
                    PopulationRegion::new(-32.9283, 151.7817, 497_000.0, 2_334.0, AreaType::Urban, "Newcastle-Maitland"),
                    PopulationRegion::new(-35.2809, 149.1300, 462_000.0, 814.0, AreaType::Urban, "Canberra"),
                    PopulationRegion::new(-30.0, 140.0, 3_600_000.0, 7_640_000.0, AreaType::Rural, "Rural Australia"),
                ], 7_692_024.0)
            },
            
            Self::Austria => {
                // Austria: 9.0M population, 83,879 km² area, ~59% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(48.2082, 16.3738, 2_002_000.0, 414.0, AreaType::Urban, "Vienna"),
                    PopulationRegion::new(47.0707, 15.4395, 295_000.0, 128.0, AreaType::Urban, "Graz"),
                    PopulationRegion::new(47.2692, 11.4041, 133_000.0, 105.0, AreaType::Urban, "Innsbruck"),
                    PopulationRegion::new(47.8095, 13.0550, 157_000.0, 66.0, AreaType::Urban, "Salzburg"),
                    PopulationRegion::new(48.3069, 14.2858, 210_000.0, 96.0, AreaType::Urban, "Linz"),
                    PopulationRegion::new(47.5, 13.5, 3_700_000.0, 83_000.0, AreaType::Rural, "Rural Austria"),
                ], 83_879.0)
            },
            
            Self::Azerbaijan => {
                // Azerbaijan: 10.2M population, 86,600 km² area, ~57% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(40.4093, 49.8671, 2_374_000.0, 2_140.0, AreaType::Urban, "Baku"),
                    PopulationRegion::new(40.6828, 46.3606, 337_000.0, 110.0, AreaType::Urban, "Ganja"),
                    PopulationRegion::new(40.5893, 48.8315, 218_000.0, 95.0, AreaType::Urban, "Sumqayit"),
                    PopulationRegion::new(40.5, 48.0, 4_400_000.0, 84_000.0, AreaType::Rural, "Rural Azerbaijan"),
                ], 86_600.0)
            },
            
            Self::TheBahamas => {
                // The Bahamas: 397k population, 13,943 km² area, ~83% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(25.0480, -77.3554, 280_000.0, 207.0, AreaType::Urban, "Nassau"),
                    PopulationRegion::new(26.5333, -78.7000, 19_000.0, 23.0, AreaType::Urban, "Freeport"),
                    PopulationRegion::new(23.1085, -75.7619, 11_000.0, 15.0, AreaType::Urban, "George Town"),
                    PopulationRegion::new(24.9000, -76.1833, 9_000.0, 12.0, AreaType::Urban, "Cooper's Town"),
                    PopulationRegion::new(26.6833, -77.4000, 7_000.0, 10.0, AreaType::Urban, "Marsh Harbour"),
                    PopulationRegion::new(25.0, -77.5, 41_000.0, 13_676.0, AreaType::Rural, "Rural Bahamas"),
                ], 13_943.0)
            },
            
            Self::Bahrain => {
                // Bahrain: 1.7M population, 785 km² area, ~89% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(26.2285, 50.5860, 635_000.0, 30.0, AreaType::Urban, "Manama"),
                    PopulationRegion::new(26.1418, 50.5579, 261_000.0, 15.0, AreaType::Urban, "Riffa"),
                    PopulationRegion::new(26.2361, 50.5481, 216_000.0, 12.0, AreaType::Urban, "Muharraq"),
                    PopulationRegion::new(26.1, 50.6, 188_000.0, 728.0, AreaType::Rural, "Rural Bahrain"),
                ], 785.0)
            },
            
            Self::Bangladesh => {
                // Bangladesh: 169.4M population, 147,630 km² area, ~39% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(23.8103, 90.4125, 21_741_000.0, 306.0, AreaType::Urban, "Dhaka"),
                    PopulationRegion::new(22.3569, 91.7832, 5_253_000.0, 168.0, AreaType::Urban, "Chittagong"),
                    PopulationRegion::new(24.3636, 88.6241, 2_280_000.0, 65.0, AreaType::Urban, "Khulna"),
                    PopulationRegion::new(24.8465, 89.3773, 907_000.0, 25.0, AreaType::Urban, "Rajshahi"),
                    PopulationRegion::new(23.4607, 91.1809, 877_000.0, 20.0, AreaType::Urban, "Sylhet"),
                    PopulationRegion::new(25.7439, 89.2752, 774_000.0, 18.0, AreaType::Urban, "Rangpur"),
                    PopulationRegion::new(22.8456, 89.5403, 654_000.0, 15.0, AreaType::Urban, "Barisal"),
                    PopulationRegion::new(24.7471, 90.4203, 532_000.0, 12.0, AreaType::Urban, "Mymensingh"),
                    PopulationRegion::new(22.7010, 90.3535, 466_000.0, 10.0, AreaType::Urban, "Jessore"),
                    PopulationRegion::new(24.0, 90.0, 99_900_000.0, 147_000.0, AreaType::Rural, "Rural Bangladesh"),
                ], 147_630.0)
            },
            
            Self::Barbados => {
                // Barbados: 288k population, 439 km² area, ~31% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(13.0969, -59.6145, 90_000.0, 39.0, AreaType::Urban, "Bridgetown"),
                    PopulationRegion::new(13.1, -59.6, 198_000.0, 400.0, AreaType::Rural, "Rural Barbados"),
                ], 439.0)
            },
            
            Self::Belarus => {
                // Belarus: 9.3M population, 207,600 km² area, ~79% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(53.9006, 27.5590, 2_039_000.0, 348.0, AreaType::Urban, "Minsk"),
                    PopulationRegion::new(52.0975, 23.6877, 368_000.0, 67.0, AreaType::Urban, "Brest"),
                    PopulationRegion::new(52.4345, 30.9754, 359_000.0, 90.0, AreaType::Urban, "Gomel"),
                    PopulationRegion::new(55.1904, 30.2049, 270_000.0, 40.0, AreaType::Urban, "Vitebsk"),
                    PopulationRegion::new(53.6693, 23.8131, 231_000.0, 65.0, AreaType::Urban, "Grodno"),
                    PopulationRegion::new(53.5, 28.0, 1_950_000.0, 207_000.0, AreaType::Rural, "Rural Belarus"),
                ], 207_600.0)
            },
            
            Self::Belgium => {
                // Belgium: 11.6M population, 30,528 km² area, ~98% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(50.8503, 4.3517, 2_110_000.0, 161.0, AreaType::Urban, "Brussels Metro"),
                    PopulationRegion::new(51.2194, 4.4025, 1_050_000.0, 204.0, AreaType::Urban, "Antwerp"),
                    PopulationRegion::new(51.0543, 3.7174, 600_000.0, 156.0, AreaType::Urban, "Ghent"),
                    PopulationRegion::new(50.6292, 5.5797, 750_000.0, 862.0, AreaType::Urban, "Liège"),
                    PopulationRegion::new(50.4669, 4.8647, 500_000.0, 194.0, AreaType::Urban, "Charleroi"),
                    PopulationRegion::new(50.5, 4.5, 240_000.0, 29_000.0, AreaType::Rural, "Rural Belgium"),
                ], 30_528.0)
            },
            
            Self::Belize => {
                // Belize: 412k population, 22,966 km² area, ~46% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(17.4995, -88.1884, 75_000.0, 35.0, AreaType::Urban, "Belize City"),
                    PopulationRegion::new(17.2514, -88.7590, 20_000.0, 12.0, AreaType::Urban, "Belmopan"),
                    PopulationRegion::new(17.5, -88.5, 222_000.0, 22_900.0, AreaType::Rural, "Rural Belize"),
                ], 22_966.0)
            },
            
            Self::Benin => {
                // Benin: 12.5M population, 114,763 km² area, ~48% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(6.3654, 2.4183, 761_000.0, 79.0, AreaType::Urban, "Cotonou"),
                    PopulationRegion::new(6.4968, 2.6288, 492_000.0, 52.0, AreaType::Urban, "Porto-Novo"),
                    PopulationRegion::new(9.3419, 2.6286, 287_000.0, 40.0, AreaType::Urban, "Parakou"),
                    PopulationRegion::new(7.0, 2.0, 6_500_000.0, 114_600.0, AreaType::Rural, "Rural Benin"),
                ], 114_763.0)
            },
            
            Self::Bhutan => {
                // Bhutan: 777k population, 38,394 km² area, ~42% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(27.4728, 89.6393, 115_000.0, 26.0, AreaType::Urban, "Thimphu"),
                    PopulationRegion::new(27.2515, 91.5264, 19_000.0, 18.0, AreaType::Urban, "Samdrup Jongkhar"),
                    PopulationRegion::new(27.5, 90.0, 450_000.0, 38_350.0, AreaType::Rural, "Rural Bhutan"),
                ], 38_394.0)
            },
            
            Self::Bolivia => {
                // Bolivia: 11.8M population, 1,098,581 km² area, ~70% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-17.8146, -63.1561, 1_916_000.0, 469.0, AreaType::Urban, "Santa Cruz"),
                    PopulationRegion::new(-16.4897, -68.1193, 1_835_000.0, 133.0, AreaType::Urban, "La Paz"),
                    PopulationRegion::new(-17.3935, -66.1570, 1_385_000.0, 391.0, AreaType::Urban, "Cochabamba"),
                    PopulationRegion::new(-19.5836, -65.7531, 291_000.0, 37.0, AreaType::Urban, "Sucre"),
                    PopulationRegion::new(-17.0, -65.0, 3_500_000.0, 1_097_000.0, AreaType::Rural, "Rural Bolivia"),
                ], 1_098_581.0)
            },
            
            Self::BosniaAndHerzegovina => {
                // Bosnia and Herzegovina: 3.3M population, 51,197 km² area, ~49% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(43.8476, 18.3564, 420_000.0, 142.0, AreaType::Urban, "Sarajevo"),
                    PopulationRegion::new(44.7722, 17.1911, 185_000.0, 115.0, AreaType::Urban, "Banja Luka"),
                    PopulationRegion::new(43.3438, 17.8078, 115_000.0, 30.0, AreaType::Urban, "Mostar"),
                    PopulationRegion::new(44.5377, 18.6709, 83_000.0, 25.0, AreaType::Urban, "Tuzla"),
                    PopulationRegion::new(44.0, 18.0, 1_680_000.0, 50_900.0, AreaType::Rural, "Rural Bosnia"),
                ], 51_197.0)
            },
            
            Self::Botswana => {
                // Botswana: 2.4M population, 581,730 km² area, ~71% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-24.6537, 25.9065, 288_000.0, 169.0, AreaType::Urban, "Gaborone"),
                    PopulationRegion::new(-21.1661, 27.5146, 101_000.0, 79.0, AreaType::Urban, "Francistown"),
                    PopulationRegion::new(-20.4674, 23.1681, 91_000.0, 31.0, AreaType::Urban, "Maun"),
                    PopulationRegion::new(-22.0, 24.0, 690_000.0, 581_400.0, AreaType::Rural, "Rural Botswana"),
                ], 581_730.0)
            },
            
            Self::Brazil => {
                // Brazil: 214.3M population, 8,515,767 km² area, ~87% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-23.5505, -46.6333, 22_430_000.0, 7_947.0, AreaType::Urban, "São Paulo Metro"),
                    PopulationRegion::new(-22.9068, -43.1729, 13_074_000.0, 1_200.0, AreaType::Urban, "Rio de Janeiro Metro"),
                    PopulationRegion::new(-19.9167, -43.9345, 6_006_000.0, 331.0, AreaType::Urban, "Belo Horizonte Metro"),
                    PopulationRegion::new(-30.0346, -51.2177, 4_285_000.0, 497.0, AreaType::Urban, "Porto Alegre Metro"),
                    PopulationRegion::new(-15.7975, -47.8919, 4_728_000.0, 5_802.0, AreaType::Urban, "Brasília Metro"),
                    PopulationRegion::new(-8.0476, -34.8770, 4_175_000.0, 218.0, AreaType::Urban, "Recife Metro"),
                    PopulationRegion::new(-3.7172, -38.5433, 4_074_000.0, 314.0, AreaType::Urban, "Fortaleza Metro"),
                    PopulationRegion::new(-12.9714, -38.5014, 3_984_000.0, 693.0, AreaType::Urban, "Salvador Metro"),
                    PopulationRegion::new(-25.4244, -49.2654, 3_733_000.0, 435.0, AreaType::Urban, "Curitiba Metro"),
                    PopulationRegion::new(-23.9609, -46.3332, 3_572_000.0, 2_704.0, AreaType::Urban, "Campinas Metro"),
                    PopulationRegion::new(-3.1019, -60.0250, 2_676_000.0, 377.0, AreaType::Urban, "Manaus"),
                    PopulationRegion::new(-1.4558, -48.4902, 2_491_000.0, 346.0, AreaType::Urban, "Belém"),
                    PopulationRegion::new(-10.9472, -37.0731, 2_338_000.0, 163.0, AreaType::Urban, "Aracaju"),
                    PopulationRegion::new(-15.0, -55.0, 24_500_000.0, 8_500_000.0, AreaType::Rural, "Rural Brazil"),
                ], 8_515_767.0)
            },
            
            Self::Brunei => {
                // Brunei: 441k population, 5,765 km² area, ~78% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(4.9031, 114.9398, 241_000.0, 100.0, AreaType::Urban, "Bandar Seri Begawan"),
                    PopulationRegion::new(4.5828, 114.1595, 31_000.0, 12.0, AreaType::Urban, "Kuala Belait"),
                    PopulationRegion::new(4.6, 114.5, 97_000.0, 5_650.0, AreaType::Rural, "Rural Brunei"),
                ], 5_765.0)
            },
            
            Self::Bulgaria => {
                // Bulgaria: 6.9M population, 110,879 km² area, ~76% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(42.6977, 23.3219, 1_330_000.0, 492.0, AreaType::Urban, "Sofia"),
                    PopulationRegion::new(42.1354, 24.7453, 367_000.0, 102.0, AreaType::Urban, "Plovdiv"),
                    PopulationRegion::new(43.2141, 27.9147, 336_000.0, 205.0, AreaType::Urban, "Varna"),
                    PopulationRegion::new(43.0757, 25.6172, 185_000.0, 82.0, AreaType::Urban, "Ruse"),
                    PopulationRegion::new(42.5, 25.0, 1_660_000.0, 110_000.0, AreaType::Rural, "Rural Bulgaria"),
                ], 110_879.0)
            },
            
            Self::BurkinaFaso => {
                // Burkina Faso: 21.5M population, 274,222 km² area, ~31% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(12.3681, -1.5275, 2_780_000.0, 219.0, AreaType::Urban, "Ouagadougou"),
                    PopulationRegion::new(11.1771, -4.2964, 904_000.0, 136.0, AreaType::Urban, "Bobo-Dioulasso"),
                    PopulationRegion::new(12.0823, -1.6715, 190_000.0, 40.0, AreaType::Urban, "Koudougou"),
                    PopulationRegion::new(12.5, -2.0, 14_800_000.0, 273_800.0, AreaType::Rural, "Rural Burkina Faso"),
                ], 274_222.0)
            },
            
            Self::Burundi => {
                // Burundi: 12.3M population, 27,834 km² area, ~14% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-3.3822, 29.3644, 1_100_000.0, 87.0, AreaType::Urban, "Bujumbura"),
                    PopulationRegion::new(-3.3731, 29.9246, 123_000.0, 32.0, AreaType::Urban, "Gitega"),
                    PopulationRegion::new(-3.5, 29.5, 10_600_000.0, 27_700.0, AreaType::Rural, "Rural Burundi"),
                ], 27_834.0)
            },
            
            Self::Cambodia => {
                // Cambodia: 17.0M population, 181,035 km² area, ~24% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(11.5625, 104.9160, 2_282_000.0, 679.0, AreaType::Urban, "Phnom Penh"),
                    PopulationRegion::new(13.0957, 103.1968, 190_000.0, 30.0, AreaType::Urban, "Battambang"),
                    PopulationRegion::new(13.3622, 103.8597, 189_000.0, 25.0, AreaType::Urban, "Siem Reap"),
                    PopulationRegion::new(11.5, 104.5, 12_900_000.0, 180_300.0, AreaType::Rural, "Rural Cambodia"),
                ], 181_035.0)
            },
            
            Self::Cameroon => {
                // Cameroon: 27.2M population, 475,442 km² area, ~58% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(3.8480, 11.5021, 3_976_000.0, 300.0, AreaType::Urban, "Yaoundé"),
                    PopulationRegion::new(4.0577, 9.7043, 3_529_000.0, 210.0, AreaType::Urban, "Douala"),
                    PopulationRegion::new(5.9597, 10.1458, 546_000.0, 110.0, AreaType::Urban, "Bamenda"),
                    PopulationRegion::new(10.5956, 14.3159, 393_000.0, 100.0, AreaType::Urban, "Maroua"),
                    PopulationRegion::new(6.0, 12.0, 11_500_000.0, 474_800.0, AreaType::Rural, "Rural Cameroon"),
                ], 475_442.0)
            },
            
            Self::Canada => {
                // Canada: 38.2M population, 9,984,670 km² area, ~82% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(43.6532, -79.3832, 6_418_000.0, 7_124.0, AreaType::Urban, "Toronto Metro"),
                    PopulationRegion::new(45.5017, -73.5673, 4_291_000.0, 4_259.0, AreaType::Urban, "Montreal Metro"),
                    PopulationRegion::new(49.2827, -123.1207, 2_737_000.0, 2_883.0, AreaType::Urban, "Vancouver Metro"),
                    PopulationRegion::new(51.0447, -114.0719, 1_481_000.0, 5_108.0, AreaType::Urban, "Calgary Metro"),
                    PopulationRegion::new(53.5461, -113.4938, 1_418_000.0, 9_438.0, AreaType::Urban, "Edmonton Metro"),
                    PopulationRegion::new(45.4215, -75.6972, 1_393_000.0, 6_767.0, AreaType::Urban, "Ottawa-Gatineau"),
                    PopulationRegion::new(49.8951, -97.1384, 834_000.0, 5_306.0, AreaType::Urban, "Winnipeg Metro"),
                    PopulationRegion::new(46.8139, -71.2080, 812_000.0, 3_408.0, AreaType::Urban, "Quebec City Metro"),
                    PopulationRegion::new(43.2607, -79.9194, 785_000.0, 1_371.0, AreaType::Urban, "Hamilton"),
                    PopulationRegion::new(43.0131, -81.1992, 543_000.0, 2_882.0, AreaType::Urban, "London"),
                    PopulationRegion::new(50.0, -95.0, 5_700_000.0, 9_940_000.0, AreaType::Rural, "Rural Canada"),
                ], 9_984_670.0)
            },
            
            Self::CapeVerde => {
                // Cape Verde: 563k population, 4,033 km² area, ~67% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(14.9055, -23.5087, 160_000.0, 103.0, AreaType::Urban, "Praia"),
                    PopulationRegion::new(16.8880, -24.9956, 76_000.0, 29.0, AreaType::Urban, "Mindelo"),
                    PopulationRegion::new(15.0833, -23.1167, 14_000.0, 8.0, AreaType::Urban, "Assomada"),
                    PopulationRegion::new(16.1667, -22.9167, 12_000.0, 7.0, AreaType::Urban, "Porto Novo"),
                    PopulationRegion::new(15.1, -23.6, 160_000.0, 3_886.0, AreaType::Rural, "Rural Cape Verde"),
                ], 4_033.0)
            },
            
            Self::CentralAfricanRepublic => {
                // Central African Republic: 4.9M population, 622,984 km² area, ~42% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(4.3947, 18.5582, 903_000.0, 67.0, AreaType::Urban, "Bangui"),
                    PopulationRegion::new(7.2627, 21.0059, 46_000.0, 28.0, AreaType::Urban, "Bimbo"),
                    PopulationRegion::new(6.5, 20.0, 2_900_000.0, 622_900.0, AreaType::Rural, "Rural CAR"),
                ], 622_984.0)
            },
            
            Self::Chad => {
                // Chad: 17.0M population, 1,284,000 km² area, ~23% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(12.1348, 15.0557, 1_533_000.0, 104.0, AreaType::Urban, "N'Djamena"),
                    PopulationRegion::new(8.5697, 16.0933, 135_000.0, 30.0, AreaType::Urban, "Moundou"),
                    PopulationRegion::new(9.3182, 16.0801, 110_000.0, 25.0, AreaType::Urban, "Sarh"),
                    PopulationRegion::new(15.0, 19.0, 13_100_000.0, 1_283_800.0, AreaType::Rural, "Rural Chad"),
                ], 1_284_000.0)
            },
            
            Self::Chile => {
                // Chile: 19.2M population, 756,102 km² area, ~88% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-33.4489, -70.6693, 7_412_000.0, 641.0, AreaType::Urban, "Santiago Metro"),
                    PopulationRegion::new(-33.0472, -71.6127, 1_035_000.0, 300.0, AreaType::Urban, "Valparaíso Metro"),
                    PopulationRegion::new(-36.8201, -73.0444, 1_044_000.0, 221.0, AreaType::Urban, "Concepción Metro"),
                    PopulationRegion::new(-23.6509, -70.3975, 607_000.0, 30.0, AreaType::Urban, "Antofagasta"),
                    PopulationRegion::new(-20.2208, -70.1431, 384_000.0, 20.0, AreaType::Urban, "Iquique"),
                    PopulationRegion::new(-41.4693, -72.9424, 317_000.0, 25.0, AreaType::Urban, "Puerto Montt"),
                    PopulationRegion::new(-39.8196, -73.2452, 294_000.0, 22.0, AreaType::Urban, "Valdivia"),
                    PopulationRegion::new(-18.4783, -70.3126, 287_000.0, 15.0, AreaType::Urban, "Arica"),
                    PopulationRegion::new(-38.7369, -72.6018, 278_000.0, 20.0, AreaType::Urban, "Temuco"),
                    PopulationRegion::new(-29.9045, -71.2489, 251_000.0, 18.0, AreaType::Urban, "La Serena"),
                    PopulationRegion::new(-35.0, -71.0, 1_200_000.0, 754_900.0, AreaType::Rural, "Rural Chile"),
                ], 756_102.0)
            },
            
            Self::China => {
                // China: 1,412M population, 9,596,961 km² area, ~63% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(31.2304, 121.4737, 27_796_000.0, 6_341.0, AreaType::Urban, "Shanghai"),
                    PopulationRegion::new(39.9042, 116.4074, 21_893_000.0, 16_411.0, AreaType::Urban, "Beijing"),
                    PopulationRegion::new(22.5431, 114.0579, 18_633_000.0, 3_980.0, AreaType::Urban, "Shenzhen-HK"),
                    PopulationRegion::new(23.1291, 113.2644, 18_677_000.0, 7_434.0, AreaType::Urban, "Guangzhou"),
                    PopulationRegion::new(30.5728, 104.0668, 16_045_000.0, 14_378.0, AreaType::Urban, "Chengdu"),
                    PopulationRegion::new(22.8170, 108.3665, 14_545_000.0, 7_263.0, AreaType::Urban, "Chongqing Urban"),
                    PopulationRegion::new(31.8639, 117.2808, 13_255_000.0, 11_445.0, AreaType::Urban, "Tianjin"),
                    PopulationRegion::new(30.2741, 120.1551, 11_936_000.0, 16_596.0, AreaType::Urban, "Hangzhou"),
                    PopulationRegion::new(30.5928, 114.3055, 11_895_000.0, 8_569.0, AreaType::Urban, "Wuhan"),
                    PopulationRegion::new(34.2667, 108.8833, 10_680_000.0, 10_096.0, AreaType::Urban, "Xi'an"),
                    PopulationRegion::new(32.0603, 118.7969, 9_204_000.0, 6_587.0, AreaType::Urban, "Nanjing"),
                    PopulationRegion::new(26.0745, 119.2965, 8_346_000.0, 12_155.0, AreaType::Urban, "Fuzhou"),
                    PopulationRegion::new(38.9122, 121.6021, 7_903_000.0, 13_238.0, AreaType::Urban, "Dalian"),
                    PopulationRegion::new(41.7923, 123.4328, 7_441_000.0, 12_948.0, AreaType::Urban, "Shenyang"),
                    PopulationRegion::new(36.6512, 117.1201, 6_814_000.0, 7_753.0, AreaType::Urban, "Jinan"),
                    PopulationRegion::new(25.0389, 102.7183, 6_663_000.0, 21_473.0, AreaType::Urban, "Kunming"),
                    PopulationRegion::new(43.8171, 125.3235, 5_965_000.0, 32_347.0, AreaType::Urban, "Changchun"),
                    PopulationRegion::new(45.8038, 126.5350, 5_353_000.0, 53_068.0, AreaType::Urban, "Harbin"),
                    PopulationRegion::new(29.5630, 106.5516, 5_104_000.0, 82_403.0, AreaType::Urban, "Chongqing Metro"),
                    PopulationRegion::new(35.0, 105.0, 440_000_000.0, 9_500_000.0, AreaType::Rural, "Rural China"),
                ], 9_596_961.0)
            },
            
            Self::Colombia => {
                // Colombia: 51.5M population, 1,141,748 km² area, ~81% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(4.7110, -74.0721, 11_285_000.0, 1_775.0, AreaType::Urban, "Bogotá Metro"),
                    PopulationRegion::new(6.2442, -75.5812, 4_103_000.0, 1_152.0, AreaType::Urban, "Medellín Metro"),
                    PopulationRegion::new(3.4516, -76.5320, 2_882_000.0, 564.0, AreaType::Urban, "Cali Metro"),
                    PopulationRegion::new(10.9685, -74.7813, 2_046_000.0, 154.0, AreaType::Urban, "Barranquilla Metro"),
                    PopulationRegion::new(11.2422, -74.2017, 1_358_000.0, 55.0, AreaType::Urban, "Cartagena"),
                    PopulationRegion::new(7.8836, -72.4967, 714_000.0, 143.0, AreaType::Urban, "Cúcuta"),
                    PopulationRegion::new(7.1254, -73.1198, 681_000.0, 30.0, AreaType::Urban, "Bucaramanga"),
                    PopulationRegion::new(8.7549, -75.8261, 557_000.0, 90.0, AreaType::Urban, "Montería"),
                    PopulationRegion::new(5.0689, -75.5174, 539_000.0, 25.0, AreaType::Urban, "Manizales"),
                    PopulationRegion::new(4.0, -72.0, 7_300_000.0, 1_138_000.0, AreaType::Rural, "Rural Colombia"),
                ], 1_141_748.0)
            },
            
            Self::Comoros => {
                // Comoros: 889k population, 1,861 km² area, ~29% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-11.7100, 43.2400, 62_000.0, 30.0, AreaType::Urban, "Moroni"),
                    PopulationRegion::new(-12.2800, 44.4500, 25_000.0, 15.0, AreaType::Urban, "Mutsamudu"),
                    PopulationRegion::new(-12.1667, 44.2667, 21_000.0, 12.0, AreaType::Urban, "Fomboni"),
                    PopulationRegion::new(-11.4667, 43.3333, 18_000.0, 10.0, AreaType::Urban, "Domoni"),
                    PopulationRegion::new(-11.8, 43.3, 591_000.0, 1_794.0, AreaType::Rural, "Rural Comoros"),
                ], 1_861.0)
            },
            
            Self::CostaRica => {
                // Costa Rica: 5.2M population, 51,100 km² area, ~81% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(9.9281, -84.0907, 2_158_000.0, 2_044.0, AreaType::Urban, "San José Metro"),
                    PopulationRegion::new(10.0154, -84.2142, 356_000.0, 42.0, AreaType::Urban, "Alajuela"),
                    PopulationRegion::new(9.8661, -83.9211, 167_000.0, 31.0, AreaType::Urban, "Cartago"),
                    PopulationRegion::new(10.0, -84.0, 1_000_000.0, 48_900.0, AreaType::Rural, "Rural Costa Rica"),
                ], 51_100.0)
            },
            
            Self::CotedIvoire => {
                // Côte d'Ivoire: 27.1M population, 322,463 km² area, ~52% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(5.3599, -4.0082, 5_517_000.0, 2_119.0, AreaType::Urban, "Abidjan"),
                    PopulationRegion::new(7.5399, -5.5471, 356_000.0, 70.0, AreaType::Urban, "Bouaké"),
                    PopulationRegion::new(9.4580, -6.8262, 320_000.0, 60.0, AreaType::Urban, "Korhogo"),
                    PopulationRegion::new(6.8184, -6.1456, 270_000.0, 50.0, AreaType::Urban, "Yamoussoukro"),
                    PopulationRegion::new(7.0, -5.5, 13_000_000.0, 320_000.0, AreaType::Rural, "Rural Côte d'Ivoire"),
                ], 322_463.0)
            },
            
            Self::Croatia => {
                // Croatia: 4.0M population, 56,594 km² area, ~58% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(45.8150, 15.9819, 806_000.0, 641.0, AreaType::Urban, "Zagreb"),
                    PopulationRegion::new(43.5081, 16.4402, 179_000.0, 79.0, AreaType::Urban, "Split"),
                    PopulationRegion::new(44.1194, 15.2314, 129_000.0, 58.0, AreaType::Urban, "Rijeka"),
                    PopulationRegion::new(45.5550, 18.6955, 108_000.0, 84.0, AreaType::Urban, "Osijek"),
                    PopulationRegion::new(45.3270, 14.4420, 57_000.0, 44.0, AreaType::Urban, "Pula"),
                    PopulationRegion::new(44.8666, 13.8495, 55_000.0, 38.0, AreaType::Urban, "Zadar"),
                    PopulationRegion::new(45.4873, 15.5468, 49_000.0, 36.0, AreaType::Urban, "Karlovac"),
                    PopulationRegion::new(45.0, 16.0, 1_540_000.0, 55_614.0, AreaType::Rural, "Rural Croatia"),
                ], 56_594.0)
            },
            
            Self::Cuba => {
                // Cuba: 11.3M population, 109,884 km² area, ~77% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(23.1136, -82.3666, 2_141_000.0, 728.0, AreaType::Urban, "Havana"),
                    PopulationRegion::new(20.0174, -75.8134, 433_000.0, 216.0, AreaType::Urban, "Santiago de Cuba"),
                    PopulationRegion::new(22.1456, -80.4364, 326_000.0, 43.0, AreaType::Urban, "Camagüey"),
                    PopulationRegion::new(20.3433, -76.2611, 242_000.0, 70.0, AreaType::Urban, "Holguín"),
                    PopulationRegion::new(22.0, -79.0, 2_600_000.0, 108_800.0, AreaType::Rural, "Rural Cuba"),
                ], 109_884.0)
            },
            
            Self::Cyprus => {
                // Cyprus: 1.2M population, 9,251 km² area, ~67% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(35.1856, 33.3823, 269_000.0, 111.0, AreaType::Urban, "Nicosia"),
                    PopulationRegion::new(34.6841, 33.0379, 108_000.0, 76.0, AreaType::Urban, "Limassol"),
                    PopulationRegion::new(34.9221, 33.6189, 51_000.0, 31.0, AreaType::Urban, "Larnaca"),
                    PopulationRegion::new(35.1, 33.2, 400_000.0, 9_033.0, AreaType::Rural, "Rural Cyprus"),
                ], 9_251.0)
            },
            
            Self::CzechRepublic => {
                // Czech Republic: 10.5M population, 78,867 km² area, ~74% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(50.0755, 14.4378, 2_165_000.0, 496.0, AreaType::Urban, "Prague Metro"),
                    PopulationRegion::new(49.1951, 16.6068, 697_000.0, 230.0, AreaType::Urban, "Brno Metro"),
                    PopulationRegion::new(49.8398, 18.2820, 325_000.0, 214.0, AreaType::Urban, "Ostrava"),
                    PopulationRegion::new(49.7437, 13.3736, 172_000.0, 138.0, AreaType::Urban, "Plzeň"),
                    PopulationRegion::new(50.0, 15.0, 2_700_000.0, 77_800.0, AreaType::Rural, "Rural Czech Republic"),
                ], 78_867.0)
            },
            
            Self::DemocraticRepublicOfTheCongo => {
                // DRC: 95.9M population, 2,344,858 km² area, ~46% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-4.4419, 15.2663, 14_970_000.0, 9_965.0, AreaType::Urban, "Kinshasa"),
                    PopulationRegion::new(-11.6609, 27.4794, 2_866_000.0, 747.0, AreaType::Urban, "Lubumbashi"),
                    PopulationRegion::new(-5.0459, 23.5236, 1_602_000.0, 200.0, AreaType::Urban, "Mbuji-Mayi"),
                    PopulationRegion::new(-1.6596, 29.2204, 1_080_000.0, 90.0, AreaType::Urban, "Goma"),
                    PopulationRegion::new(0.5143, 25.1897, 1_054_000.0, 85.0, AreaType::Urban, "Kisangani"),
                    PopulationRegion::new(-2.0, 23.0, 51_700_000.0, 2_333_000.0, AreaType::Rural, "Rural DRC"),
                ], 2_344_858.0)
            },
            
            Self::DemocraticRepublicOfTimorLeste => {
                // Timor-Leste: 1.3M population, 14,919 km² area, ~31% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-8.5569, 125.5603, 281_000.0, 49.0, AreaType::Urban, "Dili"),
                    PopulationRegion::new(-8.7839, 126.3593, 29_000.0, 13.0, AreaType::Urban, "Baucau"),
                    PopulationRegion::new(-8.7, 125.9, 917_000.0, 14_857.0, AreaType::Rural, "Rural Timor-Leste"),
                ], 14_919.0)
            },
            
            Self::Denmark => {
                // Denmark: 5.9M population, 43,094 km² area, ~88% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(55.6761, 12.5683, 1_363_000.0, 180.0, AreaType::Urban, "Copenhagen Metro"),
                    PopulationRegion::new(56.1629, 10.2039, 349_000.0, 91.0, AreaType::Urban, "Aarhus"),
                    PopulationRegion::new(55.4038, 10.4024, 216_000.0, 79.0, AreaType::Urban, "Odense"),
                    PopulationRegion::new(57.0488, 9.9217, 143_000.0, 558.0, AreaType::Urban, "Aalborg"),
                    PopulationRegion::new(56.0, 10.0, 700_000.0, 42_000.0, AreaType::Rural, "Rural Denmark"),
                ], 43_094.0)
            },
            
            Self::Djibouti => {
                // Djibouti: 1.0M population, 23,200 km² area, ~78% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(11.5721, 43.1456, 624_000.0, 630.0, AreaType::Urban, "Djibouti City"),
                    PopulationRegion::new(11.7886, 42.5903, 37_000.0, 25.0, AreaType::Urban, "Ali Sabieh"),
                    PopulationRegion::new(11.5, 42.5, 220_000.0, 22_545.0, AreaType::Rural, "Rural Djibouti"),
                ], 23_200.0)
            },
            
            Self::Dominica => {
                // Dominica: 72k population, 750 km² area, ~71% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(15.3010, -61.3863, 15_000.0, 13.0, AreaType::Urban, "Roseau"),
                    PopulationRegion::new(15.4316, -61.3707, 3_000.0, 5.0, AreaType::Urban, "Portsmouth"),
                    PopulationRegion::new(15.4, -61.3, 21_000.0, 732.0, AreaType::Rural, "Rural Dominica"),
                ], 750.0)
            },
            
            Self::DominicanRepublic => {
                // Dominican Republic: 10.8M population, 48,671 km² area, ~83% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(18.4861, -69.9312, 3_524_000.0, 1_401.0, AreaType::Urban, "Santo Domingo Metro"),
                    PopulationRegion::new(19.4436, -70.6843, 1_164_000.0, 394.0, AreaType::Urban, "Santiago"),
                    PopulationRegion::new(18.4269, -68.9655, 347_000.0, 84.0, AreaType::Urban, "La Romana"),
                    PopulationRegion::new(18.5430, -69.3203, 331_000.0, 45.0, AreaType::Urban, "San Pedro de Macorís"),
                    PopulationRegion::new(19.0, -70.0, 1_870_000.0, 46_700.0, AreaType::Rural, "Rural Dominican Republic"),
                ], 48_671.0)
            },
            
            Self::Ecuador => {
                // Ecuador: 17.9M population, 283,561 km² area, ~64% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-2.1962, -79.8862, 3_094_000.0, 344.0, AreaType::Urban, "Guayaquil Metro"),
                    PopulationRegion::new(-0.1807, -78.4678, 2_239_000.0, 372.0, AreaType::Urban, "Quito Metro"),
                    PopulationRegion::new(-2.9001, -79.0058, 661_000.0, 75.0, AreaType::Urban, "Cuenca"),
                    PopulationRegion::new(-1.2491, -78.6196, 418_000.0, 43.0, AreaType::Urban, "Ambato"),
                    PopulationRegion::new(-1.0, -78.0, 6_400_000.0, 282_700.0, AreaType::Rural, "Rural Ecuador"),
                ], 283_561.0)
            },
            
            Self::Egypt => {
                // Egypt: 104.3M population, 1,001,450 km² area, ~43% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(30.0444, 31.2357, 21_323_000.0, 3_085.0, AreaType::Urban, "Cairo Metro"),
                    PopulationRegion::new(31.2001, 29.9187, 5_381_000.0, 2_679.0, AreaType::Urban, "Alexandria"),
                    PopulationRegion::new(30.5927, 32.2714, 800_000.0, 100.0, AreaType::Urban, "Port Said"),
                    PopulationRegion::new(29.9668, 32.5498, 711_000.0, 110.0, AreaType::Urban, "Suez"),
                    PopulationRegion::new(25.6872, 32.6396, 545_000.0, 417.0, AreaType::Urban, "Luxor"),
                    PopulationRegion::new(27.1811, 31.1858, 538_000.0, 50.0, AreaType::Urban, "Asyut"),
                    PopulationRegion::new(30.5765, 31.5041, 522_000.0, 45.0, AreaType::Urban, "Tanta"),
                    PopulationRegion::new(31.0409, 31.3785, 495_000.0, 40.0, AreaType::Urban, "Mansoura"),
                    PopulationRegion::new(30.1282, 31.3429, 483_000.0, 35.0, AreaType::Urban, "Zagazig"),
                    PopulationRegion::new(24.0889, 32.8998, 474_000.0, 30.0, AreaType::Urban, "Aswan"),
                    PopulationRegion::new(27.0, 30.0, 56_900_000.0, 997_600.0, AreaType::Rural, "Rural Egypt"),
                ], 1_001_450.0)
            },
            
            Self::ElSalvador => {
                // El Salvador: 6.5M population, 21,041 km² area, ~74% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(13.6929, -89.2182, 1_107_000.0, 72.0, AreaType::Urban, "San Salvador"),
                    PopulationRegion::new(14.3038, -89.4462, 264_000.0, 55.0, AreaType::Urban, "Santa Ana"),
                    PopulationRegion::new(13.4889, -88.1806, 245_000.0, 40.0, AreaType::Urban, "San Miguel"),
                    PopulationRegion::new(13.8, -89.0, 1_700_000.0, 20_900.0, AreaType::Rural, "Rural El Salvador"),
                ], 21_041.0)
            },
            
            Self::EquatorialGuinea => {
                // Equatorial Guinea: 1.5M population, 28,051 km² area, ~73% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(3.7504, 8.7821, 333_000.0, 18.0, AreaType::Urban, "Malabo"),
                    PopulationRegion::new(1.8501, 9.7713, 246_000.0, 25.0, AreaType::Urban, "Bata"),
                    PopulationRegion::new(1.5, 10.0, 400_000.0, 28_008.0, AreaType::Rural, "Rural Equatorial Guinea"),
                ], 28_051.0)
            },
            
            Self::Eritrea => {
                // Eritrea: 3.6M population, 117,600 km² area, ~42% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(15.3389, 38.9252, 803_000.0, 45.0, AreaType::Urban, "Asmara"),
                    PopulationRegion::new(15.6109, 39.4545, 91_000.0, 20.0, AreaType::Urban, "Massawa"),
                    PopulationRegion::new(14.8897, 38.8829, 82_000.0, 15.0, AreaType::Urban, "Keren"),
                    PopulationRegion::new(15.5, 38.5, 2_100_000.0, 117_500.0, AreaType::Rural, "Rural Eritrea"),
                ], 117_600.0)
            },
            
            Self::Estonia => {
                // Estonia: 1.3M population, 45,228 km² area, ~69% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(59.4370, 24.7536, 445_000.0, 159.0, AreaType::Urban, "Tallinn"),
                    PopulationRegion::new(58.3776, 26.7291, 96_000.0, 39.0, AreaType::Urban, "Tartu"),
                    PopulationRegion::new(59.3776, 27.4185, 55_000.0, 30.0, AreaType::Urban, "Narva"),
                    PopulationRegion::new(58.5, 25.5, 415_000.0, 45_000.0, AreaType::Rural, "Rural Estonia"),
                ], 45_228.0)
            },
            
            Self::Ethiopia => {
                // Ethiopia: 120.3M population, 1,104,300 km² area, ~22% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(9.0250, 38.7469, 5_006_000.0, 527.0, AreaType::Urban, "Addis Ababa"),
                    PopulationRegion::new(7.6781, 36.8289, 605_000.0, 169.0, AreaType::Urban, "Arba Minch"),
                    PopulationRegion::new(11.5931, 37.3964, 534_000.0, 213.0, AreaType::Urban, "Bahir Dar"),
                    PopulationRegion::new(6.8227, 37.7282, 500_000.0, 98.0, AreaType::Urban, "Hawassa"),
                    PopulationRegion::new(11.1274, 39.6365, 497_000.0, 150.0, AreaType::Urban, "Mekelle"),
                    PopulationRegion::new(7.5522, 38.4872, 445_000.0, 120.0, AreaType::Urban, "Jimma"),
                    PopulationRegion::new(9.3225, 42.1198, 437_000.0, 130.0, AreaType::Urban, "Dire Dawa"),
                    PopulationRegion::new(13.4937, 39.4764, 423_000.0, 100.0, AreaType::Urban, "Gonder"),
                    PopulationRegion::new(8.5500, 39.2667, 401_000.0, 90.0, AreaType::Urban, "Shashemene"),
                    PopulationRegion::new(9.0107, 38.7613, 385_000.0, 80.0, AreaType::Urban, "Adama"),
                    PopulationRegion::new(9.0, 39.0, 88_400_000.0, 1_103_000.0, AreaType::Rural, "Rural Ethiopia"),
                ], 1_104_300.0)
            },
            
            Self::FederatedStatesOfMicronesia => {
                // Federated States of Micronesia: 115k population, 702 km² area, ~23% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(6.9248, 158.1610, 7_000.0, 12.0, AreaType::Urban, "Palikir"),
                    PopulationRegion::new(6.8927, 158.2339, 7_000.0, 10.0, AreaType::Urban, "Kolonia"),
                    PopulationRegion::new(7.4456, 151.8488, 3_500.0, 8.0, AreaType::Urban, "Weno"),
                    PopulationRegion::new(7.3315, 151.5033, 3_000.0, 6.0, AreaType::Urban, "Tol"),
                    PopulationRegion::new(7.0, 158.0, 81_500.0, 666.0, AreaType::Rural, "Rural Micronesia"),
                ], 702.0)
            },
            
            Self::Fiji => {
                // Fiji: 903k population, 18,274 km² area, ~57% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-18.1248, 178.4501, 341_000.0, 2_048.0, AreaType::Urban, "Suva Metro"),
                    PopulationRegion::new(-17.6134, 177.4481, 52_000.0, 32.0, AreaType::Urban, "Lautoka"),
                    PopulationRegion::new(-17.7765, 177.9098, 72_000.0, 57.0, AreaType::Urban, "Nadi"),
                    PopulationRegion::new(-17.5333, 178.8167, 50_000.0, 30.0, AreaType::Urban, "Labasa"),
                    PopulationRegion::new(-17.8000, 178.0000, 44_000.0, 25.0, AreaType::Urban, "Ba"),
                    PopulationRegion::new(-18.0, 178.0, 295_000.0, 16_082.0, AreaType::Rural, "Rural Fiji"),
                ], 18_274.0)
            },
            
            Self::Finland => {
                // Finland: 5.5M population, 338,424 km² area, ~85% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(60.1699, 24.9384, 1_305_000.0, 770.0, AreaType::Urban, "Helsinki Metro"),
                    PopulationRegion::new(61.4978, 23.7610, 347_000.0, 689.0, AreaType::Urban, "Tampere"),
                    PopulationRegion::new(60.4515, 22.2688, 196_000.0, 306.0, AreaType::Urban, "Turku"),
                    PopulationRegion::new(65.0121, 25.4651, 141_000.0, 1_410.0, AreaType::Urban, "Oulu"),
                    PopulationRegion::new(63.0, 27.0, 820_000.0, 335_000.0, AreaType::Rural, "Rural Finland"),
                ], 338_424.0)
            },
            
            Self::France => {
                // France: 67.8M population, 643,801 km² area, ~81% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(48.8566, 2.3522, 11_027_000.0, 2_845.0, AreaType::Urban, "Paris Metro"),
                    PopulationRegion::new(45.7640, 4.8357, 2_323_000.0, 6_019.0, AreaType::Urban, "Lyon Metro"),
                    PopulationRegion::new(43.2965, 5.3698, 1_760_000.0, 3_173.0, AreaType::Urban, "Marseille Metro"),
                    PopulationRegion::new(43.6047, 1.4442, 1_360_000.0, 5_381.0, AreaType::Urban, "Toulouse Metro"),
                    PopulationRegion::new(50.6292, 3.0573, 1_187_000.0, 977.0, AreaType::Urban, "Lille Metro"),
                    PopulationRegion::new(44.8378, -0.5792, 1_247_000.0, 5_613.0, AreaType::Urban, "Bordeaux Metro"),
                    PopulationRegion::new(47.2184, -1.5536, 972_000.0, 3_378.0, AreaType::Urban, "Nantes Metro"),
                    PopulationRegion::new(48.1113, -1.6742, 733_000.0, 704.0, AreaType::Urban, "Rennes"),
                    PopulationRegion::new(43.6047, 7.2644, 1_006_000.0, 2_383.0, AreaType::Urban, "Nice Metro"),
                    PopulationRegion::new(45.1880, 5.7240, 689_000.0, 2_621.0, AreaType::Urban, "Grenoble Metro"),
                    PopulationRegion::new(47.0, 2.0, 10_600_000.0, 620_000.0, AreaType::Rural, "Rural France"),
                ], 643_801.0)
            },
            
            Self::Gabon => {
                // Gabon: 2.3M population, 267,668 km² area, ~90% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(0.4162, 9.4673, 831_000.0, 189.0, AreaType::Urban, "Libreville"),
                    PopulationRegion::new(-0.7001, 8.7809, 285_000.0, 81.0, AreaType::Urban, "Port-Gentil"),
                    PopulationRegion::new(-1.9569, 11.4853, 61_000.0, 20.0, AreaType::Urban, "Franceville"),
                    PopulationRegion::new(-0.5, 10.0, 230_000.0, 267_400.0, AreaType::Rural, "Rural Gabon"),
                ], 267_668.0)
            },
            
            Self::TheGambia => {
                // The Gambia: 2.5M population, 11,295 km² area, ~63% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(13.4432, -16.6781, 437_000.0, 93.0, AreaType::Urban, "Banjul Metro"),
                    PopulationRegion::new(13.4531, -16.5775, 393_000.0, 76.0, AreaType::Urban, "Serekunda"),
                    PopulationRegion::new(13.4418, -16.5636, 50_000.0, 10.0, AreaType::Urban, "Brikama"),
                    PopulationRegion::new(13.5, -16.0, 950_000.0, 11_100.0, AreaType::Rural, "Rural Gambia"),
                ], 11_295.0)
            },
            
            Self::Georgia => {
                // Georgia: 3.7M population, 69,700 km² area, ~60% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(41.7151, 44.8271, 1_165_000.0, 504.0, AreaType::Urban, "Tbilisi"),
                    PopulationRegion::new(42.2662, 42.7180, 147_000.0, 64.0, AreaType::Urban, "Kutaisi"),
                    PopulationRegion::new(41.6168, 41.6367, 184_000.0, 66.0, AreaType::Urban, "Batumi"),
                    PopulationRegion::new(42.2529, 43.9653, 57_000.0, 30.0, AreaType::Urban, "Gori"),
                    PopulationRegion::new(42.0, 43.5, 1_490_000.0, 69_000.0, AreaType::Rural, "Rural Georgia"),
                ], 69_700.0)
            },
            
            Self::Germany => {
                // Germany: 83.2M population, 357,588 km² area, ~77% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(52.5200, 13.4050, 3_769_000.0, 891.0, AreaType::Urban, "Berlin"),
                    PopulationRegion::new(53.5511, 9.9937, 1_899_000.0, 755.0, AreaType::Urban, "Hamburg"),
                    PopulationRegion::new(48.1351, 11.5820, 1_484_000.0, 311.0, AreaType::Urban, "Munich"),
                    PopulationRegion::new(50.9375, 6.9603, 1_085_000.0, 405.0, AreaType::Urban, "Cologne"),
                    PopulationRegion::new(50.1109, 8.6821, 753_000.0, 248.0, AreaType::Urban, "Frankfurt"),
                    PopulationRegion::new(48.7758, 9.1829, 634_000.0, 207.0, AreaType::Urban, "Stuttgart"),
                    PopulationRegion::new(51.2277, 6.7735, 619_000.0, 217.0, AreaType::Urban, "Düsseldorf"),
                    PopulationRegion::new(51.4556, 7.0116, 586_000.0, 210.0, AreaType::Urban, "Essen"),
                    PopulationRegion::new(51.3397, 12.3731, 587_000.0, 297.0, AreaType::Urban, "Leipzig"),
                    PopulationRegion::new(51.3127, 7.4639, 582_000.0, 280.0, AreaType::Urban, "Dortmund"),
                    PopulationRegion::new(51.0504, 13.7373, 556_000.0, 328.0, AreaType::Urban, "Dresden"),
                    PopulationRegion::new(49.4521, 11.0768, 532_000.0, 186.0, AreaType::Urban, "Nuremberg"),
                    PopulationRegion::new(52.3759, 9.7320, 532_000.0, 204.0, AreaType::Urban, "Hanover"),
                    PopulationRegion::new(51.5136, 7.4653, 519_000.0, 168.0, AreaType::Urban, "Bochum"),
                    PopulationRegion::new(51.0, 10.0, 15_600_000.0, 354_000.0, AreaType::Rural, "Rural Germany"),
                ], 357_588.0)
            },
            
            Self::Ghana => {
                // Ghana: 31.7M population, 238,533 km² area, ~57% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(5.6037, -0.1870, 5_055_000.0, 225.0, AreaType::Urban, "Accra Metro"),
                    PopulationRegion::new(6.6885, -1.6244, 2_557_000.0, 254.0, AreaType::Urban, "Kumasi"),
                    PopulationRegion::new(5.1136, -1.2821, 289_000.0, 103.0, AreaType::Urban, "Sekondi-Takoradi"),
                    PopulationRegion::new(9.4034, -0.8424, 235_000.0, 750.0, AreaType::Urban, "Tamale"),
                    PopulationRegion::new(8.0, -1.0, 13_600_000.0, 237_300.0, AreaType::Rural, "Rural Ghana"),
                ], 238_533.0)
            },
            
            Self::Greece => {
                // Greece: 10.7M population, 131,957 km² area, ~79% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(37.9838, 23.7275, 3_690_000.0, 2_929.0, AreaType::Urban, "Athens Metro"),
                    PopulationRegion::new(40.6401, 22.9444, 1_110_000.0, 1_286.0, AreaType::Urban, "Thessaloniki Metro"),
                    PopulationRegion::new(38.2466, 21.7346, 214_000.0, 125.0, AreaType::Urban, "Patras"),
                    PopulationRegion::new(35.3387, 25.1442, 174_000.0, 336.0, AreaType::Urban, "Heraklion"),
                    PopulationRegion::new(37.0741, 22.4303, 139_000.0, 90.0, AreaType::Urban, "Kalamata"),
                    PopulationRegion::new(35.5138, 24.0180, 122_000.0, 240.0, AreaType::Urban, "Chania"),
                    PopulationRegion::new(36.4167, 28.2167, 105_000.0, 200.0, AreaType::Urban, "Rhodes"),
                    PopulationRegion::new(37.9500, 23.6333, 100_000.0, 180.0, AreaType::Urban, "Piraeus"),
                    PopulationRegion::new(39.6650, 20.8537, 85_000.0, 120.0, AreaType::Urban, "Ioannina"),
                    PopulationRegion::new(38.9000, 22.4333, 75_000.0, 100.0, AreaType::Urban, "Lamia"),
                    PopulationRegion::new(39.5, 22.0, 1_460_000.0, 127_300.0, AreaType::Rural, "Rural Greece"),
                ], 131_957.0)
            },
            
            Self::Grenada => {
                // Grenada: 113k population, 349 km² area, ~36% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(12.0540, -61.7486, 40_000.0, 65.0, AreaType::Urban, "St. George's"),
                    PopulationRegion::new(12.1, -61.7, 72_000.0, 284.0, AreaType::Rural, "Rural Grenada"),
                ], 349.0)
            },
            
            Self::Guatemala => {
                // Guatemala: 17.3M population, 108,889 km² area, ~52% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(14.6407, -90.5133, 2_983_000.0, 996.0, AreaType::Urban, "Guatemala City Metro"),
                    PopulationRegion::new(14.8344, -91.5185, 1_125_000.0, 484.0, AreaType::Urban, "Quetzaltenango"),
                    PopulationRegion::new(14.9589, -90.9833, 184_000.0, 50.0, AreaType::Urban, "Escuintla"),
                    PopulationRegion::new(15.0, -91.0, 8_400_000.0, 107_400.0, AreaType::Rural, "Rural Guatemala"),
                ], 108_889.0)
            },
            
            Self::Guinea => {
                // Guinea: 13.5M population, 245,857 km² area, ~37% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(9.6412, -13.5784, 1_991_000.0, 450.0, AreaType::Urban, "Conakry"),
                    PopulationRegion::new(10.7356, -10.3860, 235_000.0, 50.0, AreaType::Urban, "Kankan"),
                    PopulationRegion::new(11.3182, -12.2887, 146_000.0, 30.0, AreaType::Urban, "Kindia"),
                    PopulationRegion::new(10.5, -11.0, 8_500_000.0, 245_300.0, AreaType::Rural, "Rural Guinea"),
                ], 245_857.0)
            },
            
            Self::GuineaBissau => {
                // Guinea-Bissau: 2.0M population, 36,125 km² area, ~44% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(11.8636, -15.5977, 643_000.0, 77.0, AreaType::Urban, "Bissau"),
                    PopulationRegion::new(12.2833, -14.7667, 52_000.0, 12.0, AreaType::Urban, "Bafatá"),
                    PopulationRegion::new(12.0, -15.0, 1_130_000.0, 36_000.0, AreaType::Rural, "Rural Guinea-Bissau"),
                ], 36_125.0)
            },
            
            Self::Guyana => {
                // Guyana: 787k population, 214,969 km² area, ~27% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(6.8013, -58.1551, 118_000.0, 70.0, AreaType::Urban, "Georgetown"),
                    PopulationRegion::new(6.0045, -57.4795, 25_000.0, 20.0, AreaType::Urban, "Linden"),
                    PopulationRegion::new(5.0, -59.0, 575_000.0, 214_900.0, AreaType::Rural, "Rural Guyana"),
                ], 214_969.0)
            },
            
            Self::Haiti => {
                // Haiti: 11.5M population, 27,750 km² area, ~58% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(18.5944, -72.3074, 2_915_000.0, 158.0, AreaType::Urban, "Port-au-Prince Metro"),
                    PopulationRegion::new(19.7567, -72.2025, 375_000.0, 21.0, AreaType::Urban, "Cap-Haïtien"),
                    PopulationRegion::new(18.5396, -72.3361, 172_000.0, 10.0, AreaType::Urban, "Pétionville"),
                    PopulationRegion::new(19.0, -72.5, 4_900_000.0, 27_500.0, AreaType::Rural, "Rural Haiti"),
                ], 27_750.0)
            },
            
            Self::Honduras => {
                // Honduras: 10.1M population, 112,492 km² area, ~59% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(14.0723, -87.1921, 1_444_000.0, 220.0, AreaType::Urban, "Tegucigalpa"),
                    PopulationRegion::new(15.5047, -88.0237, 929_000.0, 92.0, AreaType::Urban, "San Pedro Sula"),
                    PopulationRegion::new(15.7800, -86.7892, 182_000.0, 30.0, AreaType::Urban, "La Ceiba"),
                    PopulationRegion::new(15.0, -87.0, 4_150_000.0, 112_100.0, AreaType::Rural, "Rural Honduras"),
                ], 112_492.0)
            },
            
            Self::Hungary => {
                // Hungary: 9.7M population, 93,028 km² area, ~72% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(47.4979, 19.0402, 3_011_000.0, 525.0, AreaType::Urban, "Budapest Metro"),
                    PopulationRegion::new(47.6875, 17.6504, 179_000.0, 282.0, AreaType::Urban, "Győr"),
                    PopulationRegion::new(46.0727, 18.2323, 161_000.0, 161.0, AreaType::Urban, "Pécs"),
                    PopulationRegion::new(46.2530, 20.1480, 170_000.0, 281.0, AreaType::Urban, "Szeged"),
                    PopulationRegion::new(47.5, 19.0, 2_740_000.0, 91_800.0, AreaType::Rural, "Rural Hungary"),
                ], 93_028.0)
            },
            
            Self::Iceland => {
                // Iceland: 372k population, 103,000 km² area, ~94% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(64.1466, -21.9426, 233_000.0, 273.0, AreaType::Urban, "Reykjavík Metro"),
                    PopulationRegion::new(64.1265, -21.8174, 13_000.0, 10.0, AreaType::Urban, "Kópavogur"),
                    PopulationRegion::new(64.0671, -21.9602, 12_000.0, 8.0, AreaType::Urban, "Hafnarfjörður"),
                    PopulationRegion::new(65.0, -19.0, 22_000.0, 102_700.0, AreaType::Rural, "Rural Iceland"),
                ], 103_000.0)
            },
            
            Self::India => {
                // India: 1,408M population, 3,287,263 km² area, ~35% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(28.7041, 77.1025, 32_941_000.0, 1_425.0, AreaType::Urban, "Delhi"),
                    PopulationRegion::new(19.0760, 72.8777, 20_961_000.0, 550.0, AreaType::Urban, "Mumbai"),
                    PopulationRegion::new(22.5726, 88.3639, 15_134_000.0, 1_851.0, AreaType::Urban, "Kolkata"),
                    PopulationRegion::new(12.9716, 77.5946, 13_193_000.0, 741.0, AreaType::Urban, "Bangalore"),
                    PopulationRegion::new(13.0827, 80.2707, 11_504_000.0, 1_049.0, AreaType::Urban, "Chennai"),
                    PopulationRegion::new(17.3850, 78.4867, 10_494_000.0, 625.0, AreaType::Urban, "Hyderabad"),
                    PopulationRegion::new(23.0225, 72.5714, 8_588_000.0, 464.0, AreaType::Urban, "Ahmedabad"),
                    PopulationRegion::new(18.5204, 73.8567, 7_412_000.0, 582.0, AreaType::Urban, "Pune"),
                    PopulationRegion::new(22.3072, 73.1812, 6_357_000.0, 331.0, AreaType::Urban, "Surat"),
                    PopulationRegion::new(26.4499, 80.3319, 3_683_000.0, 285.0, AreaType::Urban, "Kanpur"),
                    PopulationRegion::new(26.8467, 80.9462, 3_597_000.0, 402.0, AreaType::Urban, "Lucknow"),
                    PopulationRegion::new(21.1458, 79.0882, 3_004_000.0, 227.0, AreaType::Urban, "Nagpur"),
                    PopulationRegion::new(26.9124, 75.7873, 3_928_000.0, 484.0, AreaType::Urban, "Jaipur"),
                    PopulationRegion::new(23.2599, 77.4126, 2_427_000.0, 293.0, AreaType::Urban, "Bhopal"),
                    PopulationRegion::new(17.6868, 83.2185, 2_358_000.0, 682.0, AreaType::Urban, "Visakhapatnam"),
                    PopulationRegion::new(25.5941, 85.1376, 2_300_000.0, 201.0, AreaType::Urban, "Patna"),
                    PopulationRegion::new(13.0878, 80.2785, 2_283_000.0, 175.0, AreaType::Urban, "Vadodara"),
                    PopulationRegion::new(22.7196, 75.8577, 2_171_000.0, 244.0, AreaType::Urban, "Indore"),
                    PopulationRegion::new(30.7333, 76.7794, 2_122_000.0, 158.0, AreaType::Urban, "Chandigarh"),
                    PopulationRegion::new(25.3478, 72.3106, 2_019_000.0, 149.0, AreaType::Urban, "Jodhpur"),
                    PopulationRegion::new(23.0, 78.0, 880_000_000.0, 3_275_000.0, AreaType::Rural, "Rural India"),
                ], 3_287_263.0)
            },
            
            Self::Indonesia => {
                // Indonesia: 273.8M population, 1,904,569 km² area, ~57% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-6.2088, 106.8456, 34_540_000.0, 3_540.0, AreaType::Urban, "Jakarta Metro"),
                    PopulationRegion::new(-7.2575, 112.7521, 10_098_000.0, 5_343.0, AreaType::Urban, "Surabaya Metro"),
                    PopulationRegion::new(-6.9175, 107.6191, 9_049_000.0, 2_875.0, AreaType::Urban, "Bandung Metro"),
                    PopulationRegion::new(3.5952, 98.6722, 5_253_000.0, 479.0, AreaType::Urban, "Medan Metro"),
                    PopulationRegion::new(-6.9667, 110.4167, 4_679_000.0, 210.0, AreaType::Urban, "Semarang Metro"),
                    PopulationRegion::new(-5.1477, 119.4327, 3_368_000.0, 2_226.0, AreaType::Urban, "Makassar Metro"),
                    PopulationRegion::new(-0.9493, 100.3543, 2_658_000.0, 694.0, AreaType::Urban, "Padang Metro"),
                    PopulationRegion::new(-3.8000, 102.2667, 2_010_000.0, 315.0, AreaType::Urban, "Palembang"),
                    PopulationRegion::new(1.4926, 124.8414, 1_895_000.0, 409.0, AreaType::Urban, "Manado"),
                    PopulationRegion::new(-0.8917, 119.8707, 1_423_000.0, 333.0, AreaType::Urban, "Palu"),
                    PopulationRegion::new(-8.5833, 116.1167, 1_407_000.0, 222.0, AreaType::Urban, "Mataram"),
                    PopulationRegion::new(-0.0263, 109.3425, 1_396_000.0, 230.0, AreaType::Urban, "Pontianak"),
                    PopulationRegion::new(-8.6500, 115.2167, 1_327_000.0, 5_780.0, AreaType::Urban, "Denpasar"),
                    PopulationRegion::new(-2.9909, 104.7754, 1_312_000.0, 358.0, AreaType::Urban, "Jambi"),
                    PopulationRegion::new(-3.0, 120.0, 95_700_000.0, 1_890_000.0, AreaType::Rural, "Rural Indonesia"),
                ], 1_904_569.0)
            },
            
            Self::Iran => {
                // Iran: 85.0M population, 1,648,195 km² area, ~76% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(35.6892, 51.3890, 9_382_000.0, 730.0, AreaType::Urban, "Tehran"),
                    PopulationRegion::new(36.2605, 59.6168, 3_372_000.0, 328.0, AreaType::Urban, "Mashhad"),
                    PopulationRegion::new(32.6546, 51.6680, 2_132_000.0, 106.0, AreaType::Urban, "Isfahan"),
                    PopulationRegion::new(35.3219, 46.9982, 1_920_000.0, 234.0, AreaType::Urban, "Kermanshah"),
                    PopulationRegion::new(38.0792, 46.2919, 1_841_000.0, 131.0, AreaType::Urban, "Tabriz"),
                    PopulationRegion::new(29.5918, 52.5836, 1_750_000.0, 240.0, AreaType::Urban, "Shiraz"),
                    PopulationRegion::new(37.2807, 49.5831, 1_703_000.0, 162.0, AreaType::Urban, "Rasht"),
                    PopulationRegion::new(31.3183, 48.6706, 1_569_000.0, 134.0, AreaType::Urban, "Ahvaz"),
                    PopulationRegion::new(36.3107, 59.5989, 1_245_000.0, 92.0, AreaType::Urban, "Zahedan"),
                    PopulationRegion::new(34.6416, 50.8746, 1_228_000.0, 106.0, AreaType::Urban, "Qom"),
                    PopulationRegion::new(32.0, 53.0, 14_600_000.0, 1_646_000.0, AreaType::Rural, "Rural Iran"),
                ], 1_648_195.0)
            },
            
            Self::Iraq => {
                // Iraq: 41.2M population, 438,317 km² area, ~71% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(33.3152, 44.3661, 8_765_000.0, 204.0, AreaType::Urban, "Baghdad"),
                    PopulationRegion::new(36.3489, 43.1577, 1_846_000.0, 180.0, AreaType::Urban, "Mosul"),
                    PopulationRegion::new(30.5234, 47.7919, 1_497_000.0, 117.0, AreaType::Urban, "Basra"),
                    PopulationRegion::new(36.1905, 44.0119, 1_080_000.0, 70.0, AreaType::Urban, "Erbil"),
                    PopulationRegion::new(35.4679, 44.3923, 875_000.0, 60.0, AreaType::Urban, "Kirkuk"),
                    PopulationRegion::new(31.9960, 44.3150, 821_000.0, 54.0, AreaType::Urban, "Najaf"),
                    PopulationRegion::new(32.4637, 44.0249, 711_000.0, 48.0, AreaType::Urban, "Karbala"),
                    PopulationRegion::new(32.6149, 44.0227, 656_000.0, 43.0, AreaType::Urban, "Hillah"),
                    PopulationRegion::new(32.0, 44.0, 10_000_000.0, 437_700.0, AreaType::Rural, "Rural Iraq"),
                ], 438_317.0)
            },
            
            Self::Ireland => {
                // Ireland: 5.0M population, 70,273 km² area, ~64% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(53.3498, -6.2603, 1_388_000.0, 318.0, AreaType::Urban, "Dublin Metro"),
                    PopulationRegion::new(51.8969, -8.4863, 310_000.0, 187.0, AreaType::Urban, "Cork Metro"),
                    PopulationRegion::new(53.2707, -9.0568, 195_000.0, 150.0, AreaType::Urban, "Galway"),
                    PopulationRegion::new(52.6638, -8.6267, 117_000.0, 54.0, AreaType::Urban, "Limerick"),
                    PopulationRegion::new(53.0, -8.0, 1_800_000.0, 69_500.0, AreaType::Rural, "Rural Ireland"),
                ], 70_273.0)
            },
            
            Self::Israel => {
                // Israel: 9.4M population, 20,770 km² area, ~93% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(32.0853, 34.7818, 4_343_000.0, 1_554.0, AreaType::Urban, "Tel Aviv Metro"),
                    PopulationRegion::new(31.7683, 35.2137, 1_158_000.0, 125.0, AreaType::Urban, "Jerusalem"),
                    PopulationRegion::new(32.7940, 34.9896, 924_000.0, 290.0, AreaType::Urban, "Haifa Metro"),
                    PopulationRegion::new(31.2530, 34.7915, 217_000.0, 62.0, AreaType::Urban, "Beersheba"),
                    PopulationRegion::new(32.0876, 34.8881, 197_000.0, 55.0, AreaType::Urban, "Rishon LeZion"),
                    PopulationRegion::new(32.3261, 34.8516, 193_000.0, 52.0, AreaType::Urban, "Netanya"),
                    PopulationRegion::new(31.8894, 34.8031, 181_000.0, 48.0, AreaType::Urban, "Ashdod"),
                    PopulationRegion::new(32.0, 35.0, 260_000.0, 18_584.0, AreaType::Rural, "Rural Israel"),
                ], 20_770.0)
            },
            
            Self::Italy => {
                // Italy: 59.1M population, 301,340 km² area, ~71% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(41.9028, 12.4964, 4_342_000.0, 1_287.0, AreaType::Urban, "Rome Metro"),
                    PopulationRegion::new(45.4642, 9.1900, 4_336_000.0, 1_575.0, AreaType::Urban, "Milan Metro"),
                    PopulationRegion::new(40.8518, 14.2681, 3_085_000.0, 1_171.0, AreaType::Urban, "Naples Metro"),
                    PopulationRegion::new(45.0703, 7.6869, 1_720_000.0, 335.0, AreaType::Urban, "Turin Metro"),
                    PopulationRegion::new(38.1157, 13.3613, 1_252_000.0, 5_009.0, AreaType::Urban, "Palermo Metro"),
                    PopulationRegion::new(44.4949, 11.3426, 1_011_000.0, 3_703.0, AreaType::Urban, "Bologna Metro"),
                    PopulationRegion::new(43.7696, 11.2558, 1_013_000.0, 4_844.0, AreaType::Urban, "Florence Metro"),
                    PopulationRegion::new(45.4408, 12.3155, 853_000.0, 2_462.0, AreaType::Urban, "Venice Metro"),
                    PopulationRegion::new(41.1171, 16.8719, 608_000.0, 2_122.0, AreaType::Urban, "Bari"),
                    PopulationRegion::new(45.0555, 8.0463, 462_000.0, 1_553.0, AreaType::Urban, "Genoa"),
                    PopulationRegion::new(37.5079, 15.0830, 580_000.0, 1_570.0, AreaType::Urban, "Catania"),
                    PopulationRegion::new(38.1938, 15.5540, 423_000.0, 3_183.0, AreaType::Urban, "Messina"),
                    PopulationRegion::new(45.4064, 11.8768, 407_000.0, 2_281.0, AreaType::Urban, "Padua"),
                    PopulationRegion::new(42.0, 12.5, 13_900_000.0, 283_000.0, AreaType::Rural, "Rural Italy"),
                ], 301_340.0)
            },
            
            Self::Jamaica => {
                // Jamaica: 2.8M population, 10,991 km² area, ~56% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(17.9714, -76.7937, 1_243_000.0, 453.0, AreaType::Urban, "Kingston Metro"),
                    PopulationRegion::new(18.4709, -77.9219, 110_000.0, 25.0, AreaType::Urban, "Montego Bay"),
                    PopulationRegion::new(18.0179, -76.8099, 89_000.0, 20.0, AreaType::Urban, "Spanish Town"),
                    PopulationRegion::new(18.0, -77.0, 1_240_000.0, 10_400.0, AreaType::Rural, "Rural Jamaica"),
                ], 10_991.0)
            },
            
            Self::Japan => {
                // Japan: 125.4M population, 377,975 km² area, ~92% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(35.6762, 139.6503, 38_505_000.0, 13_452.0, AreaType::Urban, "Tokyo Metro"),
                    PopulationRegion::new(34.6937, 135.5023, 19_165_000.0, 13_033.0, AreaType::Urban, "Osaka-Kobe-Kyoto"),
                    PopulationRegion::new(35.1815, 136.9066, 9_197_000.0, 10_868.0, AreaType::Urban, "Nagoya Metro"),
                    PopulationRegion::new(43.0642, 141.3469, 2_636_000.0, 3_800.0, AreaType::Urban, "Sapporo Metro"),
                    PopulationRegion::new(33.5904, 130.4017, 5_502_000.0, 4_747.0, AreaType::Urban, "Fukuoka-Kitakyushu"),
                    PopulationRegion::new(34.3853, 132.4553, 2_096_000.0, 905.0, AreaType::Urban, "Hiroshima Metro"),
                    PopulationRegion::new(38.2682, 140.8694, 2_324_000.0, 6_400.0, AreaType::Urban, "Sendai Metro"),
                    PopulationRegion::new(35.4437, 139.6380, 1_304_000.0, 326.0, AreaType::Urban, "Kawasaki"),
                    PopulationRegion::new(35.1796, 136.9066, 1_163_000.0, 523.0, AreaType::Urban, "Kobe"),
                    PopulationRegion::new(34.9756, 138.3829, 1_143_000.0, 1_411.0, AreaType::Urban, "Shizuoka"),
                    PopulationRegion::new(26.2124, 127.6809, 1_377_000.0, 1_199.0, AreaType::Urban, "Naha"),
                    PopulationRegion::new(37.0, 138.0, 8_600_000.0, 324_000.0, AreaType::Rural, "Rural Japan"),
                ], 377_975.0)
            },
            
            Self::Jordan => {
                // Jordan: 10.3M population, 89,342 km² area, ~91% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(31.9454, 35.9284, 2_210_000.0, 696.0, AreaType::Urban, "Amman"),
                    PopulationRegion::new(31.1853, 35.7041, 655_000.0, 121.0, AreaType::Urban, "Zarqa"),
                    PopulationRegion::new(32.5568, 35.8469, 608_000.0, 176.0, AreaType::Urban, "Irbid"),
                    PopulationRegion::new(31.7148, 35.7933, 144_000.0, 35.0, AreaType::Urban, "Russeifa"),
                    PopulationRegion::new(31.0, 36.0, 920_000.0, 88_300.0, AreaType::Rural, "Rural Jordan"),
                ], 89_342.0)
            },
            
            Self::Kazakhstan => {
                // Kazakhstan: 19.0M population, 2,724,900 km² area, ~58% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(43.2551, 76.9126, 2_048_000.0, 683.0, AreaType::Urban, "Almaty"),
                    PopulationRegion::new(51.1655, 71.4272, 1_184_000.0, 810.0, AreaType::Urban, "Nur-Sultan"),
                    PopulationRegion::new(42.3151, 69.5893, 1_074_000.0, 200.0, AreaType::Urban, "Shymkent"),
                    PopulationRegion::new(50.4119, 80.2282, 510_000.0, 300.0, AreaType::Urban, "Semey"),
                    PopulationRegion::new(47.1167, 51.8833, 500_000.0, 280.0, AreaType::Urban, "Aktobe"),
                    PopulationRegion::new(49.8047, 73.1094, 497_000.0, 220.0, AreaType::Urban, "Karaganda"),
                    PopulationRegion::new(52.2878, 76.9456, 476_000.0, 195.0, AreaType::Urban, "Pavlodar"),
                    PopulationRegion::new(43.3551, 77.3205, 453_000.0, 175.0, AreaType::Urban, "Taraz"),
                    PopulationRegion::new(48.0, 68.0, 6_700_000.0, 2_722_000.0, AreaType::Rural, "Rural Kazakhstan"),
                ], 2_724_900.0)
            },
            
            Self::Kenya => {
                // Kenya: 54.0M population, 580,367 km² area, ~28% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-1.2921, 36.8219, 5_119_000.0, 696.0, AreaType::Urban, "Nairobi"),
                    PopulationRegion::new(-4.0435, 39.6682, 1_341_000.0, 294.0, AreaType::Urban, "Mombasa"),
                    PopulationRegion::new(0.5143, 35.2698, 608_000.0, 325.0, AreaType::Urban, "Nakuru"),
                    PopulationRegion::new(-1.0502, 37.0804, 500_000.0, 123.0, AreaType::Urban, "Ruiru"),
                    PopulationRegion::new(-0.0917, 34.7680, 475_000.0, 197.0, AreaType::Urban, "Kisumu"),
                    PopulationRegion::new(0.0353, 37.0733, 336_000.0, 85.0, AreaType::Urban, "Eldoret"),
                    PopulationRegion::new(-1.2837, 36.8269, 311_000.0, 75.0, AreaType::Urban, "Kikuyu"),
                    PopulationRegion::new(-0.7174, 37.1520, 260_000.0, 65.0, AreaType::Urban, "Thika"),
                    PopulationRegion::new(-1.0, 37.0, 37_000_000.0, 578_700.0, AreaType::Rural, "Rural Kenya"),
                ], 580_367.0)
            },
            
            Self::Kiribati => {
                // Kiribati: 121k population, 811 km² area, ~55% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(1.3559, 172.9847, 66_000.0, 40.0, AreaType::Urban, "South Tarawa"),
                    PopulationRegion::new(1.9808, 157.3603, 3_200.0, 8.0, AreaType::Urban, "Kiritimati"),
                    PopulationRegion::new(1.4167, 173.0167, 3_000.0, 7.0, AreaType::Urban, "Betio"),
                    PopulationRegion::new(1.3333, 172.9833, 2_800.0, 6.0, AreaType::Urban, "Bairiki"),
                    PopulationRegion::new(1.5, 173.0, 45_000.0, 750.0, AreaType::Rural, "Rural Kiribati"),
                ], 811.0)
            },
            
            Self::Kosovo => {
                // Kosovo: 1.8M population, 10,887 km² area, ~43% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(42.6629, 21.1655, 207_000.0, 83.0, AreaType::Urban, "Pristina"),
                    PopulationRegion::new(42.3702, 20.4298, 97_000.0, 32.0, AreaType::Urban, "Prizren"),
                    PopulationRegion::new(42.7441, 20.8333, 87_000.0, 35.0, AreaType::Urban, "Gjilan"),
                    PopulationRegion::new(42.5, 21.0, 1_024_000.0, 10_700.0, AreaType::Rural, "Rural Kosovo"),
                ], 10_887.0)
            },
            
            Self::Kuwait => {
                // Kuwait: 4.3M population, 17,818 km² area, ~100% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(29.3759, 47.9774, 3_115_000.0, 200.0, AreaType::Urban, "Kuwait City Metro"),
                    PopulationRegion::new(29.1458, 48.0975, 295_000.0, 37.0, AreaType::Urban, "Ahmadi"),
                    PopulationRegion::new(29.3347, 47.6581, 273_000.0, 30.0, AreaType::Urban, "Farwaniya"),
                    PopulationRegion::new(29.2, 47.8, 600_000.0, 17_551.0, AreaType::Rural, "Rural Kuwait"),
                ], 17_818.0)
            },
            
            Self::Kyrgyzstan => {
                // Kyrgyzstan: 6.7M population, 199,951 km² area, ~37% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(42.8746, 74.5698, 1_069_000.0, 127.0, AreaType::Urban, "Bishkek"),
                    PopulationRegion::new(40.5140, 72.8161, 1_064_000.0, 29.0, AreaType::Urban, "Osh"),
                    PopulationRegion::new(40.9333, 72.9833, 83_000.0, 18.0, AreaType::Urban, "Jalal-Abad"),
                    PopulationRegion::new(41.0, 74.0, 4_200_000.0, 199_700.0, AreaType::Rural, "Rural Kyrgyzstan"),
                ], 199_951.0)
            },
            
            Self::Laos => {
                // Laos: 7.4M population, 236,800 km² area, ~36% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(17.9757, 102.6331, 683_000.0, 130.0, AreaType::Urban, "Vientiane"),
                    PopulationRegion::new(15.1202, 105.7817, 141_000.0, 47.0, AreaType::Urban, "Savannakhet"),
                    PopulationRegion::new(14.8817, 105.8076, 122_000.0, 28.0, AreaType::Urban, "Pakse"),
                    PopulationRegion::new(18.0, 103.0, 4_700_000.0, 236_600.0, AreaType::Rural, "Rural Laos"),
                ], 236_800.0)
            },
            
            Self::Latvia => {
                // Latvia: 1.9M population, 64,589 km² area, ~68% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(56.9496, 24.1052, 633_000.0, 307.0, AreaType::Urban, "Riga"),
                    PopulationRegion::new(55.8753, 26.5361, 79_000.0, 58.0, AreaType::Urban, "Daugavpils"),
                    PopulationRegion::new(56.5000, 21.0167, 61_000.0, 60.0, AreaType::Urban, "Liepāja"),
                    PopulationRegion::new(57.0, 24.0, 592_000.0, 64_100.0, AreaType::Rural, "Rural Latvia"),
                ], 64_589.0)
            },
            
            Self::Lebanon => {
                // Lebanon: 6.8M population, 10,452 km² area, ~89% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(33.8938, 35.5018, 2_424_000.0, 100.0, AreaType::Urban, "Beirut Metro"),
                    PopulationRegion::new(33.5633, 35.3814, 530_000.0, 17.0, AreaType::Urban, "Tripoli"),
                    PopulationRegion::new(33.3705, 35.3709, 237_000.0, 20.0, AreaType::Urban, "Sidon"),
                    PopulationRegion::new(33.2773, 35.2033, 200_000.0, 16.0, AreaType::Urban, "Tyre"),
                    PopulationRegion::new(33.8, 35.7, 750_000.0, 10_300.0, AreaType::Rural, "Rural Lebanon"),
                ], 10_452.0)
            },
            
            Self::Lesotho => {
                // Lesotho: 2.2M population, 30,355 km² area, ~29% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-29.3100, 27.4800, 331_000.0, 138.0, AreaType::Urban, "Maseru"),
                    PopulationRegion::new(-29.1500, 27.5500, 36_000.0, 20.0, AreaType::Urban, "Teyateyaneng"),
                    PopulationRegion::new(-29.5, 28.0, 1_560_000.0, 30_200.0, AreaType::Rural, "Rural Lesotho"),
                ], 30_355.0)
            },
            
            Self::Liberia => {
                // Liberia: 5.2M population, 111,369 km² area, ~52% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(6.3106, -10.8048, 1_517_000.0, 194.0, AreaType::Urban, "Monrovia"),
                    PopulationRegion::new(6.4160, -10.7969, 56_000.0, 12.0, AreaType::Urban, "Paynesville"),
                    PopulationRegion::new(5.0119, -9.0383, 50_000.0, 10.0, AreaType::Urban, "Harper"),
                    PopulationRegion::new(6.5, -9.5, 2_500_000.0, 111_100.0, AreaType::Rural, "Rural Liberia"),
                ], 111_369.0)
            },
            
            Self::Libya => {
                // Libya: 6.9M population, 1,759,540 km² area, ~81% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(32.8872, 13.1913, 1_165_000.0, 400.0, AreaType::Urban, "Tripoli"),
                    PopulationRegion::new(32.1194, 20.0867, 807_000.0, 210.0, AreaType::Urban, "Benghazi"),
                    PopulationRegion::new(32.7571, 12.9681, 287_000.0, 63.0, AreaType::Urban, "Misrata"),
                    PopulationRegion::new(31.1992, 16.5887, 118_000.0, 55.0, AreaType::Urban, "Tarhuna"),
                    PopulationRegion::new(27.0, 17.0, 1_300_000.0, 1_758_800.0, AreaType::Rural, "Rural Libya"),
                ], 1_759_540.0)
            },
            
            Self::Liechtenstein => {
                // Liechtenstein: 39k population, 160 km² area, ~15% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(47.1410, 9.5209, 6_000.0, 17.0, AreaType::Urban, "Vaduz"),
                    PopulationRegion::new(47.1, 9.5, 33_000.0, 143.0, AreaType::Rural, "Rural Liechtenstein"),
                ], 160.0)
            },
            
            Self::Lithuania => {
                // Lithuania: 2.8M population, 65,300 km² area, ~68% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(54.6872, 25.2797, 580_000.0, 401.0, AreaType::Urban, "Vilnius"),
                    PopulationRegion::new(54.9012, 23.9036, 290_000.0, 157.0, AreaType::Urban, "Kaunas"),
                    PopulationRegion::new(55.7333, 21.1333, 149_000.0, 99.0, AreaType::Urban, "Klaipėda"),
                    PopulationRegion::new(55.0, 24.0, 880_000.0, 64_600.0, AreaType::Rural, "Rural Lithuania"),
                ], 65_300.0)
            },
            
            Self::Luxembourg => {
                // Luxembourg: 640k population, 2,586 km² area, ~91% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(49.6117, 6.1319, 128_000.0, 52.0, AreaType::Urban, "Luxembourg City"),
                    PopulationRegion::new(49.4804, 5.9860, 33_000.0, 15.0, AreaType::Urban, "Esch-sur-Alzette"),
                    PopulationRegion::new(49.6806, 5.8139, 29_000.0, 10.0, AreaType::Urban, "Differdange"),
                    PopulationRegion::new(49.7, 6.1, 60_000.0, 2_509.0, AreaType::Rural, "Rural Luxembourg"),
                ], 2_586.0)
            },
            
            Self::Macedonia => {
                // North Macedonia: 2.1M population, 25,713 km² area, ~59% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(41.9981, 21.4254, 544_000.0, 77.0, AreaType::Urban, "Skopje"),
                    PopulationRegion::new(41.0297, 21.3292, 73_000.0, 30.0, AreaType::Urban, "Bitola"),
                    PopulationRegion::new(41.7151, 22.1821, 66_000.0, 25.0, AreaType::Urban, "Kumanovo"),
                    PopulationRegion::new(41.5, 21.5, 860_000.0, 25_600.0, AreaType::Rural, "Rural North Macedonia"),
                ], 25_713.0)
            },
            
            Self::Madagascar => {
                // Madagascar: 28.4M population, 587,041 km² area, ~39% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-18.8792, 47.5079, 1_740_000.0, 93.0, AreaType::Urban, "Antananarivo"),
                    PopulationRegion::new(-23.3599, 43.6713, 316_000.0, 80.0, AreaType::Urban, "Toliara"),
                    PopulationRegion::new(-18.1443, 49.3958, 289_000.0, 45.0, AreaType::Urban, "Toamasina"),
                    PopulationRegion::new(-19.8692, 47.0338, 244_000.0, 50.0, AreaType::Urban, "Antsirabe"),
                    PopulationRegion::new(-20.0, 47.0, 17_400_000.0, 586_700.0, AreaType::Rural, "Rural Madagascar"),
                ], 587_041.0)
            },
            
            Self::Malawi => {
                // Malawi: 19.6M population, 118,484 km² area, ~17% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-13.9626, 33.7741, 1_122_000.0, 158.0, AreaType::Urban, "Lilongwe"),
                    PopulationRegion::new(-15.3875, 35.0085, 873_000.0, 184.0, AreaType::Urban, "Blantyre"),
                    PopulationRegion::new(-11.4502, 34.0191, 221_000.0, 86.0, AreaType::Urban, "Mzuzu"),
                    PopulationRegion::new(-14.0, 34.0, 16_400_000.0, 118_100.0, AreaType::Rural, "Rural Malawi"),
                ], 118_484.0)
            },
            
            Self::Malaysia => {
                // Malaysia: 32.7M population, 330,803 km² area, ~78% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(3.1390, 101.6869, 8_420_000.0, 2_793.0, AreaType::Urban, "Kuala Lumpur Metro"),
                    PopulationRegion::new(5.4141, 100.3288, 2_513_000.0, 2_562.0, AreaType::Urban, "Penang Metro"),
                    PopulationRegion::new(1.5533, 103.7594, 1_803_000.0, 1_064.0, AreaType::Urban, "Johor Bahru Metro"),
                    PopulationRegion::new(3.0733, 101.5185, 800_000.0, 290.0, AreaType::Urban, "Ipoh"),
                    PopulationRegion::new(6.1254, 102.2381, 354_000.0, 36.0, AreaType::Urban, "Kota Bharu"),
                    PopulationRegion::new(1.4655, 110.4294, 740_000.0, 1_545.0, AreaType::Urban, "Kuching"),
                    PopulationRegion::new(5.9804, 116.0735, 500_000.0, 366.0, AreaType::Urban, "Kota Kinabalu"),
                    PopulationRegion::new(3.8077, 103.3260, 497_000.0, 33.0, AreaType::Urban, "Kuantan"),
                    PopulationRegion::new(5.3593, 103.1368, 425_000.0, 28.0, AreaType::Urban, "Kota Terengganu"),
                    PopulationRegion::new(4.0, 102.0, 5_300_000.0, 324_000.0, AreaType::Rural, "Rural Malaysia"),
                ], 330_803.0)
            },
            
            Self::Maldives => {
                // Maldives: 541k population, 298 km² area, ~41% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(4.1755, 73.5093, 211_000.0, 6.0, AreaType::Urban, "Malé"),
                    PopulationRegion::new(4.1748, 73.5089, 41_000.0, 2.0, AreaType::Urban, "Hulhumalé"),
                    PopulationRegion::new(-0.6286, 73.0949, 12_000.0, 1.0, AreaType::Urban, "Addu City"),
                    PopulationRegion::new(5.0378, 73.0728, 8_000.0, 1.0, AreaType::Urban, "Kulhudhuffushi"),
                    PopulationRegion::new(-0.5816, 73.0969, 7_000.0, 1.0, AreaType::Urban, "Hithadhoo"),
                    PopulationRegion::new(4.2, 73.5, 252_000.0, 287.0, AreaType::Rural, "Rural Maldives"),
                ], 298.0)
            },
            
            Self::Mali => {
                // Mali: 21.0M population, 1,240,192 km² area, ~44% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(12.6392, -8.0029, 2_713_000.0, 267.0, AreaType::Urban, "Bamako"),
                    PopulationRegion::new(12.6570, -5.9957, 375_000.0, 87.0, AreaType::Urban, "Sikasso"),
                    PopulationRegion::new(13.8317, -4.8896, 283_000.0, 65.0, AreaType::Urban, "Segou"),
                    PopulationRegion::new(16.7735, -3.0097, 237_000.0, 56.0, AreaType::Urban, "Timbuktu"),
                    PopulationRegion::new(17.0, -4.0, 11_700_000.0, 1_239_700.0, AreaType::Rural, "Rural Mali"),
                ], 1_240_192.0)
            },
            
            Self::Malta => {
                // Malta: 519k population, 316 km² area, ~95% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(35.8989, 14.5146, 213_000.0, 17.0, AreaType::Urban, "Valletta Metro"),
                    PopulationRegion::new(35.8857, 14.4024, 30_000.0, 5.0, AreaType::Urban, "Birkirkara"),
                    PopulationRegion::new(35.9147, 14.4894, 25_000.0, 3.0, AreaType::Urban, "Qormi"),
                    PopulationRegion::new(35.9, 14.5, 26_000.0, 291.0, AreaType::Rural, "Rural Malta"),
                ], 316.0)
            },
            
            Self::MarshallIslands => {
                // Marshall Islands: 60k population, 181 km² area, ~78% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(7.0897, 171.3803, 28_000.0, 10.0, AreaType::Urban, "Majuro"),
                    PopulationRegion::new(8.7208, 167.7314, 12_000.0, 5.0, AreaType::Urban, "Ebeye"),
                    PopulationRegion::new(7.1315, 171.3723, 3_000.0, 3.0, AreaType::Urban, "Delap-Uliga-Djarrit"),
                    PopulationRegion::new(7.5, 171.0, 10_000.0, 163.0, AreaType::Rural, "Rural Marshall Islands"),
                ], 181.0)
            },
            
            Self::Mauritania => {
                // Mauritania: 4.8M population, 1,030,700 km² area, ~55% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(18.0858, -15.9785, 1_315_000.0, 1_000.0, AreaType::Urban, "Nouakchott"),
                    PopulationRegion::new(18.1384, -15.8031, 158_000.0, 40.0, AreaType::Urban, "Nouadhibou"),
                    PopulationRegion::new(16.1712, -13.8267, 66_000.0, 20.0, AreaType::Urban, "Rosso"),
                    PopulationRegion::new(20.0, -12.0, 2_160_000.0, 1_029_600.0, AreaType::Rural, "Rural Mauritania"),
                ], 1_030_700.0)
            },
            
            Self::Mauritius => {
                // Mauritius: 1.3M population, 2,040 km² area, ~41% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-20.1625, 57.4989, 147_000.0, 48.0, AreaType::Urban, "Port Louis"),
                    PopulationRegion::new(-20.2611, 57.4781, 110_000.0, 25.0, AreaType::Urban, "Beau Bassin-Rose Hill"),
                    PopulationRegion::new(-20.3203, 57.5156, 82_000.0, 20.0, AreaType::Urban, "Vacoas-Phoenix"),
                    PopulationRegion::new(-20.2, 57.5, 765_000.0, 1_947.0, AreaType::Rural, "Rural Mauritius"),
                ], 2_040.0)
            },
            
            Self::Mexico => {
                // Mexico: 128.9M population, 1,964,375 km² area, ~81% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(19.4326, -99.1332, 21_919_000.0, 7_866.0, AreaType::Urban, "Mexico City Metro"),
                    PopulationRegion::new(20.6597, -103.3496, 5_269_000.0, 2_735.0, AreaType::Urban, "Guadalajara Metro"),
                    PopulationRegion::new(25.6866, -100.3161, 5_341_000.0, 7_431.0, AreaType::Urban, "Monterrey Metro"),
                    PopulationRegion::new(19.0414, -98.2063, 3_251_000.0, 2_393.0, AreaType::Urban, "Puebla Metro"),
                    PopulationRegion::new(21.0251, -101.2545, 2_058_000.0, 2_042.0, AreaType::Urban, "León Metro"),
                    PopulationRegion::new(32.5149, -117.0382, 2_157_000.0, 1_392.0, AreaType::Urban, "Tijuana"),
                    PopulationRegion::new(31.6904, -106.4245, 1_513_000.0, 4_853.0, AreaType::Urban, "Ciudad Juárez"),
                    PopulationRegion::new(19.1738, -96.1342, 1_046_000.0, 475.0, AreaType::Urban, "Veracruz"),
                    PopulationRegion::new(20.5880, -100.3899, 1_385_000.0, 1_165.0, AreaType::Urban, "Querétaro"),
                    PopulationRegion::new(21.0190, -89.6237, 1_316_000.0, 348.0, AreaType::Urban, "Mérida"),
                    PopulationRegion::new(20.6953, -101.9539, 1_266_000.0, 324.0, AreaType::Urban, "Aguascalientes"),
                    PopulationRegion::new(23.0, -102.0, 19_900_000.0, 1_940_000.0, AreaType::Rural, "Rural Mexico"),
                ], 1_964_375.0)
            },
            
            Self::Moldova => {
                // Moldova: 2.6M population, 33,846 km² area, ~43% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(47.0105, 28.8638, 695_000.0, 120.0, AreaType::Urban, "Chișinău"),
                    PopulationRegion::new(46.8419, 29.6139, 90_000.0, 25.0, AreaType::Urban, "Tiraspol"),
                    PopulationRegion::new(47.7571, 27.9237, 81_000.0, 20.0, AreaType::Urban, "Bălți"),
                    PopulationRegion::new(47.5, 28.5, 1_490_000.0, 33_600.0, AreaType::Rural, "Rural Moldova"),
                ], 33_846.0)
            },
            
            Self::Monaco => {
                // Monaco: 39k population, 2 km² area, ~100% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(43.7384, 7.4246, 39_000.0, 2.0, AreaType::Urban, "Monaco"),
                ], 2.0)
            },
            
            Self::Mongolia => {
                // Mongolia: 3.4M population, 1,564,110 km² area, ~69% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(47.8864, 106.9057, 1_600_000.0, 4_704.0, AreaType::Urban, "Ulaanbaatar"),
                    PopulationRegion::new(48.9674, 89.5824, 89_000.0, 50.0, AreaType::Urban, "Ölgii"),
                    PopulationRegion::new(49.6333, 100.1500, 87_000.0, 45.0, AreaType::Urban, "Erdenet"),
                    PopulationRegion::new(48.0, 103.0, 1_050_000.0, 1_559_300.0, AreaType::Rural, "Rural Mongolia"),
                ], 1_564_110.0)
            },
            
            Self::Montenegro => {
                // Montenegro: 621k population, 13,812 km² area, ~68% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(42.4304, 19.2594, 188_000.0, 1_441.0, AreaType::Urban, "Podgorica"),
                    PopulationRegion::new(43.1238, 19.0975, 74_000.0, 24.0, AreaType::Urban, "Nikšić"),
                    PopulationRegion::new(42.7731, 18.9446, 20_000.0, 12.0, AreaType::Urban, "Herceg Novi"),
                    PopulationRegion::new(42.5, 19.0, 199_000.0, 12_300.0, AreaType::Rural, "Rural Montenegro"),
                ], 13_812.0)
            },
            
            Self::Morocco => {
                // Morocco: 37.5M population, 446,550 km² area, ~64% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(33.5731, -7.5898, 4_079_000.0, 230.0, AreaType::Urban, "Casablanca"),
                    PopulationRegion::new(34.0209, -6.8416, 1_273_000.0, 118.0, AreaType::Urban, "Rabat"),
                    PopulationRegion::new(31.6295, -7.9811, 1_137_000.0, 230.0, AreaType::Urban, "Marrakesh"),
                    PopulationRegion::new(35.7595, -5.8340, 1_034_000.0, 119.0, AreaType::Urban, "Tangier"),
                    PopulationRegion::new(34.0333, -5.0000, 966_000.0, 424.0, AreaType::Urban, "Fez"),
                    PopulationRegion::new(33.5883, -7.6114, 893_000.0, 220.0, AreaType::Urban, "Salé"),
                    PopulationRegion::new(32.0, -6.0, 13_500_000.0, 445_400.0, AreaType::Rural, "Rural Morocco"),
                ], 446_550.0)
            },
            
            Self::Mozambique => {
                // Mozambique: 32.2M population, 801,590 km² area, ~37% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-25.9692, 32.5732, 1_136_000.0, 347.0, AreaType::Urban, "Maputo"),
                    PopulationRegion::new(-25.9167, 32.5667, 1_126_000.0, 300.0, AreaType::Urban, "Matola"),
                    PopulationRegion::new(-19.8436, 34.8389, 616_000.0, 230.0, AreaType::Urban, "Beira"),
                    PopulationRegion::new(-15.1167, 39.2667, 351_000.0, 66.0, AreaType::Urban, "Nampula"),
                    PopulationRegion::new(-18.0, 35.0, 20_300_000.0, 800_600.0, AreaType::Rural, "Rural Mozambique"),
                ], 801_590.0)
            },
            
            Self::Myanmar => {
                // Myanmar: 54.8M population, 676,578 km² area, ~31% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(16.8661, 96.1951, 5_610_000.0, 610.0, AreaType::Urban, "Yangon"),
                    PopulationRegion::new(21.9588, 96.0891, 1_440_000.0, 156.0, AreaType::Urban, "Mandalay"),
                    PopulationRegion::new(19.7463, 96.1609, 491_000.0, 125.0, AreaType::Urban, "Naypyidaw"),
                    PopulationRegion::new(16.4833, 97.6167, 181_000.0, 30.0, AreaType::Urban, "Mawlamyine"),
                    PopulationRegion::new(21.0, 96.0, 37_800_000.0, 675_600.0, AreaType::Rural, "Rural Myanmar"),
                ], 676_578.0)
            },
            
            Self::Namibia => {
                // Namibia: 2.5M population, 825,615 km² area, ~52% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-22.5594, 17.0832, 431_000.0, 5_133.0, AreaType::Urban, "Windhoek"),
                    PopulationRegion::new(-22.6700, 14.5269, 107_000.0, 29.0, AreaType::Urban, "Swakopmund"),
                    PopulationRegion::new(-22.9576, 14.5046, 107_000.0, 29.0, AreaType::Urban, "Walvis Bay"),
                    PopulationRegion::new(-22.0, 17.0, 1_200_000.0, 820_400.0, AreaType::Rural, "Rural Namibia"),
                ], 825_615.0)
            },
            
            Self::Nauru => {
                // Nauru: 11k population, 21 km² area, ~100% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-0.5477, 166.9209, 11_000.0, 21.0, AreaType::Urban, "Yaren District"),
                ], 21.0)
            },
            
            Self::Nepal => {
                // Nepal: 30.2M population, 147,516 km² area, ~21% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(27.7172, 85.3240, 1_330_000.0, 50.0, AreaType::Urban, "Kathmandu"),
                    PopulationRegion::new(26.4525, 87.2718, 196_000.0, 30.0, AreaType::Urban, "Biratnagar"),
                    PopulationRegion::new(28.2380, 83.9956, 384_000.0, 70.0, AreaType::Urban, "Pokhara"),
                    PopulationRegion::new(27.7, 85.3, 23_900_000.0, 147_300.0, AreaType::Rural, "Rural Nepal"),
                ], 147_516.0)
            },
            
            Self::Netherlands => {
                // Netherlands: 17.5M population, 41,850 km² area, ~92% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(52.3676, 4.9041, 2_480_000.0, 2_580.0, AreaType::Urban, "Amsterdam Metro"),
                    PopulationRegion::new(51.9244, 4.4777, 1_024_000.0, 324.0, AreaType::Urban, "Rotterdam"),
                    PopulationRegion::new(52.0907, 5.1214, 906_000.0, 1_449.0, AreaType::Urban, "Utrecht"),
                    PopulationRegion::new(52.1601, 4.4970, 1_404_000.0, 1_024.0, AreaType::Urban, "The Hague Metro"),
                    PopulationRegion::new(51.4416, 5.4697, 786_000.0, 332.0, AreaType::Urban, "Eindhoven Metro"),
                    PopulationRegion::new(51.5719, 4.7683, 218_000.0, 129.0, AreaType::Urban, "Breda"),
                    PopulationRegion::new(53.2194, 6.5665, 233_000.0, 83.0, AreaType::Urban, "Groningen"),
                    PopulationRegion::new(52.0, 5.0, 1_400_000.0, 35_900.0, AreaType::Rural, "Rural Netherlands"),
                ], 41_850.0)
            },
            
            Self::NewZealand => {
                // New Zealand: 5.1M population, 268,021 km² area, ~87% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-36.8485, 174.7633, 1_658_000.0, 5_600.0, AreaType::Urban, "Auckland Metro"),
                    PopulationRegion::new(-41.2865, 174.7762, 543_000.0, 1_390.0, AreaType::Urban, "Wellington Metro"),
                    PopulationRegion::new(-43.5321, 172.6362, 521_000.0, 1_426.0, AreaType::Urban, "Christchurch Metro"),
                    PopulationRegion::new(-37.7870, 175.2793, 242_000.0, 1_115.0, AreaType::Urban, "Hamilton"),
                    PopulationRegion::new(-38.1368, 176.2497, 151_000.0, 168.0, AreaType::Urban, "Tauranga"),
                    PopulationRegion::new(-45.8788, 170.5028, 134_000.0, 256.0, AreaType::Urban, "Dunedin"),
                    PopulationRegion::new(-39.0556, 174.0752, 83_000.0, 2_062.0, AreaType::Urban, "New Plymouth"),
                    PopulationRegion::new(-39.4928, 176.9120, 81_000.0, 156.0, AreaType::Urban, "Napier-Hastings"),
                    PopulationRegion::new(-38.6857, 178.0236, 79_000.0, 94.0, AreaType::Urban, "Gisborne"),
                    PopulationRegion::new(-46.4132, 168.3538, 56_000.0, 3_314.0, AreaType::Urban, "Invercargill"),
                    PopulationRegion::new(-42.0, 172.0, 350_000.0, 258_600.0, AreaType::Rural, "Rural New Zealand"),
                ], 268_021.0)
            },
            
            Self::Nicaragua => {
                // Nicaragua: 6.7M population, 130,373 km² area, ~59% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(12.1364, -86.2514, 1_080_000.0, 544.0, AreaType::Urban, "Managua"),
                    PopulationRegion::new(12.4340, -86.8779, 203_000.0, 44.0, AreaType::Urban, "León"),
                    PopulationRegion::new(11.9727, -86.0948, 161_000.0, 38.0, AreaType::Urban, "Masaya"),
                    PopulationRegion::new(12.0, -85.0, 2_750_000.0, 129_700.0, AreaType::Rural, "Rural Nicaragua"),
                ], 130_373.0)
            },
            
            Self::Niger => {
                // Niger: 25.1M population, 1,267,000 km² area, ~17% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(13.5137, 2.1098, 1_336_000.0, 255.0, AreaType::Urban, "Niamey"),
                    PopulationRegion::new(13.8001, 8.9816, 123_000.0, 30.0, AreaType::Urban, "Zinder"),
                    PopulationRegion::new(13.0499, 7.6088, 118_000.0, 25.0, AreaType::Urban, "Maradi"),
                    PopulationRegion::new(17.0, 8.0, 21_000_000.0, 1_266_700.0, AreaType::Rural, "Rural Niger"),
                ], 1_267_000.0)
            },
            
            Self::Nigeria => {
                // Nigeria: 213.4M population, 923,768 km² area, ~52% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(6.5244, 3.3792, 15_946_000.0, 1_171.0, AreaType::Urban, "Lagos"),
                    PopulationRegion::new(12.0022, 8.5919, 3_840_000.0, 476.0, AreaType::Urban, "Kano"),
                    PopulationRegion::new(7.3775, 3.9069, 3_649_000.0, 1_204.0, AreaType::Urban, "Ibadan"),
                    PopulationRegion::new(9.0765, 7.3986, 3_278_000.0, 1_069.0, AreaType::Urban, "Abuja"),
                    PopulationRegion::new(6.3350, 5.6037, 1_874_000.0, 369.0, AreaType::Urban, "Benin City"),
                    PopulationRegion::new(4.8156, 7.0498, 1_865_000.0, 360.0, AreaType::Urban, "Port Harcourt"),
                    PopulationRegion::new(11.8333, 13.1500, 1_336_000.0, 194.0, AreaType::Urban, "Maiduguri"),
                    PopulationRegion::new(6.1704, 6.7410, 1_049_000.0, 228.0, AreaType::Urban, "Warri"),
                    PopulationRegion::new(11.0992, 7.6508, 950_000.0, 216.0, AreaType::Urban, "Kaduna"),
                    PopulationRegion::new(5.1053, 7.3646, 918_000.0, 103.0, AreaType::Urban, "Aba"),
                    PopulationRegion::new(10.0, 8.0, 99_000_000.0, 920_000.0, AreaType::Rural, "Rural Nigeria"),
                ], 923_768.0)
            },
            
            Self::NorthKorea => {
                // North Korea: 25.9M population, 120,540 km² area, ~62% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(39.0194, 125.7381, 3_255_000.0, 2_113.0, AreaType::Urban, "Pyongyang"),
                    PopulationRegion::new(41.8010, 129.7857, 667_000.0, 155.0, AreaType::Urban, "Chongjin"),
                    PopulationRegion::new(38.5072, 125.6897, 555_000.0, 132.0, AreaType::Urban, "Nampo"),
                    PopulationRegion::new(39.9181, 127.5364, 530_000.0, 110.0, AreaType::Urban, "Hamhung"),
                    PopulationRegion::new(40.0, 127.0, 9_800_000.0, 118_000.0, AreaType::Rural, "Rural North Korea"),
                ], 120_540.0)
            },
            
            Self::Norway => {
                // Norway: 5.4M population, 323,802 km² area, ~83% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(59.9139, 10.7522, 1_071_000.0, 454.0, AreaType::Urban, "Oslo Metro"),
                    PopulationRegion::new(60.3913, 5.3221, 420_000.0, 445.0, AreaType::Urban, "Bergen Metro"),
                    PopulationRegion::new(63.4305, 10.3951, 280_000.0, 321.0, AreaType::Urban, "Trondheim Metro"),
                    PopulationRegion::new(58.9700, 5.7331, 225_000.0, 71.0, AreaType::Urban, "Stavanger Metro"),
                    PopulationRegion::new(68.4384, 17.4272, 86_000.0, 103.0, AreaType::Urban, "Bodø"),
                    PopulationRegion::new(67.2803, 14.4049, 83_000.0, 98.0, AreaType::Urban, "Narvik"),
                    PopulationRegion::new(69.6492, 18.9553, 77_000.0, 2_521.0, AreaType::Urban, "Tromsø"),
                    PopulationRegion::new(62.0, 9.0, 670_000.0, 322_500.0, AreaType::Rural, "Rural Norway"),
                ], 323_802.0)
            },
            
            Self::Oman => {
                // Oman: 5.2M population, 309,500 km² area, ~87% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(23.5880, 58.3829, 1_590_000.0, 3_500.0, AreaType::Urban, "Muscat Metro"),
                    PopulationRegion::new(17.0187, 54.0924, 318_000.0, 77.0, AreaType::Urban, "Salalah"),
                    PopulationRegion::new(23.6739, 58.1893, 267_000.0, 55.0, AreaType::Urban, "Seeb"),
                    PopulationRegion::new(22.9333, 57.5333, 209_000.0, 45.0, AreaType::Urban, "Nizwa"),
                    PopulationRegion::new(21.0, 57.0, 680_000.0, 305_800.0, AreaType::Rural, "Rural Oman"),
                ], 309_500.0)
            },
            
            Self::Pakistan => {
                // Pakistan: 225.2M population, 881,913 km² area, ~37% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(24.8607, 67.0011, 14_916_000.0, 3_780.0, AreaType::Urban, "Karachi"),
                    PopulationRegion::new(31.5497, 74.3436, 11_738_000.0, 1_772.0, AreaType::Urban, "Lahore"),
                    PopulationRegion::new(33.6844, 73.0479, 2_301_000.0, 906.0, AreaType::Urban, "Islamabad-Rawalpindi"),
                    PopulationRegion::new(30.1984, 71.4687, 2_065_000.0, 348.0, AreaType::Urban, "Multan"),
                    PopulationRegion::new(31.4180, 73.0790, 1_977_000.0, 370.0, AreaType::Urban, "Faisalabad"),
                    PopulationRegion::new(25.3960, 68.3578, 1_677_000.0, 168.0, AreaType::Urban, "Hyderabad"),
                    PopulationRegion::new(34.0150, 71.5805, 1_594_000.0, 92.0, AreaType::Urban, "Peshawar"),
                    PopulationRegion::new(32.0740, 72.6861, 1_388_000.0, 122.0, AreaType::Urban, "Jhang"),
                    PopulationRegion::new(29.3956, 71.6836, 1_217_000.0, 108.0, AreaType::Urban, "Bahawalpur"),
                    PopulationRegion::new(32.1877, 74.1945, 1_197_000.0, 103.0, AreaType::Urban, "Gujranwala"),
                    PopulationRegion::new(31.4504, 74.5366, 1_165_000.0, 97.0, AreaType::Urban, "Gujrat"),
                    PopulationRegion::new(30.8138, 73.4534, 1_027_000.0, 83.0, AreaType::Urban, "Sahiwal"),
                    PopulationRegion::new(30.0, 70.0, 130_000_000.0, 876_500.0, AreaType::Rural, "Rural Pakistan"),
                ], 881_913.0)
            },
            
            Self::Palau => {
                // Palau: 18k population, 459 km² area, ~81% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(7.3419, 134.4794, 11_000.0, 18.0, AreaType::Urban, "Koror"),
                    PopulationRegion::new(7.5036, 134.6292, 2_500.0, 6.0, AreaType::Urban, "Airai"),
                    PopulationRegion::new(7.5, 134.5, 500.0, 435.0, AreaType::Rural, "Rural Palau"),
                ], 459.0)
            },
            
            Self::Panama => {
                // Panama: 4.4M population, 75,417 km² area, ~68% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(8.9824, -79.5199, 1_938_000.0, 2_561.0, AreaType::Urban, "Panama City Metro"),
                    PopulationRegion::new(8.4333, -82.4333, 139_000.0, 30.0, AreaType::Urban, "David"),
                    PopulationRegion::new(8.0975, -80.9826, 48_000.0, 12.0, AreaType::Urban, "Santiago"),
                    PopulationRegion::new(9.0, -79.5, 1_410_000.0, 72_800.0, AreaType::Rural, "Rural Panama"),
                ], 75_417.0)
            },
            
            Self::PapuaNewGuinea => {
                // Papua New Guinea: 9.1M population, 462,840 km² area, ~13% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-9.4438, 147.1802, 383_000.0, 240.0, AreaType::Urban, "Port Moresby"),
                    PopulationRegion::new(-6.7333, 147.0000, 77_000.0, 50.0, AreaType::Urban, "Lae"),
                    PopulationRegion::new(-5.5600, 150.2600, 39_000.0, 20.0, AreaType::Urban, "Arawa"),
                    PopulationRegion::new(-6.0, 147.0, 7_900_000.0, 462_500.0, AreaType::Rural, "Rural PNG"),
                ], 462_840.0)
            },
            
            Self::Paraguay => {
                // Paraguay: 7.2M population, 406,752 km² area, ~62% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-25.2637, -57.5759, 3_452_000.0, 2_582.0, AreaType::Urban, "Asunción Metro"),
                    PopulationRegion::new(-25.5084, -54.6111, 301_000.0, 67.0, AreaType::Urban, "Ciudad del Este"),
                    PopulationRegion::new(-27.3334, -55.8667, 91_000.0, 25.0, AreaType::Urban, "Encarnación"),
                    PopulationRegion::new(-23.0, -57.0, 2_700_000.0, 404_100.0, AreaType::Rural, "Rural Paraguay"),
                ], 406_752.0)
            },
            
            Self::Peru => {
                // Peru: 33.4M population, 1,285,216 km² area, ~79% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-12.0464, -77.0428, 11_046_000.0, 2_672.0, AreaType::Urban, "Lima Metro"),
                    PopulationRegion::new(-16.4090, -71.5375, 1_129_000.0, 2_923.0, AreaType::Urban, "Arequipa"),
                    PopulationRegion::new(-8.1116, -79.0288, 975_000.0, 692.0, AreaType::Urban, "Trujillo"),
                    PopulationRegion::new(-6.7701, -79.8223, 763_000.0, 192.0, AreaType::Urban, "Chiclayo"),
                    PopulationRegion::new(-12.0266, -75.2041, 563_000.0, 494.0, AreaType::Urban, "Huancayo"),
                    PopulationRegion::new(-3.7437, -73.2516, 498_000.0, 368.0, AreaType::Urban, "Iquitos"),
                    PopulationRegion::new(-5.1783, -80.6466, 484_000.0, 277.0, AreaType::Urban, "Piura"),
                    PopulationRegion::new(-13.5226, -71.9673, 480_000.0, 385.0, AreaType::Urban, "Cusco"),
                    PopulationRegion::new(-10.0, -75.0, 5_500_000.0, 1_280_000.0, AreaType::Rural, "Rural Peru"),
                ], 1_285_216.0)
            },
            
            Self::Philippines => {
                // Philippines: 111.0M population, 300,000 km² area, ~48% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(14.5995, 120.9842, 13_923_000.0, 619.0, AreaType::Urban, "Manila Metro"),
                    PopulationRegion::new(7.0731, 125.6128, 1_776_000.0, 2_444.0, AreaType::Urban, "Davao"),
                    PopulationRegion::new(10.3157, 123.8854, 1_013_000.0, 315.0, AreaType::Urban, "Cebu City"),
                    PopulationRegion::new(6.9214, 122.0790, 457_000.0, 133.0, AreaType::Urban, "Zamboanga City"),
                    PopulationRegion::new(7.8010, 123.4132, 439_000.0, 40.0, AreaType::Urban, "Cagayan de Oro"),
                    PopulationRegion::new(10.6969, 122.5605, 433_000.0, 124.0, AreaType::Urban, "Bacolod"),
                    PopulationRegion::new(8.4822, 124.6472, 354_000.0, 163.0, AreaType::Urban, "Butuan"),
                    PopulationRegion::new(14.5547, 121.0244, 354_000.0, 103.0, AreaType::Urban, "Caloocan"),
                    PopulationRegion::new(10.7202, 122.5621, 511_000.0, 279.0, AreaType::Urban, "Iloilo City"),
                    PopulationRegion::new(11.2472, 125.0006, 188_000.0, 197.0, AreaType::Urban, "Tacloban"),
                    PopulationRegion::new(13.1391, 123.7358, 182_000.0, 107.0, AreaType::Urban, "Legazpi"),
                    PopulationRegion::new(9.6500, 123.8500, 172_000.0, 80.0, AreaType::Urban, "Tagbilaran"),
                    PopulationRegion::new(16.4023, 120.5960, 170_000.0, 77.0, AreaType::Urban, "Baguio"),
                    PopulationRegion::new(12.0, 122.0, 55_200_000.0, 296_400.0, AreaType::Rural, "Rural Philippines"),
                ], 300_000.0)
            },
            
            Self::Poland => {
                // Poland: 37.8M population, 312,696 km² area, ~60% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(52.2297, 21.0122, 3_153_000.0, 517.0, AreaType::Urban, "Warsaw Metro"),
                    PopulationRegion::new(50.0647, 19.9450, 1_783_000.0, 327.0, AreaType::Urban, "Kraków Metro"),
                    PopulationRegion::new(51.7592, 19.4560, 1_095_000.0, 293.0, AreaType::Urban, "Łódź Metro"),
                    PopulationRegion::new(51.1079, 17.0385, 1_080_000.0, 293.0, AreaType::Urban, "Wrocław Metro"),
                    PopulationRegion::new(52.4064, 16.9252, 1_005_000.0, 262.0, AreaType::Urban, "Poznań Metro"),
                    PopulationRegion::new(50.2649, 19.0238, 993_000.0, 164.0, AreaType::Urban, "Katowice Metro"),
                    PopulationRegion::new(54.3520, 18.6466, 972_000.0, 416.0, AreaType::Urban, "Gdańsk Metro"),
                    PopulationRegion::new(52.0, 19.0, 15_100_000.0, 310_400.0, AreaType::Rural, "Rural Poland"),
                ], 312_696.0)
            },
            
            Self::Portugal => {
                // Portugal: 10.3M population, 92,090 km² area, ~66% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(38.7223, -9.1393, 2_957_000.0, 2_962.0, AreaType::Urban, "Lisbon Metro"),
                    PopulationRegion::new(41.1579, -8.6291, 2_489_000.0, 2_395.0, AreaType::Urban, "Porto Metro"),
                    PopulationRegion::new(40.2033, -8.4103, 235_000.0, 319.0, AreaType::Urban, "Coimbra"),
                    PopulationRegion::new(41.5454, -8.4265, 181_000.0, 183.0, AreaType::Urban, "Braga"),
                    PopulationRegion::new(37.0194, -7.9304, 150_000.0, 607.0, AreaType::Urban, "Faro"),
                    PopulationRegion::new(32.6333, -16.9000, 112_000.0, 828.0, AreaType::Urban, "Funchal"),
                    PopulationRegion::new(38.5667, -7.9000, 110_000.0, 314.0, AreaType::Urban, "Évora"),
                    PopulationRegion::new(41.5333, -8.6167, 101_000.0, 183.0, AreaType::Urban, "Viana do Castelo"),
                    PopulationRegion::new(40.0, -8.0, 3_100_000.0, 86_200.0, AreaType::Rural, "Rural Portugal"),
                ], 92_090.0)
            },
            
            Self::Qatar => {
                // Qatar: 2.8M population, 11,586 km² area, ~99% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(25.2854, 51.5310, 1_514_000.0, 132.0, AreaType::Urban, "Doha"),
                    PopulationRegion::new(25.3548, 51.2496, 765_000.0, 90.0, AreaType::Urban, "Al Rayyan"),
                    PopulationRegion::new(25.3862, 51.6017, 121_000.0, 25.0, AreaType::Urban, "Umm Salal"),
                    PopulationRegion::new(25.4106, 51.4930, 117_000.0, 20.0, AreaType::Urban, "Al Daayen"),
                    PopulationRegion::new(25.3, 51.2, 30_000.0, 11_300.0, AreaType::Rural, "Rural Qatar"),
                ], 11_586.0)
            },
            
            Self::RepublicOfTheCongo => {
                // Republic of the Congo: 5.7M population, 342,000 km² area, ~68% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-4.2634, 15.2429, 2_388_000.0, 1_821.0, AreaType::Urban, "Brazzaville"),
                    PopulationRegion::new(-4.7761, 11.8635, 1_214_000.0, 79.0, AreaType::Urban, "Pointe-Noire"),
                    PopulationRegion::new(-4.2097, 12.6547, 58_000.0, 20.0, AreaType::Urban, "Dolisie"),
                    PopulationRegion::new(-2.0, 15.0, 1_820_000.0, 340_100.0, AreaType::Rural, "Rural Congo"),
                ], 342_000.0)
            },
            
            Self::Romania => {
                // Romania: 19.0M population, 238,397 km² area, ~54% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(44.4268, 26.1025, 2_155_000.0, 228.0, AreaType::Urban, "Bucharest"),
                    PopulationRegion::new(46.7712, 23.6236, 325_000.0, 179.0, AreaType::Urban, "Cluj-Napoca"),
                    PopulationRegion::new(47.1585, 27.6014, 318_000.0, 94.0, AreaType::Urban, "Iași"),
                    PopulationRegion::new(45.7494, 21.2272, 318_000.0, 138.0, AreaType::Urban, "Timișoara"),
                    PopulationRegion::new(44.1810, 28.6348, 283_000.0, 231.0, AreaType::Urban, "Constanța"),
                    PopulationRegion::new(46.0, 25.0, 8_750_000.0, 237_300.0, AreaType::Rural, "Rural Romania"),
                ], 238_397.0)
            },
            
            Self::Russia => {
                // Russia: 143.4M population, 17,098,242 km² area, ~75% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(55.7558, 37.6173, 12_641_000.0, 2_561.0, AreaType::Urban, "Moscow"),
                    PopulationRegion::new(59.9311, 30.3609, 5_384_000.0, 1_439.0, AreaType::Urban, "St. Petersburg"),
                    PopulationRegion::new(55.0084, 82.9357, 1_625_000.0, 505.0, AreaType::Urban, "Novosibirsk"),
                    PopulationRegion::new(56.8389, 60.6057, 1_494_000.0, 468.0, AreaType::Urban, "Yekaterinburg"),
                    PopulationRegion::new(56.0269, 92.8672, 1_094_000.0, 354.0, AreaType::Urban, "Krasnoyarsk"),
                    PopulationRegion::new(53.2001, 50.1500, 1_165_000.0, 541.0, AreaType::Urban, "Samara"),
                    PopulationRegion::new(58.0105, 56.2502, 1_056_000.0, 799.0, AreaType::Urban, "Perm"),
                    PopulationRegion::new(55.3333, 86.0833, 1_075_000.0, 293.0, AreaType::Urban, "Novokuznetsk"),
                    PopulationRegion::new(55.7540, 48.7439, 1_255_000.0, 425.0, AreaType::Urban, "Kazan"),
                    PopulationRegion::new(52.0300, 113.5000, 1_054_000.0, 500.0, AreaType::Urban, "Chita"),
                    PopulationRegion::new(54.7388, 55.9721, 1_126_000.0, 479.0, AreaType::Urban, "Ufa"),
                    PopulationRegion::new(53.1958, 45.0183, 1_019_000.0, 353.0, AreaType::Urban, "Volgograd"),
                    PopulationRegion::new(47.2357, 39.7015, 1_119_000.0, 348.0, AreaType::Urban, "Rostov-on-Don"),
                    PopulationRegion::new(55.0415, 73.3686, 1_174_000.0, 573.0, AreaType::Urban, "Omsk"),
                    PopulationRegion::new(43.1056, 131.8735, 606_000.0, 325.0, AreaType::Urban, "Vladivostok"),
                    PopulationRegion::new(60.0, 90.0, 29_700_000.0, 17_090_000.0, AreaType::Rural, "Rural Russia"),
                ], 17_098_242.0)
            },
            
            Self::Rwanda => {
                // Rwanda: 13.3M population, 26,338 km² area, ~17% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-1.9403, 30.0587, 1_204_000.0, 730.0, AreaType::Urban, "Kigali"),
                    PopulationRegion::new(-2.4973, 28.9075, 66_000.0, 20.0, AreaType::Urban, "Gisenyi"),
                    PopulationRegion::new(-1.6965, 29.2394, 63_000.0, 18.0, AreaType::Urban, "Ruhengeri"),
                    PopulationRegion::new(-2.0, 29.5, 11_000_000.0, 25_600.0, AreaType::Rural, "Rural Rwanda"),
                ], 26_338.0)
            },
            
            Self::SaintKittsAndNevis => {
                // Saint Kitts and Nevis: 54k population, 261 km² area, ~31% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(17.3026, -62.7177, 13_000.0, 7.0, AreaType::Urban, "Basseterre"),
                    PopulationRegion::new(17.3578, -62.7822, 2_500.0, 3.0, AreaType::Urban, "Sandy Point Town"),
                    PopulationRegion::new(17.1408, -62.6228, 1_800.0, 2.0, AreaType::Urban, "Charlestown"),
                    PopulationRegion::new(17.3, -62.7, 32_700.0, 249.0, AreaType::Rural, "Rural St Kitts and Nevis"),
                ], 261.0)
            },
            
            Self::SaintLucia => {
                // Saint Lucia: 185k population, 617 km² area, ~19% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(14.0101, -60.9875, 22_000.0, 11.0, AreaType::Urban, "Castries"),
                    PopulationRegion::new(13.9740, -61.0242, 6_000.0, 5.0, AreaType::Urban, "Gros Islet"),
                    PopulationRegion::new(13.8412, -60.9605, 5_000.0, 4.0, AreaType::Urban, "Vieux Fort"),
                    PopulationRegion::new(13.9, -61.0, 138_000.0, 597.0, AreaType::Rural, "Rural Saint Lucia"),
                ], 617.0)
            },
            
            Self::SaintVincentAndTheGrenadines => {
                // Saint Vincent and the Grenadines: 110k population, 389 km² area, ~53% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(13.1600, -61.2248, 27_000.0, 13.0, AreaType::Urban, "Kingstown"),
                    PopulationRegion::new(13.0031, -61.2397, 1_200.0, 2.0, AreaType::Urban, "Port Elizabeth"),
                    PopulationRegion::new(13.1526, -61.1964, 1_000.0, 1.0, AreaType::Urban, "Georgetown"),
                    PopulationRegion::new(13.1, -61.2, 49_800.0, 373.0, AreaType::Rural, "Rural St Vincent"),
                ], 389.0)
            },
            
            Self::Samoa => {
                // Samoa: 200k population, 2,842 km² area, ~18% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-13.8314, -171.7518, 36_000.0, 60.0, AreaType::Urban, "Apia"),
                    PopulationRegion::new(-13.8, -172.0, 164_000.0, 2_782.0, AreaType::Rural, "Rural Samoa"),
                ], 2_842.0)
            },
            
            Self::SanMarino => {
                // San Marino: 34k population, 61 km² area, ~97% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(43.9424, 12.4578, 4_000.0, 7.0, AreaType::Urban, "San Marino City"),
                    PopulationRegion::new(43.9333, 12.4167, 29_000.0, 54.0, AreaType::Urban, "Urban San Marino"),
                ], 61.0)
            },
            
            Self::SaoTomeAndPrincipe => {
                // São Tomé and Príncipe: 223k population, 964 km² area, ~74% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(0.3365, 6.7273, 90_000.0, 17.0, AreaType::Urban, "São Tomé"),
                    PopulationRegion::new(1.6333, 7.4167, 8_000.0, 4.0, AreaType::Urban, "Santo António"),
                    PopulationRegion::new(0.2017, 6.5317, 6_000.0, 3.0, AreaType::Urban, "Neves"),
                    PopulationRegion::new(0.3, 6.7, 44_000.0, 940.0, AreaType::Rural, "Rural São Tomé"),
                ], 964.0)
            },
            
            Self::SaudiArabia => {
                // Saudi Arabia: 35.0M population, 2,149,690 km² area, ~84% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(24.7136, 46.6753, 7_538_000.0, 1_913.0, AreaType::Urban, "Riyadh"),
                    PopulationRegion::new(21.3891, 39.8579, 4_610_000.0, 1_686.0, AreaType::Urban, "Jeddah"),
                    PopulationRegion::new(21.4267, 39.8261, 2_042_000.0, 760.0, AreaType::Urban, "Mecca"),
                    PopulationRegion::new(24.4861, 39.6111, 1_387_000.0, 589.0, AreaType::Urban, "Medina"),
                    PopulationRegion::new(26.4207, 50.0888, 1_252_000.0, 672.0, AreaType::Urban, "Dammam"),
                    PopulationRegion::new(18.3004, 42.7103, 949_000.0, 334.0, AreaType::Urban, "Abha"),
                    PopulationRegion::new(24.6877, 46.7219, 814_000.0, 280.0, AreaType::Urban, "Kharj"),
                    PopulationRegion::new(26.3267, 43.9750, 768_000.0, 263.0, AreaType::Urban, "Buraydah"),
                    PopulationRegion::new(21.2700, 40.4158, 689_000.0, 235.0, AreaType::Urban, "Taif"),
                    PopulationRegion::new(16.8892, 42.5511, 668_000.0, 228.0, AreaType::Urban, "Jazan"),
                    PopulationRegion::new(24.0, 45.0, 2_900_000.0, 2_145_000.0, AreaType::Rural, "Rural Saudi Arabia"),
                ], 2_149_690.0)
            },
            
            Self::Senegal => {
                // Senegal: 17.2M population, 196,722 km² area, ~48% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(14.7167, -17.4677, 3_140_000.0, 82.0, AreaType::Urban, "Dakar"),
                    PopulationRegion::new(12.5686, -16.2436, 230_000.0, 40.0, AreaType::Urban, "Ziguinchor"),
                    PopulationRegion::new(14.6937, -17.4441, 197_000.0, 30.0, AreaType::Urban, "Thiès"),
                    PopulationRegion::new(14.0, -16.0, 8_900_000.0, 196_600.0, AreaType::Rural, "Rural Senegal"),
                ], 196_722.0)
            },
            
            Self::Serbia => {
                // Serbia: 6.8M population, 88,361 km² area, ~57% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(44.7866, 20.4489, 1_405_000.0, 360.0, AreaType::Urban, "Belgrade"),
                    PopulationRegion::new(45.2671, 19.8335, 306_000.0, 277.0, AreaType::Urban, "Novi Sad"),
                    PopulationRegion::new(43.3209, 21.8954, 187_000.0, 113.0, AreaType::Urban, "Niš"),
                    PopulationRegion::new(44.0144, 20.9114, 147_000.0, 77.0, AreaType::Urban, "Kragujevac"),
                    PopulationRegion::new(44.5, 20.5, 2_920_000.0, 87_500.0, AreaType::Rural, "Rural Serbia"),
                ], 88_361.0)
            },
            
            Self::Seychelles => {
                // Seychelles: 99k population, 452 km² area, ~58% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-4.6827, 55.4804, 28_000.0, 20.0, AreaType::Urban, "Victoria"),
                    PopulationRegion::new(-4.7667, 55.5167, 8_500.0, 6.0, AreaType::Urban, "Anse Royale"),
                    PopulationRegion::new(-4.7833, 55.5333, 4_000.0, 3.0, AreaType::Urban, "Beau Vallon"),
                    PopulationRegion::new(-4.3167, 55.7667, 3_500.0, 2.0, AreaType::Urban, "Baie Lazare"),
                    PopulationRegion::new(-4.6, 55.5, 26_000.0, 421.0, AreaType::Rural, "Rural Seychelles"),
                ], 452.0)
            },
            
            Self::SierraLeone => {
                // Sierra Leone: 8.1M population, 71,740 km² area, ~43% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(8.4656, -13.2317, 1_236_000.0, 68.0, AreaType::Urban, "Freetown"),
                    PopulationRegion::new(7.8765, -11.1875, 241_000.0, 30.0, AreaType::Urban, "Bo"),
                    PopulationRegion::new(8.4821, -11.1885, 128_000.0, 20.0, AreaType::Urban, "Kenema"),
                    PopulationRegion::new(8.5, -12.0, 4_600_000.0, 71_600.0, AreaType::Rural, "Rural Sierra Leone"),
                ], 71_740.0)
            },
            
            Self::Singapore => {
                // Singapore: 5.7M population, 728 km² area, ~100% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(1.3521, 103.8198, 5_686_000.0, 728.0, AreaType::Urban, "Singapore"),
                ], 728.0)
            },
            
            Self::Slovakia => {
                // Slovakia: 5.4M population, 49,037 km² area, ~54% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(48.1486, 17.1077, 475_000.0, 368.0, AreaType::Urban, "Bratislava"),
                    PopulationRegion::new(48.9961, 21.2393, 239_000.0, 243.0, AreaType::Urban, "Košice"),
                    PopulationRegion::new(49.0683, 18.9237, 180_000.0, 193.0, AreaType::Urban, "Prešov"),
                    PopulationRegion::new(48.7163, 21.2611, 95_000.0, 80.0, AreaType::Urban, "Nitra"),
                    PopulationRegion::new(48.5, 19.5, 2_500_000.0, 48_200.0, AreaType::Rural, "Rural Slovakia"),
                ], 49_037.0)
            },
            
            Self::Slovenia => {
                // Slovenia: 2.1M population, 20,273 km² area, ~55% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(46.0569, 14.5058, 537_000.0, 555.0, AreaType::Urban, "Ljubljana Metro"),
                    PopulationRegion::new(46.5547, 15.6466, 141_000.0, 168.0, AreaType::Urban, "Maribor"),
                    PopulationRegion::new(46.2397, 15.2686, 55_000.0, 50.0, AreaType::Urban, "Celje"),
                    PopulationRegion::new(46.2, 14.5, 945_000.0, 19_500.0, AreaType::Rural, "Rural Slovenia"),
                ], 20_273.0)
            },
            
            Self::SolomonIslands => {
                // Solomon Islands: 703k population, 28,896 km² area, ~24% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-9.4319, 159.9556, 93_000.0, 22.0, AreaType::Urban, "Honiara"),
                    PopulationRegion::new(-8.7667, 160.7000, 6_200.0, 5.0, AreaType::Urban, "Auki"),
                    PopulationRegion::new(-10.5667, 161.7667, 5_100.0, 4.0, AreaType::Urban, "Gizo"),
                    PopulationRegion::new(-8.1333, 157.2000, 4_500.0, 3.0, AreaType::Urban, "Buala"),
                    PopulationRegion::new(-9.0, 160.0, 519_200.0, 28_862.0, AreaType::Rural, "Rural Solomon Islands"),
                ], 28_896.0)
            },
            
            Self::Somalia => {
                // Somalia: 16.4M population, 637,657 km² area, ~46% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(2.0469, 45.3182, 2_610_000.0, 91.0, AreaType::Urban, "Mogadishu"),
                    PopulationRegion::new(9.5200, 44.0700, 515_000.0, 50.0, AreaType::Urban, "Hargeisa"),
                    PopulationRegion::new(0.5667, 47.5000, 437_000.0, 40.0, AreaType::Urban, "Merca"),
                    PopulationRegion::new(6.0, 46.0, 8_800_000.0, 637_500.0, AreaType::Rural, "Rural Somalia"),
                ], 637_657.0)
            },
            
            Self::SouthAfrica => {
                // South Africa: 60.0M population, 1,221,037 km² area, ~67% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-26.2041, 28.0473, 11_191_000.0, 6_856.0, AreaType::Urban, "Johannesburg Metro"),
                    PopulationRegion::new(-33.9249, 18.4241, 4_710_000.0, 2_446.0, AreaType::Urban, "Cape Town Metro"),
                    PopulationRegion::new(-29.8587, 31.0292, 3_969_000.0, 2_556.0, AreaType::Urban, "Durban Metro"),
                    PopulationRegion::new(-25.7479, 28.2293, 3_888_000.0, 6_197.0, AreaType::Urban, "Pretoria Metro"),
                    PopulationRegion::new(-33.9608, 25.6022, 770_000.0, 947.0, AreaType::Urban, "Port Elizabeth Metro"),
                    PopulationRegion::new(-32.0833, 18.8666, 348_000.0, 13_647.0, AreaType::Urban, "Saldanha Bay"),
                    PopulationRegion::new(-33.0167, 27.9, 312_000.0, 6_357.0, AreaType::Urban, "East London"),
                    PopulationRegion::new(-28.7282, 24.7499, 255_000.0, 164.0, AreaType::Urban, "Kimberley"),
                    PopulationRegion::new(-26.6667, 27.0833, 232_000.0, 38.0, AreaType::Urban, "Welkom"),
                    PopulationRegion::new(-29.0852, 26.1596, 222_000.0, 6_283.0, AreaType::Urban, "Bloemfontein"),
                    PopulationRegion::new(-29.0, 24.0, 18_000_000.0, 1_208_000.0, AreaType::Rural, "Rural South Africa"),
                ], 1_221_037.0)
            },
            
            Self::SouthKorea => {
                // South Korea: 51.7M population, 100,210 km² area, ~81% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(37.5665, 126.9780, 25_674_000.0, 11_690.0, AreaType::Urban, "Seoul Metro"),
                    PopulationRegion::new(35.1796, 129.0756, 3_404_000.0, 770.0, AreaType::Urban, "Busan"),
                    PopulationRegion::new(37.4563, 126.7052, 3_032_000.0, 1_045.0, AreaType::Urban, "Incheon"),
                    PopulationRegion::new(35.8714, 128.6014, 2_431_000.0, 884.0, AreaType::Urban, "Daegu"),
                    PopulationRegion::new(35.1595, 126.8526, 1_478_000.0, 501.0, AreaType::Urban, "Gwangju"),
                    PopulationRegion::new(36.3504, 127.3845, 1_542_000.0, 540.0, AreaType::Urban, "Daejeon"),
                    PopulationRegion::new(35.5384, 129.3114, 1_046_000.0, 1_058.0, AreaType::Urban, "Ulsan"),
                    PopulationRegion::new(37.8813, 127.7298, 983_000.0, 872.0, AreaType::Urban, "Chuncheon"),
                    PopulationRegion::new(36.5, 127.5, 6_700_000.0, 85_300.0, AreaType::Rural, "Rural South Korea"),
                ], 100_210.0)
            },
            
            Self::SouthSudan => {
                // South Sudan: 11.4M population, 619,745 km² area, ~20% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(4.8594, 31.5713, 525_000.0, 52.0, AreaType::Urban, "Juba"),
                    PopulationRegion::new(9.5370, 31.6603, 90_000.0, 20.0, AreaType::Urban, "Wau"),
                    PopulationRegion::new(6.8100, 29.6950, 61_000.0, 15.0, AreaType::Urban, "Yei"),
                    PopulationRegion::new(7.0, 30.0, 9_100_000.0, 619_600.0, AreaType::Rural, "Rural South Sudan"),
                ], 619_745.0)
            },
            
            Self::Spain => {
                // Spain: 47.4M population, 505,992 km² area, ~81% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(40.4168, -3.7038, 6_751_000.0, 606.0, AreaType::Urban, "Madrid Metro"),
                    PopulationRegion::new(41.3851, 2.1734, 5_609_000.0, 636.0, AreaType::Urban, "Barcelona Metro"),
                    PopulationRegion::new(39.4699, -0.3763, 2_560_000.0, 628.0, AreaType::Urban, "Valencia Metro"),
                    PopulationRegion::new(37.3891, -5.9845, 1_949_000.0, 4_906.0, AreaType::Urban, "Seville Metro"),
                    PopulationRegion::new(36.7213, -4.4217, 1_661_000.0, 1_336.0, AreaType::Urban, "Málaga Metro"),
                    PopulationRegion::new(43.2627, -2.9253, 1_037_000.0, 497.0, AreaType::Urban, "Bilbao Metro"),
                    PopulationRegion::new(39.5750, 2.6502, 634_000.0, 209.0, AreaType::Urban, "Palma"),
                    PopulationRegion::new(37.9922, -1.1307, 493_000.0, 558.0, AreaType::Urban, "Murcia"),
                    PopulationRegion::new(36.5344, -6.2994, 477_000.0, 32.0, AreaType::Urban, "Cádiz"),
                    PopulationRegion::new(37.8882, -4.7794, 472_000.0, 38.0, AreaType::Urban, "Córdoba"),
                    PopulationRegion::new(40.0, -4.0, 7_600_000.0, 497_000.0, AreaType::Rural, "Rural Spain"),
                ], 505_992.0)
            },
            
            Self::SriLanka => {
                // Sri Lanka: 21.9M population, 65,610 km² area, ~19% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(6.9271, 79.8612, 753_000.0, 37.0, AreaType::Urban, "Colombo"),
                    PopulationRegion::new(7.4675, 80.6234, 169_000.0, 26.0, AreaType::Urban, "Kurunegala"),
                    PopulationRegion::new(7.2906, 80.6337, 140_000.0, 30.0, AreaType::Urban, "Kandy"),
                    PopulationRegion::new(8.0, 80.5, 17_700_000.0, 65_500.0, AreaType::Rural, "Rural Sri Lanka"),
                ], 65_610.0)
            },
            
            Self::Sudan => {
                // Sudan: 44.9M population, 1,886,068 km² area, ~35% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(15.5007, 32.5599, 5_829_000.0, 1_010.0, AreaType::Urban, "Khartoum Metro"),
                    PopulationRegion::new(19.6158, 37.2164, 524_000.0, 52.0, AreaType::Urban, "Port Sudan"),
                    PopulationRegion::new(13.1833, 30.2167, 524_000.0, 75.0, AreaType::Urban, "Nyala"),
                    PopulationRegion::new(11.8659, 34.3869, 344_000.0, 40.0, AreaType::Urban, "Kusti"),
                    PopulationRegion::new(15.0, 30.0, 29_200_000.0, 1_884_900.0, AreaType::Rural, "Rural Sudan"),
                ], 1_886_068.0)
            },
            
            Self::Suriname => {
                // Suriname: 598k population, 163,820 km² area, ~66% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(5.8521, -55.2038, 241_000.0, 182.0, AreaType::Urban, "Paramaribo"),
                    PopulationRegion::new(5.8, -55.2, 203_000.0, 163_600.0, AreaType::Rural, "Rural Suriname"),
                ], 163_820.0)
            },
            
            Self::Swaziland => {
                // Eswatini: 1.2M population, 17,364 km² area, ~24% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-26.3054, 31.1367, 95_000.0, 81.0, AreaType::Urban, "Mbabane"),
                    PopulationRegion::new(-26.4833, 31.2000, 76_000.0, 60.0, AreaType::Urban, "Manzini"),
                    PopulationRegion::new(-26.5, 31.5, 914_000.0, 17_200.0, AreaType::Rural, "Rural Eswatini"),
                ], 17_364.0)
            },
            
            Self::Sweden => {
                // Sweden: 10.4M population, 450,295 km² area, ~88% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(59.3293, 18.0686, 2_415_000.0, 1_912.0, AreaType::Urban, "Stockholm Metro"),
                    PopulationRegion::new(57.7089, 11.9746, 1_089_000.0, 896.0, AreaType::Urban, "Gothenburg Metro"),
                    PopulationRegion::new(55.6050, 13.0038, 749_000.0, 347.0, AreaType::Urban, "Malmö Metro"),
                    PopulationRegion::new(59.8586, 17.6389, 177_000.0, 49.0, AreaType::Urban, "Uppsala"),
                    PopulationRegion::new(62.3908, 17.3069, 127_000.0, 30.0, AreaType::Urban, "Sundsvall"),
                    PopulationRegion::new(63.8258, 20.2630, 123_000.0, 29.0, AreaType::Urban, "Umeå"),
                    PopulationRegion::new(58.4108, 15.6214, 110_000.0, 25.0, AreaType::Urban, "Linköping"),
                    PopulationRegion::new(56.1612, 15.5869, 93_000.0, 20.0, AreaType::Urban, "Karlskrona"),
                    PopulationRegion::new(60.0, 15.0, 800_000.0, 447_000.0, AreaType::Rural, "Rural Sweden"),
                ], 450_295.0)
            },
            
            Self::Switzerland => {
                // Switzerland: 8.7M population, 41,285 km² area, ~74% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(47.3769, 8.5417, 1_408_000.0, 1_729.0, AreaType::Urban, "Zürich Metro"),
                    PopulationRegion::new(46.2044, 6.1432, 609_000.0, 343.0, AreaType::Urban, "Geneva Metro"),
                    PopulationRegion::new(47.5596, 7.5886, 537_000.0, 559.0, AreaType::Urban, "Basel Metro"),
                    PopulationRegion::new(46.9480, 7.4474, 423_000.0, 233.0, AreaType::Urban, "Bern Metro"),
                    PopulationRegion::new(46.5197, 6.6323, 438_000.0, 376.0, AreaType::Urban, "Lausanne Metro"),
                    PopulationRegion::new(47.0, 8.5, 2_260_000.0, 38_400.0, AreaType::Rural, "Rural Switzerland"),
                ], 41_285.0)
            },
            
            Self::Syria => {
                // Syria: 18.5M population, 185,180 km² area, ~56% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(33.5138, 36.2765, 2_211_000.0, 105.0, AreaType::Urban, "Damascus"),
                    PopulationRegion::new(36.2021, 37.1343, 1_917_000.0, 190.0, AreaType::Urban, "Aleppo"),
                    PopulationRegion::new(33.5074, 36.3097, 968_000.0, 60.0, AreaType::Urban, "Rif Dimashq"),
                    PopulationRegion::new(34.7305, 36.7137, 809_000.0, 102.0, AreaType::Urban, "Homs"),
                    PopulationRegion::new(35.0, 38.0, 8_100_000.0, 184_800.0, AreaType::Rural, "Rural Syria"),
                ], 185_180.0)
            },
            
            Self::Taiwan => {
                // Taiwan: 23.6M population, 36,193 km² area, ~79% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(25.0330, 121.5654, 7_047_000.0, 2_457.0, AreaType::Urban, "Taipei Metro"),
                    PopulationRegion::new(22.6273, 120.3014, 2_773_000.0, 176.0, AreaType::Urban, "Kaohsiung"),
                    PopulationRegion::new(24.1477, 120.6736, 2_820_000.0, 2_192.0, AreaType::Urban, "Taichung"),
                    PopulationRegion::new(22.9998, 120.2269, 764_000.0, 69.0, AreaType::Urban, "Tainan"),
                    PopulationRegion::new(24.8015, 120.9715, 218_000.0, 29.0, AreaType::Urban, "Hsinchu"),
                    PopulationRegion::new(24.5, 121.0, 4_950_000.0, 31_000.0, AreaType::Rural, "Rural Taiwan"),
                ], 36_193.0)
            },
            
            Self::Tajikistan => {
                // Tajikistan: 9.7M population, 143,100 km² area, ~27% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(38.5598, 68.7870, 912_000.0, 124.0, AreaType::Urban, "Dushanbe"),
                    PopulationRegion::new(40.2833, 69.6333, 150_000.0, 35.0, AreaType::Urban, "Khujand"),
                    PopulationRegion::new(38.5484, 68.7790, 109_000.0, 25.0, AreaType::Urban, "Bokhtar"),
                    PopulationRegion::new(39.0, 70.0, 7_100_000.0, 142_900.0, AreaType::Rural, "Rural Tajikistan"),
                ], 143_100.0)
            },
            
            Self::Tanzania => {
                // Tanzania: 61.5M population, 945,087 km² area, ~35% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-6.7924, 39.2083, 6_368_000.0, 1_590.0, AreaType::Urban, "Dar es Salaam"),
                    PopulationRegion::new(-3.3869, 36.6830, 621_000.0, 83.0, AreaType::Urban, "Arusha"),
                    PopulationRegion::new(-5.0689, 39.0986, 540_000.0, 65.0, AreaType::Urban, "Tanga"),
                    PopulationRegion::new(-2.5166, 32.9000, 540_000.0, 70.0, AreaType::Urban, "Mwanza"),
                    PopulationRegion::new(-5.9095, 35.7516, 436_000.0, 58.0, AreaType::Urban, "Dodoma"),
                    PopulationRegion::new(-8.9094, 33.4608, 414_000.0, 53.0, AreaType::Urban, "Mbeya"),
                    PopulationRegion::new(-4.8723, 29.6420, 303_000.0, 38.0, AreaType::Urban, "Kigoma"),
                    PopulationRegion::new(-10.2667, 40.1833, 280_000.0, 35.0, AreaType::Urban, "Mtwara"),
                    PopulationRegion::new(-6.0, 35.0, 37_500_000.0, 943_200.0, AreaType::Rural, "Rural Tanzania"),
                ], 945_087.0)
            },
            
            Self::Thailand => {
                // Thailand: 70.0M population, 513,120 km² area, ~51% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(13.7563, 100.5018, 11_070_000.0, 1_569.0, AreaType::Urban, "Bangkok Metro"),
                    PopulationRegion::new(7.0086, 100.4733, 439_000.0, 70.0, AreaType::Urban, "Hat Yai"),
                    PopulationRegion::new(18.7883, 98.9853, 435_000.0, 40.0, AreaType::Urban, "Chiang Mai"),
                    PopulationRegion::new(14.9714, 102.1027, 358_000.0, 46.0, AreaType::Urban, "Nakhon Ratchasima"),
                    PopulationRegion::new(16.4322, 102.8236, 355_000.0, 40.0, AreaType::Urban, "Khon Kaen"),
                    PopulationRegion::new(12.6095, 102.1038, 315_000.0, 35.0, AreaType::Urban, "Chonburi"),
                    PopulationRegion::new(16.8211, 100.2668, 300_000.0, 32.0, AreaType::Urban, "Phitsanulok"),
                    PopulationRegion::new(15.0, 100.0, 33_400_000.0, 511_400.0, AreaType::Rural, "Rural Thailand"),
                ], 513_120.0)
            },
            
            Self::Togo => {
                // Togo: 8.5M population, 56,785 km² area, ~43% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(6.1256, 1.2254, 2_188_000.0, 280.0, AreaType::Urban, "Lomé"),
                    PopulationRegion::new(8.9831, 1.1444, 168_000.0, 30.0, AreaType::Urban, "Sokodé"),
                    PopulationRegion::new(9.5511, 1.1865, 158_000.0, 25.0, AreaType::Urban, "Kara"),
                    PopulationRegion::new(8.0, 1.0, 4_800_000.0, 56_400.0, AreaType::Rural, "Rural Togo"),
                ], 56_785.0)
            },
            
            Self::Tonga => {
                // Tonga: 107k population, 747 km² area, ~23% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-21.1393, -175.2049, 23_000.0, 30.0, AreaType::Urban, "Nukuʻalofa"),
                    PopulationRegion::new(-21.1333, -175.2, 6_000.0, 8.0, AreaType::Urban, "Neiafu"),
                    PopulationRegion::new(-20.0333, -174.3667, 3_500.0, 5.0, AreaType::Urban, "Haveluloto"),
                    PopulationRegion::new(-18.6500, -173.9833, 3_000.0, 4.0, AreaType::Urban, "Vava'u"),
                    PopulationRegion::new(-21.0, -175.0, 69_500.0, 700.0, AreaType::Rural, "Rural Tonga"),
                ], 747.0)
            },
            
            Self::TrinidadAndTobago => {
                // Trinidad and Tobago: 1.4M population, 5,130 km² area, ~53% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(10.6596, -61.5086, 545_000.0, 547.0, AreaType::Urban, "Port of Spain Metro"),
                    PopulationRegion::new(10.2667, -61.4667, 83_000.0, 20.0, AreaType::Urban, "San Fernando"),
                    PopulationRegion::new(10.6425, -61.4013, 61_000.0, 15.0, AreaType::Urban, "Arima"),
                    PopulationRegion::new(10.5333, -61.4119, 58_000.0, 13.0, AreaType::Urban, "Chaguanas"),
                    PopulationRegion::new(11.1891, -60.7525, 18_000.0, 5.0, AreaType::Urban, "Scarborough"),
                    PopulationRegion::new(10.5, -61.3, 570_000.0, 4_530.0, AreaType::Rural, "Rural Trinidad"),
                ], 5_130.0)
            },
            
            Self::Tunisia => {
                // Tunisia: 12.0M population, 163,610 km² area, ~70% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(36.8065, 10.1815, 2_845_000.0, 3_135.0, AreaType::Urban, "Tunis Metro"),
                    PopulationRegion::new(35.8288, 10.6405, 742_000.0, 235.0, AreaType::Urban, "Sousse"),
                    PopulationRegion::new(36.4514, 10.7356, 291_000.0, 63.0, AreaType::Urban, "Nabeul"),
                    PopulationRegion::new(35.6804, 10.0982, 275_000.0, 40.0, AreaType::Urban, "Kairouan"),
                    PopulationRegion::new(36.0, 9.0, 3_600_000.0, 160_100.0, AreaType::Rural, "Rural Tunisia"),
                ], 163_610.0)
            },
            
            Self::Turkey => {
                // Turkey: 85.0M population, 783,562 km² area, ~77% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(41.0082, 28.9784, 15_848_000.0, 5_343.0, AreaType::Urban, "Istanbul"),
                    PopulationRegion::new(39.9334, 32.8597, 5_748_000.0, 25_437.0, AreaType::Urban, "Ankara"),
                    PopulationRegion::new(38.4192, 27.1287, 4_440_000.0, 8_276.0, AreaType::Urban, "Izmir"),
                    PopulationRegion::new(40.1885, 29.0610, 3_175_000.0, 4_302.0, AreaType::Urban, "Bursa"),
                    PopulationRegion::new(36.9914, 35.3308, 2_254_000.0, 13_488.0, AreaType::Urban, "Adana"),
                    PopulationRegion::new(37.8737, 32.4847, 2_254_000.0, 20_814.0, AreaType::Urban, "Konya"),
                    PopulationRegion::new(36.9081, 30.6956, 2_619_000.0, 19_086.0, AreaType::Urban, "Antalya"),
                    PopulationRegion::new(37.0660, 37.3781, 1_986_000.0, 3_584.0, AreaType::Urban, "Gaziantep"),
                    PopulationRegion::new(38.7317, 35.4787, 1_241_000.0, 13_251.0, AreaType::Urban, "Kayseri"),
                    PopulationRegion::new(41.0053, 39.7277, 757_000.0, 6_277.0, AreaType::Urban, "Trabzon"),
                    PopulationRegion::new(39.0, 35.0, 14_800_000.0, 700_000.0, AreaType::Rural, "Rural Turkey"),
                ], 783_562.0)
            },
            
            Self::Turkmenistan => {
                // Turkmenistan: 6.0M population, 488_100 km² area, ~52% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(37.9500, 58.3833, 810_000.0, 200.0, AreaType::Urban, "Ashgabat"),
                    PopulationRegion::new(39.0833, 63.5833, 408_000.0, 75.0, AreaType::Urban, "Türkmenabat"),
                    PopulationRegion::new(38.9697, 56.2711, 243_000.0, 50.0, AreaType::Urban, "Daşoguz"),
                    PopulationRegion::new(39.0, 59.0, 2_880_000.0, 487_800.0, AreaType::Rural, "Rural Turkmenistan"),
                ], 488_100.0)
            },
            
            Self::Tuvalu => {
                // Tuvalu: 12k population, 26 km² area, ~63% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-8.5243, 179.1942, 7_000.0, 3.0, AreaType::Urban, "Funafuti"),
                    PopulationRegion::new(-8.5, 179.0, 4_000.0, 23.0, AreaType::Rural, "Rural Tuvalu"),
                ], 26.0)
            },
            
            Self::Uganda => {
                // Uganda: 47.1M population, 241,038 km² area, ~26% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(0.3163, 32.5822, 3_652_000.0, 189.0, AreaType::Urban, "Kampala Metro"),
                    PopulationRegion::new(0.6201, 32.4450, 509_000.0, 46.0, AreaType::Urban, "Wakiso"),
                    PopulationRegion::new(-0.4452, 32.4421, 289_000.0, 31.0, AreaType::Urban, "Mukono"),
                    PopulationRegion::new(0.0, 32.0, 35_000_000.0, 240_700.0, AreaType::Rural, "Rural Uganda"),
                ], 241_038.0)
            },
            
            Self::Ukraine => {
                // Ukraine: 43.2M population, 603,500 km² area, ~70% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(50.4501, 30.5234, 3_017_000.0, 839.0, AreaType::Urban, "Kyiv"),
                    PopulationRegion::new(49.9935, 36.2304, 1_443_000.0, 350.0, AreaType::Urban, "Kharkiv"),
                    PopulationRegion::new(46.4825, 30.7233, 1_010_000.0, 162.0, AreaType::Urban, "Odesa"),
                    PopulationRegion::new(48.4647, 35.0462, 980_000.0, 424.0, AreaType::Urban, "Dnipro"),
                    PopulationRegion::new(47.8388, 35.1396, 715_000.0, 236.0, AreaType::Urban, "Zaporizhzhia"),
                    PopulationRegion::new(48.0, 33.0, 13_000_000.0, 601_600.0, AreaType::Rural, "Rural Ukraine"),
                ], 603_500.0)
            },
            
            Self::UnitedArabEmirates => {
                // UAE: 10.0M population, 83,600 km² area, ~87% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(25.2048, 55.2708, 3_481_000.0, 4_114.0, AreaType::Urban, "Dubai"),
                    PopulationRegion::new(24.4539, 54.3773, 1_684_000.0, 972.0, AreaType::Urban, "Abu Dhabi"),
                    PopulationRegion::new(25.3463, 55.4209, 807_000.0, 259.0, AreaType::Urban, "Sharjah"),
                    PopulationRegion::new(25.4052, 56.2611, 545_000.0, 100.0, AreaType::Urban, "Ajman"),
                    PopulationRegion::new(25.3350, 55.3890, 262_000.0, 80.0, AreaType::Urban, "Al Ain"),
                    PopulationRegion::new(25.4000, 55.5136, 230_000.0, 70.0, AreaType::Urban, "Ras Al Khaimah"),
                    PopulationRegion::new(25.1294, 55.3765, 140_000.0, 60.0, AreaType::Urban, "Fujairah"),
                    PopulationRegion::new(24.9, 55.0, 668_000.0, 78_145.0, AreaType::Rural, "Rural UAE"),
                ], 83_600.0)
            },
            
            Self::UnitedKingdom => {
                // UK: 67.7M population, 242,495 km² area, ~84% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(51.5074, -0.1278, 9_648_000.0, 1_738.0, AreaType::Urban, "London Metro"),
                    PopulationRegion::new(52.4862, -1.8904, 2_938_000.0, 599.0, AreaType::Urban, "Birmingham Metro"),
                    PopulationRegion::new(53.4808, -2.2426, 2_731_000.0, 1_277.0, AreaType::Urban, "Manchester Metro"),
                    PopulationRegion::new(53.8008, -1.5491, 2_347_000.0, 488.0, AreaType::Urban, "Leeds-Bradford"),
                    PopulationRegion::new(55.8642, -4.2518, 1_861_000.0, 3_339.0, AreaType::Urban, "Glasgow Metro"),
                    PopulationRegion::new(53.4084, -2.9916, 1_557_000.0, 646.0, AreaType::Urban, "Liverpool Metro"),
                    PopulationRegion::new(53.3811, -1.4701, 1_376_000.0, 1_553.0, AreaType::Urban, "Sheffield Metro"),
                    PopulationRegion::new(54.9783, -1.6178, 1_100_000.0, 2_329.0, AreaType::Urban, "Newcastle Metro"),
                    PopulationRegion::new(52.9548, -1.1581, 919_000.0, 641.0, AreaType::Urban, "Nottingham Metro"),
                    PopulationRegion::new(51.4545, -2.5879, 909_000.0, 1_015.0, AreaType::Urban, "Bristol Metro"),
                    PopulationRegion::new(55.9533, -3.1883, 902_000.0, 264.0, AreaType::Urban, "Edinburgh"),
                    PopulationRegion::new(50.8229, -0.1363, 789_000.0, 641.0, AreaType::Urban, "Brighton"),
                    PopulationRegion::new(52.2053, 0.1218, 752_000.0, 634.0, AreaType::Urban, "Cambridge"),
                    PopulationRegion::new(50.9097, -1.4044, 636_000.0, 394.0, AreaType::Urban, "Southampton"),
                    PopulationRegion::new(50.7256, -3.5269, 617_000.0, 816.0, AreaType::Urban, "Plymouth"),
                    PopulationRegion::new(53.0, -1.5, 6_500_000.0, 230_000.0, AreaType::Rural, "Rural UK"),
                ], 242_495.0)
            },
            
            Self::UnitedStatesOfAmerica => {
                // USA: 331.9M population, 9,833,520 km² area, ~83% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(40.7128, -74.0060, 20_141_000.0, 17_405.0, AreaType::Urban, "New York Metro"),
                    PopulationRegion::new(34.0522, -118.2437, 13_201_000.0, 12_562.0, AreaType::Urban, "Los Angeles Metro"),
                    PopulationRegion::new(41.8781, -87.6298, 9_620_000.0, 10_857.0, AreaType::Urban, "Chicago Metro"),
                    PopulationRegion::new(32.7767, -96.7970, 7_638_000.0, 9_287.0, AreaType::Urban, "Dallas-Fort Worth"),
                    PopulationRegion::new(29.7604, -95.3698, 7_122_000.0, 10_063.0, AreaType::Urban, "Houston Metro"),
                    PopulationRegion::new(38.9072, -77.0369, 6_385_000.0, 14_413.0, AreaType::Urban, "Washington DC Metro"),
                    PopulationRegion::new(39.9526, -75.1652, 6_246_000.0, 13_257.0, AreaType::Urban, "Philadelphia Metro"),
                    PopulationRegion::new(33.7490, -84.3880, 6_090_000.0, 8_377.0, AreaType::Urban, "Atlanta Metro"),
                    PopulationRegion::new(25.7617, -80.1918, 6_013_000.0, 6_138.0, AreaType::Urban, "Miami Metro"),
                    PopulationRegion::new(33.4484, -112.0740, 5_060_000.0, 14_599.0, AreaType::Urban, "Phoenix Metro"),
                    PopulationRegion::new(42.3601, -71.0589, 4_942_000.0, 11_701.0, AreaType::Urban, "Boston Metro"),
                    PopulationRegion::new(37.7749, -122.4194, 4_750_000.0, 6_985.0, AreaType::Urban, "San Francisco Bay Area"),
                    PopulationRegion::new(32.7157, -117.1611, 3_339_000.0, 4_527.0, AreaType::Urban, "San Diego Metro"),
                    PopulationRegion::new(44.9778, -93.2650, 3_691_000.0, 16_012.0, AreaType::Urban, "Minneapolis-St. Paul"),
                    PopulationRegion::new(42.8864, -78.8784, 4_089_000.0, 7_315.0, AreaType::Urban, "Detroit Metro"),
                    PopulationRegion::new(27.9506, -82.4572, 3_243_000.0, 6_376.0, AreaType::Urban, "Tampa Bay Metro"),
                    PopulationRegion::new(47.6062, -122.3321, 4_019_000.0, 15_210.0, AreaType::Urban, "Seattle Metro"),
                    PopulationRegion::new(39.7392, -104.9903, 2_968_000.0, 21_622.0, AreaType::Urban, "Denver Metro"),
                    PopulationRegion::new(45.5152, -122.6784, 2_513_000.0, 6_685.0, AreaType::Urban, "Portland Metro"),
                    PopulationRegion::new(28.5383, -81.3792, 2_609_000.0, 4_013.0, AreaType::Urban, "Orlando Metro"),
                    PopulationRegion::new(39.0997, -94.5786, 2_193_000.0, 20_570.0, AreaType::Urban, "Kansas City Metro"),
                    PopulationRegion::new(40.4406, -79.9959, 2_324_000.0, 13_761.0, AreaType::Urban, "Pittsburgh Metro"),
                    PopulationRegion::new(40.7608, -111.8910, 2_607_000.0, 21_590.0, AreaType::Urban, "Salt Lake City Metro"),
                    PopulationRegion::new(39.7684, -86.1581, 2_111_000.0, 9_885.0, AreaType::Urban, "Indianapolis Metro"),
                    PopulationRegion::new(39.1031, -84.5120, 2_256_000.0, 12_072.0, AreaType::Urban, "Cincinnati Metro"),
                    PopulationRegion::new(41.4993, -81.6944, 2_048_000.0, 10_895.0, AreaType::Urban, "Cleveland Metro"),
                    PopulationRegion::new(36.1627, -86.7816, 2_011_000.0, 13_310.0, AreaType::Urban, "Nashville Metro"),
                    PopulationRegion::new(30.2672, -97.7431, 2_295_000.0, 11_100.0, AreaType::Urban, "Austin Metro"),
                    PopulationRegion::new(29.4241, -98.4936, 2_588_000.0, 20_824.0, AreaType::Urban, "San Antonio Metro"),
                    PopulationRegion::new(36.1540, -95.9928, 1_044_000.0, 9_305.0, AreaType::Urban, "Tulsa Metro"),
                    PopulationRegion::new(38.0, -97.0, 40_000_000.0, 9_600_000.0, AreaType::Rural, "Rural USA"),
                ], 9_833_520.0)
            },
            
            Self::Uruguay => {
                // Uruguay: 3.5M population, 181,034 km² area, ~95% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-34.9011, -56.1645, 1_381_000.0, 200.0, AreaType::Urban, "Montevideo"),
                    PopulationRegion::new(-31.3833, -57.9667, 113_000.0, 25.0, AreaType::Urban, "Salto"),
                    PopulationRegion::new(-32.3333, -58.0833, 87_000.0, 20.0, AreaType::Urban, "Paysandú"),
                    PopulationRegion::new(-33.2524, -58.0265, 78_000.0, 17.0, AreaType::Urban, "Mercedes"),
                    PopulationRegion::new(-34.3375, -56.7139, 58_000.0, 13.0, AreaType::Urban, "Las Piedras"),
                    PopulationRegion::new(-33.0, -56.0, 50_000.0, 180_759.0, AreaType::Rural, "Rural Uruguay"),
                ], 181_034.0)
            },
            
            Self::Uzbekistan => {
                // Uzbekistan: 34.9M population, 448,978 km² area, ~50% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(41.2995, 69.2401, 2_571_000.0, 335.0, AreaType::Urban, "Tashkent"),
                    PopulationRegion::new(39.6530, 66.9597, 546_000.0, 120.0, AreaType::Urban, "Samarkand"),
                    PopulationRegion::new(40.3789, 71.7825, 544_000.0, 90.0, AreaType::Urban, "Namangan"),
                    PopulationRegion::new(40.5283, 70.9429, 544_000.0, 77.0, AreaType::Urban, "Andijan"),
                    PopulationRegion::new(39.7747, 64.4286, 279_000.0, 60.0, AreaType::Urban, "Bukhara"),
                    PopulationRegion::new(40.8420, 69.6333, 259_000.0, 55.0, AreaType::Urban, "Qarshi"),
                    PopulationRegion::new(42.4589, 59.6103, 232_000.0, 50.0, AreaType::Urban, "Nukus"),
                    PopulationRegion::new(40.1167, 67.8422, 183_000.0, 40.0, AreaType::Urban, "Jizzakh"),
                    PopulationRegion::new(41.0, 64.0, 16_500_000.0, 448_251.0, AreaType::Rural, "Rural Uzbekistan"),
                ], 448_978.0)
            },
            
            Self::Vanuatu => {
                // Vanuatu: 315k population, 12,189 km² area, ~25% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-17.7333, 168.3273, 66_000.0, 23.0, AreaType::Urban, "Port Vila"),
                    PopulationRegion::new(-15.3767, 167.1627, 16_000.0, 10.0, AreaType::Urban, "Luganville"),
                    PopulationRegion::new(-17.0, 168.0, 237_000.0, 12_156.0, AreaType::Rural, "Rural Vanuatu"),
                ], 12_189.0)
            },
            
            Self::VaticanCity => {
                // Vatican City: 825 population, 0.44 km² area, ~100% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(41.9029, 12.4534, 825.0, 0.44, AreaType::Urban, "Vatican City"),
                ], 0.44)
            },
            
            Self::Venezuela => {
                // Venezuela: 28.7M population, 916,445 km² area, ~88% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(10.4806, -66.9036, 5_285_000.0, 4_715.0, AreaType::Urban, "Caracas Metro"),
                    PopulationRegion::new(10.6585, -71.6406, 2_458_000.0, 623.0, AreaType::Urban, "Maracaibo"),
                    PopulationRegion::new(10.2442, -67.6066, 2_182_000.0, 2_050.0, AreaType::Urban, "Valencia"),
                    PopulationRegion::new(10.3632, -71.6245, 1_628_000.0, 441.0, AreaType::Urban, "Maracay"),
                    PopulationRegion::new(10.2340, -64.1934, 1_239_000.0, 192.0, AreaType::Urban, "Barcelona-Puerto La Cruz"),
                    PopulationRegion::new(8.6196, -70.2049, 495_000.0, 30.0, AreaType::Urban, "Mérida"),
                    PopulationRegion::new(7.8936, -72.5048, 463_000.0, 25.0, AreaType::Urban, "San Cristóbal"),
                    PopulationRegion::new(10.0678, -69.3467, 453_000.0, 20.0, AreaType::Urban, "Barquisimeto"),
                    PopulationRegion::new(8.0, -66.0, 2_000_000.0, 909_000.0, AreaType::Rural, "Rural Venezuela"),
                ], 916_445.0)
            },
            
            Self::Vietnam => {
                // Vietnam: 98.2M population, 331,212 km² area, ~38% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(10.8231, 106.6297, 8_993_000.0, 2_061.0, AreaType::Urban, "Ho Chi Minh City"),
                    PopulationRegion::new(21.0285, 105.8542, 8_054_000.0, 3_359.0, AreaType::Urban, "Hanoi"),
                    PopulationRegion::new(20.8449, 106.6881, 1_493_000.0, 1_495.0, AreaType::Urban, "Haiphong"),
                    PopulationRegion::new(16.0544, 108.2022, 1_231_000.0, 1_285.0, AreaType::Urban, "Da Nang"),
                    PopulationRegion::new(10.2539, 105.9721, 1_200_000.0, 1_440.0, AreaType::Urban, "Can Tho"),
                    PopulationRegion::new(11.9404, 108.4583, 640_000.0, 3_568.0, AreaType::Urban, "Dalat"),
                    PopulationRegion::new(10.9642, 106.8251, 532_000.0, 209.0, AreaType::Urban, "Bien Hoa"),
                    PopulationRegion::new(12.2388, 109.1888, 527_000.0, 209.0, AreaType::Urban, "Nha Trang"),
                    PopulationRegion::new(16.4498, 107.5623, 519_000.0, 117.0, AreaType::Urban, "Hue"),
                    PopulationRegion::new(10.5417, 106.4142, 513_000.0, 650.0, AreaType::Urban, "Vung Tau"),
                    PopulationRegion::new(16.0, 107.0, 57_300_000.0, 322_600.0, AreaType::Rural, "Rural Vietnam"),
                ], 331_212.0)
            },
            
            Self::Yemen => {
                // Yemen: 30.5M population, 527,968 km² area, ~38% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(15.3547, 44.2067, 3_075_000.0, 200.0, AreaType::Urban, "Sana'a"),
                    PopulationRegion::new(12.7794, 45.0187, 1_079_000.0, 75.0, AreaType::Urban, "Aden"),
                    PopulationRegion::new(14.5434, 49.1241, 738_000.0, 60.0, AreaType::Urban, "Al Mukalla"),
                    PopulationRegion::new(13.5766, 44.0177, 724_000.0, 50.0, AreaType::Urban, "Ta'izz"),
                    PopulationRegion::new(15.0, 48.0, 18_900_000.0, 527_600.0, AreaType::Rural, "Rural Yemen"),
                ], 527_968.0)
            },
            
            Self::Zambia => {
                // Zambia: 19.0M population, 752,618 km² area, ~45% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-15.4067, 28.2871, 2_906_000.0, 375.0, AreaType::Urban, "Lusaka"),
                    PopulationRegion::new(-12.9658, 28.6286, 765_000.0, 85.0, AreaType::Urban, "Ndola"),
                    PopulationRegion::new(-12.8097, 28.2133, 678_000.0, 52.0, AreaType::Urban, "Kitwe"),
                    PopulationRegion::new(-15.8562, 27.8523, 293_000.0, 30.0, AreaType::Urban, "Kabwe"),
                    PopulationRegion::new(-13.0, 28.0, 10_400_000.0, 752_100.0, AreaType::Rural, "Rural Zambia"),
                ], 752_618.0)
            },
            
            Self::Zimbabwe => {
                // Zimbabwe: 15.1M population, 390,757 km² area, ~32% urban.
                CountryPopulationData::from_regions(vec![
                    PopulationRegion::new(-17.8252, 31.0335, 2_331_000.0, 872.0, AreaType::Urban, "Harare Metro"),
                    PopulationRegion::new(-20.1530, 28.5766, 741_000.0, 479.0, AreaType::Urban, "Bulawayo"),
                    PopulationRegion::new(-18.0208, 31.0719, 200_000.0, 30.0, AreaType::Urban, "Chitungwiza"),
                    PopulationRegion::new(-19.4500, 29.8167, 155_000.0, 25.0, AreaType::Urban, "Gweru"),
                    PopulationRegion::new(-19.0, 30.0, 10_300_000.0, 389_400.0, AreaType::Rural, "Rural Zimbabwe"),
                ], 390_757.0)
            },
        }
    }

}

/// Classification of a population centre as urban or rural.
#[derive(Clone, Debug)]
pub struct LocationPopulationRegion {
    /// Latitude of a location in the region in decimal degrees.
    pub latitude: f64,
    /// Longitude of a location in the region in decimal degrees.
    pub longitude: f64,
    /// Region with population data.
    pub region: PopulationRegion,
}

/// Classification of a population centre as urban or rural.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AreaType {
    /// Urban area - typically cities and towns with >50k population.
    Urban,
    /// Rural area - countryside, villages, and small towns.
    Rural,
}

/// Represents a geographical region with population data.
/// 
/// Each region is non-overlapping and represents either an urban centre
/// or a rural district within a country.
#[derive(Clone, Debug)]
pub struct PopulationRegion {
    /// Central latitude of the region in decimal degrees.
    pub latitude: f64,
    /// Central longitude of the region in decimal degrees.
    pub longitude: f64,
    /// Total population of the region.
    pub population: f64,
    /// Area of the region in square kilometres.
    pub area_km2: f64,
    /// Whether this is an urban or rural region.
    pub area_type: AreaType,
    /// Name of the region (city name or district description).
    pub name: &'static str,
}

impl PopulationRegion {
    /// Creates a new population region.
    pub fn new(
        latitude:   f64,
        longitude:  f64,
        population: f64,
        area_km2:   f64,
        area_type:  AreaType,
        name:       &'static str,
    )
        -> Self
    {
        Self {
            latitude,
            longitude,
            population,
            area_km2,
            area_type,
            name,
        }
    }
    
    /// Calculates the population density (people per km²).
    pub fn density(&self) -> f64 {
        self.population / self.area_km2
    }
    
    /// Generates a random point within this region.
    /// 
    /// For urban areas, uses a concentrated distribution near the centre.
    /// For rural areas, uses a more spread out distribution.
    fn random_point_in_region(&self) -> (f64, f64) {
        // Calculate approximate radius from area (assuming roughly circular regions).
        let radius_km = (self.area_km2 / std::f64::consts::PI).sqrt();
        
        // Convert to degrees (rough approximation).
        // 1 degree latitude ≈ 111km.
        let radius_lat = radius_km / 111.0;
        let radius_lon = radius_km / (111.0 * self.latitude.to_radians().cos());
        
        // Generate point distribution based on area type.
        let (r, theta) = match self.area_type {
            AreaType::Urban => {
                // Urban: concentrated near centre (exponential decay).
                let r = -Rand::value::<f64>().ln() * 0.3; // Most within 30% of radius.
                let theta = Rand::value::<f64>() * 2.0 * std::f64::consts::PI;
                (r.min(1.0), theta)
            },
            AreaType::Rural => {
                // Rural: more uniform distribution.
                let r = Rand::value::<f64>().sqrt(); // Uniform in circle.
                let theta = Rand::value::<f64>() * 2.0 * std::f64::consts::PI;
                (r, theta)
            },
        };
        
        let lat_offset = r * radius_lat * theta.sin();
        let lon_offset = r * radius_lon * theta.cos();
        
        (self.latitude + lat_offset, self.longitude + lon_offset)
    }
}

/// Country population data including total statistics.
#[derive(Clone, Debug)]
pub struct CountryPopulationData {
    /// All regions in the country.
    pub regions: Vec<PopulationRegion>,
    /// Total country population (sum of all regions).
    pub total_population: f64,
    /// Total country area in km².
    pub total_area_km2: f64,
}

impl CountryPopulationData {
    /// Creates country data from a list of regions.
    pub fn from_regions(regions: Vec<PopulationRegion>, total_area_km2: f64) -> Self {
        let total_population = regions.iter().map(|r| r.population).sum();
        Self {
            regions,
            total_population,
            total_area_km2,
        }
    }
    
    /// Returns the percentage of population in urban areas.
    pub fn urban_population_percentage(&self) -> f64 {
        let urban_pop: f64 = self.regions
            .iter()
            .filter(|r| r.area_type == AreaType::Urban)
            .map(|r| r.population)
            .sum();
        (urban_pop / self.total_population) * 100.0
    }
}

#[derive(Debug, Display)]
pub enum CelestialBodies {
    Earth,
    Jupiter,
    Luna,
    Mars,
    Mercury,
    Saturn,
}
