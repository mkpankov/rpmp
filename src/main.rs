fn main() {
    use librpm::Index;

    let mut matches = Index::Name.find("rpm-devel");
    let package = matches.next();
    println!("{:?}", if package.is_some() {
        true
    } else {
        false
    });
}
