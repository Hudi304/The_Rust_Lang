

const URL: &str  = "https://natcom-api-development.azurewebsites.net/swagger/v1/swagger.json";

#[tokio::main]
async fn main() {
    println!("Hello, cargo!");
    let body = reqwest::get(URL);

    let awaited_feature = match  body.await {
        Ok(result) => {
                return ()
            },
        Err(error) => {
            return ()
            
            }
        }; 

    // let response_feature =  match awaited_feature.await{
    //     Ok(result)=>  result,
    //     Err(error) => {
    //         println!("panic2");
    //         panic!("Something went wrong : {:?}", error) 
    //     }
    // };


    // for word in response_feature.lines() {
    //     println!("> {}", word);
    // }

    // let slipt_result =  response_feature.split("\n");


    // let obj = match json::parse(&response_feature) {
    //     Ok(result)=>  result,
    //     Err(error) => panic!("Something went wrong : {:?}", error)

    // }; 


    // println!("{}", obj);

    // for word in slipt_result {
    //     println!("{}", word)
    // }
    // println!("body = {:?}", response_feature);
}
