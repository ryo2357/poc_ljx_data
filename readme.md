# データ処理検討

## 課題の列挙

### データ自体の処理

- 単位調整：μm×10程度の整数型にする？
- 傾き・位置補正
- 土手部のノイズ処理

### データのハンドリング

バイナリ自体からのデータローダーを実装、下記要求機能

- データ処理計測のための時間の管理
- LJXからの受信データで処理する際に大きな変更を必要としないこと

## 検証したコード

dust_codeディレクトリに名前を変えておいておく

### データローダー作成検討

自作のバイナリログのパースを行う構造体を実装しようとした

stdライブラリにてstreamのように順次処理するには以下が不備で実装することができなかった

- 指定バイトで区切って読み出しを行うには要素数を決めた配列を用意する必要がある  
  ``let buf: [u32; 3200] = [0; 3200];``配列数の指定には動的な値を入れれず、構造体にまとめる方法を思いつかなかった
- 区切り文字も特にないため、readerの標準メソッドでは処理できない。
- readerからVecに変換すると使い勝手は良くなるがメモリに全部乗る

データ構造をハードコーディングすれば読み出し処理はできそうであるが、汎用性はなくなる

保存形式について再検証が必要