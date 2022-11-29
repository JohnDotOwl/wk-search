// Project : Ads.txt Search Engine
// Feature : Parallel Request Code (Not Concurrent)
// Source Reference : https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest/51047786#51047786
// Parallel vs Concurrent : https://stackoverflow.com/questions/1897993/what-is-the-difference-between-concurrent-programming-and-parallel-programming

use futures::{stream, StreamExt}; // 0.3.8
// use reqwest::Client; // 0.10.9
use tokio; // 0.2.24, features = ["macros"]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;

//What is the size of usize
const PARALLEL_REQUESTS: usize = 50;

//What is this #[tokio::main]
#[tokio::main]
async fn main() {
    // We can call stathats/our own version of stathats too
    // What is Vector
    // What does the ! Represent
    // let mut urls = vec!["https://af.requestcatcher.com/","https://af2.requestcatcher.com/","https://mothership.sg/ads.txt"];
    let mut urls = Vec::new();

    if let Ok(lines) = read_lines("./domain.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(domain_name) = line {
                println!("{}", domain_name);
                urls.push(String::from("https://") + &domain_name + &String::from("/ads.txt"));
            }
        }
    }

    //Created Once then Cloned Later
    let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(10))
    .build();

    // let client = Client::new();

    let bodies = stream::iter(urls)
        .map(|url| {
            //How does Clone work in terms of memory usage. 
            //What if i dont clone but Client::new(); o.O 
            // let client = client.clone();

            // Ram will shoot up to 150MB++ if you do clone and throw the builder outside of the map
            let client = client.as_ref().expect("REASON").clone();
            // let client = reqwest::Client::builder()
            // .timeout(Duration::from_secs(10))
            // .build();

            tokio::spawn(async move {
                //wait with ? , why ah? 
                // let resp = client.expect("Error").get(url).send().await?;
                let resp = client.get(url).send().await?;
                resp.text().await
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS);


    //Memory in Ram will just keep growing until about 290MB-300MB 
    bodies.for_each(|b| async {
            match b {
                // Ok(Ok(b)) => {
                //     // println!("{:?}", b.find("google.com, pub-"))
                //     if b.find("google.com, pub-").is_some(){
                //         //Scan all lines
                //         let words: Vec<&str> = b.lines().collect();
                //         for i in &words {
                //             if i.find("google.com, pub-").is_some() && i.find("DIRECT").is_some(){
                //                 println!("{}", i);
                //                 break;
                //             }
                //         }
                //         // println!("{:?}", words);
                //     }else{
                //         // No update on DB required.
                //         // println!("No Ads.txt Data Found.");
                //     }
                // },
                Ok(Ok(b)) => println!("Got {} bytes", b.len()),
                Ok(Err(e)) => eprintln!("Got a reqwest::Error: {}", e),
                Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
            }
        }).await;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


// Single Thread Code , Non Concurrent & Non Parallel 
// May be useful for serverless request 
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // URL Source - API Call
//     // URL Source - Redis
//     // URL Source - Postgresql Rust Library
//     //Visit Website
//     let url = "https://mothership.sg/ads.txt";
//     let resp = reqwest::get(url)
//         .await?
//         .text()
//         .await?;
    
//     //Search for Google Ads.txt
//     println!("{:?}", resp.find("google.com, pub-"));
//     if resp.find("google.com, pub-").is_some(){
//         //Scan all lines
//         let words: Vec<&str> = resp.lines().collect();
//         for i in &words {
//             if i.find("google.com, pub-").is_some() && i.find("DIRECT").is_some(){
//                 println!("{}", i);
//             }
//         }
//         // println!("{:?}", words);
//     }else{
//         // No update on DB required.
//         println!("No Ads.txt Data Found.");
//     }
//     Ok(())
// }