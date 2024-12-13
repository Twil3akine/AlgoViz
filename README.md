<h1 id="Top">✌AlgoViz's Doc✌</h1>

<br>

## AlgoVizについて
TUIで動くアルゴリズム可視化ソフトウェアです。純Rust製です。動作には`Cargo`が必要です。

<br>

## 使用技術
<p style="display: inline">
    <img src="https://img.shields.io/badge/-Rust-000000.svg?logo=rust&style=flat-square">
</p>

<br><br>

## 開発環境セットアップ

リポジトリ (https://github.com/twil3akine/algoviz) をクローン

```bash
# sshを使用しない場合
git clone https://github.com/twil3akine/AlgoViz.git

# sshを使用する場合
git clone git@github.com:twil3akine/AlgoViz
```

<br>

## 使用方法(ver. X.X.X)

1. `./algoviz`ディレクトリに移動
```bash
cd ./algoviz
```

<br>

2. `./algoviz`を実行。`-s`オプションで逐次実行、`-h`オプションでヘルプが表示されます。
```bash
./algoviz
```

<br>

3. 実装されているアルゴリズム(現在は基本ソート3種とセグメント木(開発中))が表示されるので、↑↓で選択して`Enter`
```bash
Use ↑ or ↓ to move, Enter to select:
> Basic_Sort
  Segment_Tree
  Quit
```

4. I. Basic_Sort選択時: ソートの種類(バブル、選択、挿入または中止)が表示されるので、↑↓で選択して`Enter`
```bash
Use ↑ or ↓ to move, Enter to select:
> Bubble
  Choose
  Insert
  Quit
```

4. II. Segmen_Tree選択時: そのまま実行されます。

<br>


(初回時のみ、コンパイルが生じるので、待ち時間が長くなると予想されます。ご了承くださいm(_ _)m)

<br>

## 不具合やリクエストのお問い合わせ
お手数をおかけしますが、`Issue`からお問い合わせくださいませ。