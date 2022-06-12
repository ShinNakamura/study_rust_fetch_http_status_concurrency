# 複数のURLに対して並行でHTTP Statusを取得する

URLが改行区切りで書いてあるテキストが用意してあり、

こんな
```txt
https://foo.com
https://bar.com
https://baz.com
```

これを標準入力から読み込んで、それぞれのHTTP statusを確認し、
`url,status`のCSV形式で標準出力に返す。

```sh
<url_list.txt cargo run --
# 次のように出力
url,status
https://foo.com,200
https://bar.com,500
https://baz.com,400
```

あまり一気にリクエストするとよくないので
リクエストのタスクの中にスリープを仕込んで、
ある程度リクエストの勢いが緩くなるようにしてある。
