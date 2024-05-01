use postgres::Client;
use postgres::NoTls;
fn main() {
    #[warn(unused_mut)]
    let mut client= Client::connect("postgresql://liuziling:a2sh6817@localhost/pgdb", NoTls);
}
