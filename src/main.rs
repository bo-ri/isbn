use csv;
use serde::Deserialize;
use std::error::Error;
use rand::Rng;
use xmltree::Element;

#[derive(Debug)]
struct Isbn {
    head_code: String,
    country_code: String,
    publisher_code: String,
    publication_code: String,
    check_digit_10: String,
    check_digit_13: String,
}

impl Isbn {
    fn new(head_code: String, country_code: String, publisher_code: String) -> Self {
        let publication_code = Self::generate_publication_code(&country_code, &publisher_code);
        let check_digit_10 = Self::calc_check_digit_10(&country_code, &publisher_code, &publication_code);
        let check_digit_13 = Self::calc_check_digit_13(&head_code, &country_code, &publisher_code, &publication_code);
        Isbn { head_code, country_code, publisher_code, publication_code, check_digit_10, check_digit_13 }
    }

    /// ISBNの書籍コードをランダムで生成する
    /// 書籍コードの桁数は10 - (国コード + 出版社コード + チェックディジット) で求められる
    /// 必要な桁数に合わせて足りない桁数は0パディングする
    fn generate_publication_code(country_code: &String, publisher_code: &String) -> String {
        let country_code_digit = country_code.len();
        let publisher_code_digit = publisher_code.len();
        let publication_code_digit = 10 - (country_code_digit + publisher_code_digit + 1);

        // 書籍コードの桁数がわかったので、桁数+1分の100...の文字列を作る
        let mut max_publication_code_string = String::from("1");
        for _ in 1..=publication_code_digit {
            max_publication_code_string.push_str("0");
        };
        let max_publication_code: usize = max_publication_code_string.parse().unwrap();

        let mut rng = rand::thread_rng();
        let publication_code = rng.gen_range(0..max_publication_code).to_string();
        let digit_diff: usize = (max_publication_code_string.len() - 1) - publication_code.len();

        if digit_diff == 0 {
            publication_code
        } else {
            let mut padded_publication_code: String = String::from(&publication_code);
            for _ in 1..=digit_diff {
                padded_publication_code = String::from("0") + &padded_publication_code;
            };
            padded_publication_code
        }
    }

    /// ISBN13のチェックディジットの計算
    fn calc_check_digit_13(head_code: &String, country_code: &String, publisher_code: &String, publication_code: &String) -> String {
        let isbn_string_without_check_digit = String::new() + &head_code + &country_code + &publisher_code + &publication_code;
        // 奇数桁の合計
        let mut odd_total: usize = 0;
        for i in (0..isbn_string_without_check_digit.len()).step_by(2) {
            let num_char = isbn_string_without_check_digit.chars().nth(i).unwrap();
            let num = num_char as usize - 48;
            odd_total += num;
        };

        // 偶数桁の合計
        let mut even_total: usize = 0;
        for i in (1..isbn_string_without_check_digit.len()).step_by(2) {
            let num_char = isbn_string_without_check_digit.chars().nth(i).unwrap();
            let num = num_char as usize - 48;
            even_total += num * 3;
        };

        // チェックディジットの計算
        let check_digit_surplus = (odd_total + even_total) % 10;
        if check_digit_surplus == 0 {
            String::from("0")
        } else {
            (10 - check_digit_surplus).to_string()
        }
    }

    /// ISBN10のチェックディジットの計算
    fn calc_check_digit_10(country_code: &String, publisher_code: &String, publication_code: &String) -> String {
        let isbn_string_without_check_digit = String::new() + &country_code + &publisher_code + &publication_code;

        let mut total: usize = 0;
        for i in (0..isbn_string_without_check_digit.len()) {
            let num_chart = isbn_string_without_check_digit.chars().nth(i).unwrap();
            let num = num_chart as usize - 48;
            total += num * (10 - i);
        }

        // チェックディジットの計算
        let check_digit_surplus = total % 11;
        if check_digit_surplus == 0 {
            String::from("0")
        } else if check_digit_surplus == 1 {
            String::from("X")
        } else {
            (11 - check_digit_surplus).to_string()
        }
    }

    fn create_isbn_10(&self) -> String {
        String::new()
            + &self.country_code
            + &self.publisher_code
            + &self.publication_code
            + &self.check_digit_10
    }

    fn create_isbn_13(&self) -> String {
        String::new()
            + &self.head_code
            + &self.country_code
            + &self.publisher_code
            + &self.publication_code
            + &self.check_digit_13
    }
}

#[derive(Debug, Deserialize)]
struct Publisher {
    code: String,
    name: String,
}

fn read_csv() -> Result<Vec<Publisher>, Box<dyn Error>>{
    let mut publisher_list = Vec::new();
    // let csv_text = fs::read_to_string(file_path)?;
    let csv_text = include_str!("../csv/isbn978.csv");
    let mut rdr = csv::Reader::from_reader(csv_text.as_bytes());
    for result in rdr.records() {
        let record = result?.deserialize(None)?;
        publisher_list.push(record);
    }
    Ok(publisher_list)
}

async fn get_publication(client: &reqwest::Client, isbn: &String) -> reqwest::Result<String> {
    let response = client.get("https://iss.ndl.go.jp/api/opensearch?cnt=1&isbn=".to_string() + &isbn)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let mut counter = 0;
    loop {
        if counter > 10 {
            println!("cannot find any books in 10 times");
            break;
        }
        let publisher_list = read_csv().unwrap();
        let mut rng = rand::thread_rng();
        let publisher_code_index = rng.gen_range(0..publisher_list.len());

        let isbn: Isbn = Isbn::new(String::from("978"), String::from("4"), publisher_list[publisher_code_index].code.to_string());

        // reqwest
        let response_xml = get_publication(&client, &isbn.create_isbn_13()).await.unwrap();

        // parse xml
        let element = Element::parse(response_xml.as_bytes()).unwrap();
        let channel = element.get_child("channel").expect("cannot find channel in xml tree");
        let total_results: usize = (channel.get_child("totalResults").expect("cannot find totalResults in xml tree"))
            .children[0]
            .as_text()
            .unwrap()
            .parse()
            .unwrap();
        if total_results > 0 {
            // booklogのパスパラメータはISBN10
            println!("https://booklog.jp/item/1/{}", isbn.create_isbn_10());
            break;
        }
        println!("{} ... not found", isbn.create_isbn_13());
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        counter += 1;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pubalication_code() {
        // 最大桁数の場合(7桁)
        let country_code_7 = String::from("4");   // 日本
        let publisher_code_7 = String::from("1");  // 旺文社
        let publication_code7: String = Isbn::generate_publication_code(&country_code_7, &publisher_code_7);
        assert!(publication_code7.to_string().len() == 7);

        // 6桁の場合
        let country_code_6 = String::from("4");
        let publisher_code_6 = String::from("12");
        let publication_code6 = Isbn::generate_publication_code(&country_code_6, &publisher_code_6);
        assert!(publication_code6.len() == 6);

        // 5桁の場合
        let country_code_5 = String::from("4");
        let publisher_code_5 = String::from("123");
        let publication_code5 = Isbn::generate_publication_code(&country_code_5, &publisher_code_5);
        assert!(publication_code5.len() == 5);

        // 4桁の場合
        let country_code_4 = String::from("4");
        let publisher_code_4 = String::from("1234");
        let publication_code4 = Isbn::generate_publication_code(&country_code_4, &publisher_code_4);
        assert!(publication_code4.len() == 4);
    }

    #[test]
    fn test_calc_check_digit_10() {
        // 4-10-109205
        let country_code = String::from("4");
        let publisher_code = String::from("10");
        let publication_code = String::from("109205");

        let check_digit_10: String = Isbn::calc_check_digit_10(&country_code, &publisher_code, &publication_code);
        assert_eq!(check_digit_10, String::from("2"));
    }

    #[test]
    fn test_calc_check_digit_13() {
        // 978-4-7981-7154-8
        let head_code = String::from("978");
        let country_code = String::from("4");
        let publisher_code = String::from("7981");
        let publication_code = String::from("7154");
        let expected = String::from("8");

        let check_digit_13: String = Isbn::calc_check_digit_13(&head_code, &country_code, &publisher_code, &publication_code);
        assert_eq!(check_digit_13, expected);
    }

    #[test]
    fn test_create_isbn_10() {
        let isbn = Isbn::new(String::from("978"), String::from("4"), String::from("10"));
        assert!(isbn.create_isbn_10().len() == 10);
    }

    #[test]
    fn test_create_isbn_13() {
        let isbn = Isbn::new(String::from("978"), String::from("4"), String::from("10"));
        assert!(isbn.create_isbn_13().len() == 13);
    }
}