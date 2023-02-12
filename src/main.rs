use csv;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use rand::Rng;

struct Isbn {
    head_code: usize,
    country_code: usize,
    publisher_code: usize,
    publication_code: usize,
    check_digit_10: usize,
    check_digit_13: usize,
}

impl Isbn {
    fn new(head_code: usize, country_code: usize, publisher_code: usize) -> Self {
        let publication_code = Self::generate_publication_code(country_code, publisher_code);
        let check_digit_10 = Self::calc_check_digit_13(head_code, country_code, publisher_code, publication_code);
        let check_digit_13 = Self::calc_check_digit_13(head_code, country_code, publisher_code, publication_code);
        Isbn { head_code, country_code, publisher_code, publication_code, check_digit_10, check_digit_13 }
    }

    /// ISBNの書籍コードをランダムで生成する
    /// 書籍コードの桁数は10 - (国コード + 出版社コード + チェックディジット) で求められる
    fn generate_publication_code(country_code: usize, publisher_code: usize) -> usize {
        let country_code_digit = country_code.to_string().len();
        let publisher_code_digit = publisher_code.to_string().len();
        let publication_code_digit = 10 - (country_code_digit + publisher_code_digit + 1);

        // 書籍コードの桁数がわかったので、桁数+1分の100...の文字列を作る
        let mut max_publication_code_string = String::from("1");
        for i in 1..=publication_code_digit {
            max_publication_code_string.push_str("0");
        };
        let max_publication_code: usize = max_publication_code_string.parse().unwrap();

        let mut rng = rand::thread_rng();
        rng.gen_range(0..max_publication_code)
    }

    /// ISBN13のチェックディジットの計算
    fn calc_check_digit_13(head_code: usize, country_code: usize, publisher_code: usize, publication_code: usize) -> usize {
        let head_str = head_code.to_string();
        let country_str = country_code.to_string();
        let publisher_str = publisher_code.to_string();
        let publication_str = publication_code.to_string();

        let isbn_string_without_check_digit = String::new() + &head_str + &country_str + &publisher_str + &publication_str;
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
            even_total += num;
        };

        // チェックディジットの計算
        let check_digit_surplus = (odd_total + even_total) % 10;
        if check_digit_surplus == 0 {
            0
        } else {
            10 - check_digit_surplus
        }
    }

    /// ISBN10のチェックディジットの計算
    fn calc_check_digit_10(country_code: usize, publisher_code: usize, publication_code: usize) -> usize {
        let country_str = country_code.to_string();
        let publisher_str = publisher_code.to_string();
        let publication_str = publication_code.to_string();

        let isbn_string_without_check_digit = String::new() + &country_str + &publisher_str + &publication_str;
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
            even_total += num;
        };

        // チェックディジットの計算
        let check_digit_surplus = (odd_total + even_total) % 10;
        if (check_digit_surplus == 0) {
            0
        } else {
            10 - check_digit_surplus
        }
    }
}

fn main() {
    println!("Hello, world!");
}
