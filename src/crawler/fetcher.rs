use reqwest::{Error};

pub async  fn fetch_url(url:&str) -> Result<String,Error>{
   let resp =  reqwest::get(url).await?;
   let content = resp.text().await?;
   Ok(content)
}