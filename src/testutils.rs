// SPDX-License-Identifier: Apache-2.0 or MIT

//! Utility functions for testing.

#![cfg(test)]

use std::fmt::Debug;

use pretty_assertions::assert_eq;
use serde::{de::DeserializeOwned, Serialize};

macro_rules! test_serde_roundtrip_prop {
    ($t:ty) => {
        paste::paste! {
            #[test]
            fn [<test_ $t:snake _serde_roundtrip_prop>]() {
                use arbitrary::Arbitrary as _;
                fn prop(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<()> {
                    let obj = $t::arbitrary(u)?;
                    crate::testutils::assert_serde_roundtrip(obj);
                    Ok(())
                }
                arbtest::builder().run(prop);
            }
        }
    };
}
pub(crate) use test_serde_roundtrip_prop;

macro_rules! test_json {
    ($t:ty, $file:expr, $testcase:ident) => {
        paste::paste! {
            #[test]
            fn [<test_ $t:snake _ $testcase _json>]() {
                let json_str = include_str!($file);
                let obj: $t = serde_json::from_str(json_str).expect("should deserialize into $t");
                crate::testutils::assert_serde_roundtrip(obj)
            }
        }
    };
}
pub(crate) use test_json;

macro_rules! test_enum_to_string {
    ($t:ident) => {
        paste::paste! {
            #[test]
            fn [<test_ $t:snake _to_string>]() {
                for variant in $t::VARIANTS {
                    let serialized = serde_json::to_string(variant).expect("should serialize");
                    let serialized = serialized.replace("\"", "");
                    assert_eq!(
                        serialized,
                        variant.to_string(),
                        "$t: {variant} string representation should match JSON serialization"
                    );
                }
            }
        }
    };
}
pub(crate) use test_enum_to_string;

pub fn assert_serde_roundtrip<T>(obj: T)
where
    T: Serialize + DeserializeOwned + Debug + PartialEq,
{
    let obj_str = serde_json::to_string(&obj).expect("obj should serialize");
    let obj_de: T = serde_json::from_str(&obj_str).expect("obj should deserialize");
    assert_eq!(obj, obj_de, "serialization should be idempotent");
}

pub mod arbitrary_with {
    use isocountry::CountryCode;

    pub fn option_country(
        u: &mut arbitrary::Unstructured,
    ) -> arbitrary::Result<Option<CountryCode>> {
        u.choose(&[
            None,
            Some(CountryCode::AFG),
            Some(CountryCode::ALA),
            Some(CountryCode::ALB),
            Some(CountryCode::DZA),
            Some(CountryCode::ASM),
            Some(CountryCode::AND),
            Some(CountryCode::AGO),
            Some(CountryCode::AIA),
            Some(CountryCode::ATA),
            Some(CountryCode::ATG),
            Some(CountryCode::ARG),
            Some(CountryCode::ARM),
            Some(CountryCode::ABW),
            Some(CountryCode::AUS),
            Some(CountryCode::AUT),
            Some(CountryCode::AZE),
            Some(CountryCode::BHS),
            Some(CountryCode::BHR),
            Some(CountryCode::BGD),
            Some(CountryCode::BRB),
            Some(CountryCode::BLR),
            Some(CountryCode::BEL),
            Some(CountryCode::BLZ),
            Some(CountryCode::BEN),
            Some(CountryCode::BMU),
            Some(CountryCode::BTN),
            Some(CountryCode::BOL),
            Some(CountryCode::BES),
            Some(CountryCode::BIH),
            Some(CountryCode::BWA),
            Some(CountryCode::BVT),
            Some(CountryCode::BRA),
            Some(CountryCode::IOT),
            Some(CountryCode::BRN),
            Some(CountryCode::BGR),
            Some(CountryCode::BFA),
            Some(CountryCode::BDI),
            Some(CountryCode::CPV),
            Some(CountryCode::KHM),
            Some(CountryCode::CMR),
            Some(CountryCode::CAN),
            Some(CountryCode::CYM),
            Some(CountryCode::CAF),
            Some(CountryCode::TCD),
            Some(CountryCode::CHL),
            Some(CountryCode::CHN),
            Some(CountryCode::CXR),
            Some(CountryCode::CCK),
            Some(CountryCode::COL),
            Some(CountryCode::COM),
            Some(CountryCode::COG),
            Some(CountryCode::COD),
            Some(CountryCode::COK),
            Some(CountryCode::CRI),
            Some(CountryCode::CIV),
            Some(CountryCode::HRV),
            Some(CountryCode::CUB),
            Some(CountryCode::CUW),
            Some(CountryCode::CYP),
            Some(CountryCode::CZE),
            Some(CountryCode::DNK),
            Some(CountryCode::DJI),
            Some(CountryCode::DMA),
            Some(CountryCode::DOM),
            Some(CountryCode::ECU),
            Some(CountryCode::EGY),
            Some(CountryCode::SLV),
            Some(CountryCode::GNQ),
            Some(CountryCode::ERI),
            Some(CountryCode::EST),
            Some(CountryCode::ETH),
            Some(CountryCode::FLK),
            Some(CountryCode::FRO),
            Some(CountryCode::FJI),
            Some(CountryCode::FIN),
            Some(CountryCode::FRA),
            Some(CountryCode::GUF),
            Some(CountryCode::PYF),
            Some(CountryCode::ATF),
            Some(CountryCode::GAB),
            Some(CountryCode::GMB),
            Some(CountryCode::GEO),
            Some(CountryCode::DEU),
            Some(CountryCode::GHA),
            Some(CountryCode::GIB),
            Some(CountryCode::GRC),
            Some(CountryCode::GRL),
            Some(CountryCode::GRD),
            Some(CountryCode::GLP),
            Some(CountryCode::GUM),
            Some(CountryCode::GTM),
            Some(CountryCode::GGY),
            Some(CountryCode::GIN),
            Some(CountryCode::GNB),
            Some(CountryCode::GUY),
            Some(CountryCode::HTI),
            Some(CountryCode::HMD),
            Some(CountryCode::VAT),
            Some(CountryCode::HND),
            Some(CountryCode::HKG),
            Some(CountryCode::HUN),
            Some(CountryCode::ISL),
            Some(CountryCode::IND),
            Some(CountryCode::IDN),
            Some(CountryCode::IRN),
            Some(CountryCode::IRQ),
            Some(CountryCode::IRL),
            Some(CountryCode::IMN),
            Some(CountryCode::ISR),
            Some(CountryCode::ITA),
            Some(CountryCode::JAM),
            Some(CountryCode::JPN),
            Some(CountryCode::JEY),
            Some(CountryCode::JOR),
            Some(CountryCode::KAZ),
            Some(CountryCode::KEN),
            Some(CountryCode::KIR),
            Some(CountryCode::PRK),
            Some(CountryCode::KOR),
            Some(CountryCode::KWT),
            Some(CountryCode::KGZ),
            Some(CountryCode::LAO),
            Some(CountryCode::LVA),
            Some(CountryCode::LBN),
            Some(CountryCode::LSO),
            Some(CountryCode::LBR),
            Some(CountryCode::LBY),
            Some(CountryCode::LIE),
            Some(CountryCode::LTU),
            Some(CountryCode::LUX),
            Some(CountryCode::MAC),
            Some(CountryCode::MKD),
            Some(CountryCode::MDG),
            Some(CountryCode::MWI),
            Some(CountryCode::MYS),
            Some(CountryCode::MDV),
            Some(CountryCode::MLI),
            Some(CountryCode::MLT),
            Some(CountryCode::MHL),
            Some(CountryCode::MTQ),
            Some(CountryCode::MRT),
            Some(CountryCode::MUS),
            Some(CountryCode::MYT),
            Some(CountryCode::MEX),
            Some(CountryCode::FSM),
            Some(CountryCode::MDA),
            Some(CountryCode::MCO),
            Some(CountryCode::MNG),
            Some(CountryCode::MNE),
            Some(CountryCode::MSR),
            Some(CountryCode::MAR),
            Some(CountryCode::MOZ),
            Some(CountryCode::MMR),
            Some(CountryCode::NAM),
            Some(CountryCode::NRU),
            Some(CountryCode::NPL),
            Some(CountryCode::NLD),
            Some(CountryCode::NCL),
            Some(CountryCode::NZL),
            Some(CountryCode::NIC),
            Some(CountryCode::NER),
            Some(CountryCode::NGA),
            Some(CountryCode::NIU),
            Some(CountryCode::NFK),
            Some(CountryCode::MNP),
            Some(CountryCode::NOR),
            Some(CountryCode::OMN),
            Some(CountryCode::PAK),
            Some(CountryCode::PLW),
            Some(CountryCode::PSE),
            Some(CountryCode::PAN),
            Some(CountryCode::PNG),
            Some(CountryCode::PRY),
            Some(CountryCode::PER),
            Some(CountryCode::PHL),
            Some(CountryCode::PCN),
            Some(CountryCode::POL),
            Some(CountryCode::PRT),
            Some(CountryCode::PRI),
            Some(CountryCode::QAT),
            Some(CountryCode::REU),
            Some(CountryCode::ROU),
            Some(CountryCode::RUS),
            Some(CountryCode::RWA),
            Some(CountryCode::BLM),
            Some(CountryCode::SHN),
            Some(CountryCode::KNA),
            Some(CountryCode::LCA),
            Some(CountryCode::MAF),
            Some(CountryCode::SPM),
            Some(CountryCode::VCT),
            Some(CountryCode::WSM),
            Some(CountryCode::SMR),
            Some(CountryCode::STP),
            Some(CountryCode::SAU),
            Some(CountryCode::SEN),
            Some(CountryCode::SRB),
            Some(CountryCode::SYC),
            Some(CountryCode::SLE),
            Some(CountryCode::SGP),
            Some(CountryCode::SXM),
            Some(CountryCode::SVK),
            Some(CountryCode::SVN),
            Some(CountryCode::SLB),
            Some(CountryCode::SOM),
            Some(CountryCode::ZAF),
            Some(CountryCode::SGS),
            Some(CountryCode::SSD),
            Some(CountryCode::ESP),
            Some(CountryCode::LKA),
            Some(CountryCode::SDN),
            Some(CountryCode::SUR),
            Some(CountryCode::SJM),
            Some(CountryCode::SWZ),
            Some(CountryCode::SWE),
            Some(CountryCode::CHE),
            Some(CountryCode::SYR),
            Some(CountryCode::TWN),
            Some(CountryCode::TJK),
            Some(CountryCode::TZA),
            Some(CountryCode::THA),
            Some(CountryCode::TLS),
            Some(CountryCode::TGO),
            Some(CountryCode::TKL),
            Some(CountryCode::TON),
            Some(CountryCode::TTO),
            Some(CountryCode::TUN),
            Some(CountryCode::TUR),
            Some(CountryCode::TKM),
            Some(CountryCode::TCA),
            Some(CountryCode::TUV),
            Some(CountryCode::UGA),
            Some(CountryCode::UKR),
            Some(CountryCode::ARE),
            Some(CountryCode::GBR),
            Some(CountryCode::USA),
            Some(CountryCode::UMI),
            Some(CountryCode::URY),
            Some(CountryCode::UZB),
            Some(CountryCode::VUT),
            Some(CountryCode::VEN),
            Some(CountryCode::VNM),
            Some(CountryCode::VGB),
            Some(CountryCode::VIR),
            Some(CountryCode::WLF),
            Some(CountryCode::ESH),
            Some(CountryCode::YEM),
            Some(CountryCode::ZMB),
            Some(CountryCode::ZWE),
        ])
        .cloned()
    }

    pub fn clamped_option_f64(
        min: f64,
        max: f64,
    ) -> impl Fn(&mut arbitrary::Unstructured) -> arbitrary::Result<Option<f64>> {
        move |u: &mut arbitrary::Unstructured| -> arbitrary::Result<Option<f64>> {
            let steps = u32::MAX;
            let factor = (max - min) / (steps as f64);
            let random_int: u32 = u.int_in_range(0..=steps)?;
            let random = min + factor * (random_int as f64);
            Ok(Some(random))
        }
    }
}
