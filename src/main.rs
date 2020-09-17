use ipa::Ipa;

fn main() {
    let ipa = Ipa::from_file(std::path::Path::new("ipa.yml")).unwrap();

    ipa.process().unwrap();
}
