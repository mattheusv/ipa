use ipa::Ipa;

fn main() {
    let ipa = Ipa::new(std::path::Path::new("ipa.yml")).unwrap();

    ipa.process().unwrap();
}
