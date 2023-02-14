use csv;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use rand::Rng;

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
        for i in 1..=publication_code_digit {
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
            for i in 1..=digit_diff {
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
        if (check_digit_surplus == 0) {
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

fn main() {
    println!("Hello, world!");
}
