use crate::params::to_snakecase;
use serde_derive::{Deserialize, Serialize};

/// Currency is the list of supported currencies.
///
/// For more details see https://support.stripe.com/questions/which-currencies-does-stripe-support.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Currency {
    #[serde(rename = "BTC")]
    BTC, // Bitcoin
    #[serde(rename = "ETH")]
    ETH, // Ethereum
    #[serde(rename = "XMR")]
    XMR, // Monero
    #[serde(rename = "AED")]
    AED, // United Arab Emirates Dirham
    #[serde(rename = "AFN")]
    AFN, // Afghan Afghani
    #[serde(rename = "ALL")]
    ALL, // Albanian Lek
    #[serde(rename = "AMD")]
    AMD, // Armenian Dram
    #[serde(rename = "ANG")]
    ANG, // Netherlands Antillean Gulden
    #[serde(rename = "AOA")]
    AOA, // Angolan Kwanza
    #[serde(rename = "ARS")]
    ARS, // Argentine Peso
    #[serde(rename = "AUD")]
    AUD, // Australian Dollar
    #[serde(rename = "AWG")]
    AWG, // Aruban Florin
    #[serde(rename = "AZN")]
    AZN, // Azerbaijani Manat
    #[serde(rename = "BAM")]
    BAM, // Bosnia & Herzegovina Convertible Mark
    #[serde(rename = "BBD")]
    BBD, // Barbadian Dollar
    #[serde(rename = "BDT")]
    BDT, // Bangladeshi Taka
    #[serde(rename = "BGN")]
    BGN, // Bulgarian Lev
    #[serde(rename = "BIF")]
    BIF, // Burundian Franc
    #[serde(rename = "BMD")]
    BMD, // Bermudian Dollar
    #[serde(rename = "BND")]
    BND, // Brunei Dollar
    #[serde(rename = "BOB")]
    BOB, // Bolivian Boliviano
    #[serde(rename = "BRL")]
    BRL, // Brazilian Real
    #[serde(rename = "BSD")]
    BSD, // Bahamian Dollar
    #[serde(rename = "BWP")]
    BWP, // Botswana Pula
    #[serde(rename = "BZD")]
    BZD, // Belize Dollar
    #[serde(rename = "CAD")]
    CAD, // Canadian Dollar
    #[serde(rename = "CDF")]
    CDF, // Congolese Franc
    #[serde(rename = "CHF")]
    CHF, // Swiss Franc
    #[serde(rename = "CLP")]
    CLP, // Chilean Peso
    #[serde(rename = "CNY")]
    CNY, // Chinese Renminbi Yuan
    #[serde(rename = "COP")]
    COP, // Colombian Peso
    #[serde(rename = "CRC")]
    CRC, // Costa Rican Colón
    #[serde(rename = "CVE")]
    CVE, // Cape Verdean Escudo
    #[serde(rename = "CZK")]
    CZK, // Czech Koruna
    #[serde(rename = "DJF")]
    DJF, // Djiboutian Franc
    #[serde(rename = "DKK")]
    DKK, // Danish Krone
    #[serde(rename = "DOP")]
    DOP, // Dominican Peso
    #[serde(rename = "DZD")]
    DZD, // Algerian Dinar
    #[serde(rename = "EEK")]
    EEK, // Estonian Kroon
    #[serde(rename = "EGP")]
    EGP, // Egyptian Pound
    #[serde(rename = "ETB")]
    ETB, // Ethiopian Birr
    #[serde(rename = "EUR")]
    EUR, // Euro
    #[serde(rename = "FJD")]
    FJD, // Fijian Dollar
    #[serde(rename = "FKP")]
    FKP, // Falkland Islands Pound
    #[serde(rename = "GBP")]
    GBP, // British Pound
    #[serde(rename = "GEL")]
    GEL, // Georgian Lari
    #[serde(rename = "GIP")]
    GIP, // Gibraltar Pound
    #[serde(rename = "GMD")]
    GMD, // Gambian Dalasi
    #[serde(rename = "GNF")]
    GNF, // Guinean Franc
    #[serde(rename = "GTQ")]
    GTQ, // Guatemalan Quetzal
    #[serde(rename = "GYD")]
    GYD, // Guyanese Dollar
    #[serde(rename = "HKD")]
    HKD, // Hong Kong Dollar
    #[serde(rename = "HNL")]
    HNL, // Honduran Lempira
    #[serde(rename = "HRK")]
    HRK, // Croatian Kuna
    #[serde(rename = "HTG")]
    HTG, // Haitian Gourde
    #[serde(rename = "HUF")]
    HUF, // Hungarian Forint
    #[serde(rename = "IDR")]
    IDR, // Indonesian Rupiah
    #[serde(rename = "ILS")]
    ILS, // Israeli New Sheqel
    #[serde(rename = "INR")]
    INR, // Indian Rupee
    #[serde(rename = "ISK")]
    ISK, // Icelandic Króna
    #[serde(rename = "JMD")]
    JMD, // Jamaican Dollar
    #[serde(rename = "JPY")]
    JPY, // Japanese Yen
    #[serde(rename = "KES")]
    KES, // Kenyan Shilling
    #[serde(rename = "KGS")]
    KGS, // Kyrgyzstani Som
    #[serde(rename = "KHR")]
    KHR, // Cambodian Riel
    #[serde(rename = "KMF")]
    KMF, // Comorian Franc
    #[serde(rename = "KRW")]
    KRW, // South Korean Won
    #[serde(rename = "KYD")]
    KYD, // Cayman Islands Dollar
    #[serde(rename = "KZT")]
    KZT, // Kazakhstani Tenge
    #[serde(rename = "LAK")]
    LAK, // Lao Kip
    #[serde(rename = "LPG")]
    LBP, // Lebanese Pound
    #[serde(rename = "LKR")]
    LKR, // Sri Lankan Rupee
    #[serde(rename = "LRD")]
    LRD, // Liberian Dollar
    #[serde(rename = "LSL")]
    LSL, // Lesotho Loti
    #[serde(rename = "LTL")]
    LTL, // Lithuanian Litas
    #[serde(rename = "LVL")]
    LVL, // Latvian Lats
    #[serde(rename = "MAD")]
    MAD, // Moroccan Dirham
    #[serde(rename = "MDL")]
    MDL, // Moldovan Leu
    #[serde(rename = "MGA")]
    MGA, // Malagasy Ariary
    #[serde(rename = "MKD")]
    MKD, // Macedonian Denar
    #[serde(rename = "MNT")]
    MNT, // Mongolian Tögrög
    #[serde(rename = "MOP")]
    MOP, // Macanese Pataca
    #[serde(rename = "MRO")]
    MRO, // Mauritanian Ouguiya
    #[serde(rename = "MUR")]
    MUR, // Mauritian Rupee
    #[serde(rename = "MVR")]
    MVR, // Maldivian Rufiyaa
    #[serde(rename = "MWK")]
    MWK, // Malawian Kwacha
    #[serde(rename = "MXN")]
    MXN, // Mexican Peso
    #[serde(rename = "MYR")]
    MYR, // Malaysian Ringgit
    #[serde(rename = "MZN")]
    MZN, // Mozambican Metical
    #[serde(rename = "NAD")]
    NAD, // Namibian Dollar
    #[serde(rename = "NGN")]
    NGN, // Nigerian Naira
    #[serde(rename = "NIO")]
    NIO, // Nicaraguan Córdoba
    #[serde(rename = "NOK")]
    NOK, // Norwegian Krone
    #[serde(rename = "NPR")]
    NPR, // Nepalese Rupee
    #[serde(rename = "NZD")]
    NZD, // New Zealand Dollar
    #[serde(rename = "PAB")]
    PAB, // Panamanian Balboa
    #[serde(rename = "PEN")]
    PEN, // Peruvian Nuevo Sol
    #[serde(rename = "PGK")]
    PGK, // Papua New Guinean Kina
    #[serde(rename = "PHP")]
    PHP, // Philippine Peso
    #[serde(rename = "PKR")]
    PKR, // Pakistani Rupee
    #[serde(rename = "PLN")]
    PLN, // Polish Złoty
    #[serde(rename = "PYG")]
    PYG, // Paraguayan Guaraní
    #[serde(rename = "QAR")]
    QAR, // Qatari Riyal
    #[serde(rename = "RON")]
    RON, // Romanian Leu
    #[serde(rename = "RSD")]
    RSD, // Serbian Dinar
    #[serde(rename = "RUB")]
    RUB, // Russian Ruble
    #[serde(rename = "RWF")]
    RWF, // Rwandan Franc
    #[serde(rename = "SAR")]
    SAR, // Saudi Riyal
    #[serde(rename = "SBD")]
    SBD, // Solomon Islands Dollar
    #[serde(rename = "SCR")]
    SCR, // Seychellois Rupee
    #[serde(rename = "SEK")]
    SEK, // Swedish Krona
    #[serde(rename = "SGD")]
    SGD, // Singapore Dollar
    #[serde(rename = "SHP")]
    SHP, // Saint Helenian Pound
    #[serde(rename = "SLL")]
    SLL, // Sierra Leonean Leone
    #[serde(rename = "SOS")]
    SOS, // Somali Shilling
    #[serde(rename = "SRD")]
    SRD, // Surinamese Dollar
    #[serde(rename = "STD")]
    STD, // São Tomé and Príncipe Dobra
    #[serde(rename = "SVC")]
    SVC, // Salvadoran Colón
    #[serde(rename = "SZL")]
    SZL, // Swazi Lilangeni
    #[serde(rename = "THB")]
    THB, // Thai Baht
    #[serde(rename = "TJS")]
    TJS, // Tajikistani Somoni
    #[serde(rename = "TOP")]
    TOP, // Tongan Paʻanga
    #[serde(rename = "TRY")]
    TRY, // Turkish Lira
    #[serde(rename = "TTD")]
    TTD, // Trinidad and Tobago Dollar
    #[serde(rename = "TWD")]
    TWD, // New Taiwan Dollar
    #[serde(rename = "TZS")]
    TZS, // Tanzanian Shilling
    #[serde(rename = "UAH")]
    UAH, // Ukrainian Hryvnia
    #[serde(rename = "UGX")]
    UGX, // Ugandan Shilling
    #[serde(rename = "USD")]
    USD, // United States Dollar
    #[serde(rename = "UYU")]
    UYU, // Uruguayan Peso
    #[serde(rename = "UZS")]
    UZS, // Uzbekistani Som
    #[serde(rename = "VEF")]
    VEF, // Venezuelan Bolívar
    #[serde(rename = "VND")]
    VND, // Vietnamese Đồng
    #[serde(rename = "VUV")]
    VUV, // Vanuatu Vatu
    #[serde(rename = "WST")]
    WST, // Samoan Tala
    #[serde(rename = "XAF")]
    XAF, // Central African Cfa Franc
    #[serde(rename = "XCD")]
    XCD, // East Caribbean Dollar
    #[serde(rename = "XOF")]
    XOF, // West African Cfa Franc
    #[serde(rename = "XPF")]
    XPF, // Cfp Franc
    #[serde(rename = "YER")]
    YER, // Yemeni Rial
    #[serde(rename = "ZAR")]
    ZAR, // South African Rand
    #[serde(rename = "ZMW")]
    ZMW, // Zambian Kwacha
}

impl Default for Currency {
    fn default() -> Self {
        Currency::USD
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_snakecase(&format!("{:?}", self)))
    }
}

impl std::str::FromStr for Currency {
    type Err = ParseCurrencyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "btc" => Ok(Currency::BTC),
            "eth" => Ok(Currency::ETH),
            "xmr" => Ok(Currency::XMR),
            //
            "aed" => Ok(Currency::AED),
            "afn" => Ok(Currency::AFN),
            "all" => Ok(Currency::ALL),
            "amd" => Ok(Currency::AMD),
            "ang" => Ok(Currency::ANG),
            "aoa" => Ok(Currency::AOA),
            "ars" => Ok(Currency::ARS),
            "aud" => Ok(Currency::AUD),
            "awg" => Ok(Currency::AWG),
            "azn" => Ok(Currency::AZN),
            "bam" => Ok(Currency::BAM),
            "bbd" => Ok(Currency::BBD),
            "bdt" => Ok(Currency::BDT),
            "bgn" => Ok(Currency::BGN),
            "bif" => Ok(Currency::BIF),
            "bmd" => Ok(Currency::BMD),
            "bnd" => Ok(Currency::BND),
            "bob" => Ok(Currency::BOB),
            "brl" => Ok(Currency::BRL),
            "bsd" => Ok(Currency::BSD),
            "bwp" => Ok(Currency::BWP),
            "bzd" => Ok(Currency::BZD),
            "cad" => Ok(Currency::CAD),
            "cdf" => Ok(Currency::CDF),
            "chf" => Ok(Currency::CHF),
            "clp" => Ok(Currency::CLP),
            "cny" => Ok(Currency::CNY),
            "cop" => Ok(Currency::COP),
            "crc" => Ok(Currency::CRC),
            "cve" => Ok(Currency::CVE),
            "czk" => Ok(Currency::CZK),
            "djf" => Ok(Currency::DJF),
            "dkk" => Ok(Currency::DKK),
            "dop" => Ok(Currency::DOP),
            "dzd" => Ok(Currency::DZD),
            "eek" => Ok(Currency::EEK),
            "egp" => Ok(Currency::EGP),
            "etb" => Ok(Currency::ETB),
            "eur" => Ok(Currency::EUR),
            "fjd" => Ok(Currency::FJD),
            "fkp" => Ok(Currency::FKP),
            "gbp" => Ok(Currency::GBP),
            "gel" => Ok(Currency::GEL),
            "gip" => Ok(Currency::GIP),
            "gmd" => Ok(Currency::GMD),
            "gnf" => Ok(Currency::GNF),
            "gtq" => Ok(Currency::GTQ),
            "gyd" => Ok(Currency::GYD),
            "hkd" => Ok(Currency::HKD),
            "hnl" => Ok(Currency::HNL),
            "hrk" => Ok(Currency::HRK),
            "htg" => Ok(Currency::HTG),
            "huf" => Ok(Currency::HUF),
            "idr" => Ok(Currency::IDR),
            "ils" => Ok(Currency::ILS),
            "inr" => Ok(Currency::INR),
            "isk" => Ok(Currency::ISK),
            "jmd" => Ok(Currency::JMD),
            "jpy" => Ok(Currency::JPY),
            "kes" => Ok(Currency::KES),
            "kgs" => Ok(Currency::KGS),
            "khr" => Ok(Currency::KHR),
            "kmf" => Ok(Currency::KMF),
            "krw" => Ok(Currency::KRW),
            "kyd" => Ok(Currency::KYD),
            "kzt" => Ok(Currency::KZT),
            "lak" => Ok(Currency::LAK),
            "lbp" => Ok(Currency::LBP),
            "lkr" => Ok(Currency::LKR),
            "lrd" => Ok(Currency::LRD),
            "lsl" => Ok(Currency::LSL),
            "ltl" => Ok(Currency::LTL),
            "lvl" => Ok(Currency::LVL),
            "mad" => Ok(Currency::MAD),
            "mdl" => Ok(Currency::MDL),
            "mga" => Ok(Currency::MGA),
            "mkd" => Ok(Currency::MKD),
            "mnt" => Ok(Currency::MNT),
            "mop" => Ok(Currency::MOP),
            "mro" => Ok(Currency::MRO),
            "mur" => Ok(Currency::MUR),
            "mvr" => Ok(Currency::MVR),
            "mwk" => Ok(Currency::MWK),
            "mxn" => Ok(Currency::MXN),
            "myr" => Ok(Currency::MYR),
            "mzn" => Ok(Currency::MZN),
            "nad" => Ok(Currency::NAD),
            "ngn" => Ok(Currency::NGN),
            "nio" => Ok(Currency::NIO),
            "nok" => Ok(Currency::NOK),
            "npr" => Ok(Currency::NPR),
            "nzd" => Ok(Currency::NZD),
            "pab" => Ok(Currency::PAB),
            "pen" => Ok(Currency::PEN),
            "pgk" => Ok(Currency::PGK),
            "php" => Ok(Currency::PHP),
            "pkr" => Ok(Currency::PKR),
            "pln" => Ok(Currency::PLN),
            "pyg" => Ok(Currency::PYG),
            "qar" => Ok(Currency::QAR),
            "ron" => Ok(Currency::RON),
            "rsd" => Ok(Currency::RSD),
            "rub" => Ok(Currency::RUB),
            "rwf" => Ok(Currency::RWF),
            "sar" => Ok(Currency::SAR),
            "sbd" => Ok(Currency::SBD),
            "scr" => Ok(Currency::SCR),
            "sek" => Ok(Currency::SEK),
            "sgd" => Ok(Currency::SGD),
            "shp" => Ok(Currency::SHP),
            "sll" => Ok(Currency::SLL),
            "sos" => Ok(Currency::SOS),
            "srd" => Ok(Currency::SRD),
            "std" => Ok(Currency::STD),
            "svc" => Ok(Currency::SVC),
            "szl" => Ok(Currency::SZL),
            "thb" => Ok(Currency::THB),
            "tjs" => Ok(Currency::TJS),
            "top" => Ok(Currency::TOP),
            "try" => Ok(Currency::TRY),
            "ttd" => Ok(Currency::TTD),
            "twd" => Ok(Currency::TWD),
            "tzs" => Ok(Currency::TZS),
            "uah" => Ok(Currency::UAH),
            "ugx" => Ok(Currency::UGX),
            "usd" => Ok(Currency::USD),
            "uyu" => Ok(Currency::UYU),
            "uzs" => Ok(Currency::UZS),
            "vef" => Ok(Currency::VEF),
            "vnd" => Ok(Currency::VND),
            "vuv" => Ok(Currency::VUV),
            "wst" => Ok(Currency::WST),
            "xaf" => Ok(Currency::XAF),
            "xcd" => Ok(Currency::XCD),
            "xof" => Ok(Currency::XOF),
            "xpf" => Ok(Currency::XPF),
            "yer" => Ok(Currency::YER),
            "zar" => Ok(Currency::ZAR),
            "zmw" => Ok(Currency::ZMW),
            _ => Err(ParseCurrencyError(())),
        }
    }
}

#[derive(Debug)]
pub struct ParseCurrencyError(/* private */ ());

impl std::fmt::Display for ParseCurrencyError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(::std::error::Error::description(self))
    }
}

impl std::error::Error for ParseCurrencyError {
    fn description(&self) -> &str {
        "unknown currency code"
    }
}
