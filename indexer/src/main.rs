mod dictionary;
mod crawled_page;
mod database;
mod config;

fn main() {
    let conf = config::Config::read_from_file("../config.yaml");
    let database: &mut dyn database::Database = &mut database::PostgresDatabase::new(&conf.database);

    let crawled_page = database.get_crawled_page();

    println!("crawled_page: {:?}\n", crawled_page);

    index();
}

fn index() {
    // get page from backend as crawled page

    // convert to indexed page

    // put back in database

}