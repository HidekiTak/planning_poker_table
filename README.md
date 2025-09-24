# Planning Poker Table
- 
- デザインがいけてないのは承知していますが、個人の能力の範囲外です。

## run service

```shell
> cargo run
```

1. First Player
   ### access this url
   http://localhost:8080/

   ### input table-name and your nickname

   ### distribute this table url

2. Other Players

   ### access table url

   ### check table-name

   ### input your nickname

3. play cards

   set_agenda / set_options / vote / open / reset

## build

### covert.py

convert.sh で resource下のファイルをrustに組み込む。

### build docker image & local run

```shell
> docker build -t planning-poker .
> docker run -p 8080:8080 planning-poker
```

## build binary

### prepare to cross compile

#### Mac

https://github.com/messense/homebrew-macos-cross-toolchains
から必要なバイナリをダウンロードして解凍してください。

