use curl::easy::{Easy};
use tokio;

mod crawl_page;
// mod blocking;


#[tokio::main]
async fn main() {

    let url = "https://example.com/";

    let destinations = crawl_and_save(url);

    println!("{:?}", destinations)
}

fn crawl_and_save(url: &str) -> Vec<String>{

    let mut destinations = vec![];
        
    //body
    let mut out_vec = Vec::new();

    {
        let mut curl = Easy::new();
        curl.url(url).unwrap();
        let mut transfer = curl.transfer();
        
        let _ = transfer.write_function(|bytes| {
            out_vec.extend_from_slice(bytes);
            return Ok(bytes.len())
        }).unwrap();
        
        transfer.perform().unwrap();
    }

    let byte_array: &[u8] = out_vec.as_slice();

    let _ = crawl_page::process_out(&byte_array, url, &mut destinations);

    return destinations;
}