# isbn
search random book from isbn code

```bash
# build
$ cargo run build

# exec command
$ ./target/debug/isbn
https://booklog.jp/item/1/4767985099
```

When you cannot find any books in 10 times, please try one more.
```bash
$ ./target/debug/isbn
9784796193801 ... not found
9784846343200 ... not found
9784568755930 ... not found
9784819962377 ... not found
9784752211679 ... not found
9784807379477 ... not found
9784754811952 ... not found
9784845301713 ... not found
9784846191207 ... not found
9784803801590 ... not found
9784804873015 ... not found
cannot find any books in 10 times
```