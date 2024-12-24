use crate::language::Dict;

// 一行は日本語で18文字まで

pub const HELLO: Dict<&'static str> = Dict {
    ja: "やあ、見慣れない顔だね。\nここは商人キャンプだよ。\n来客は歓迎さ。",
    en: "Hey, you look unfamiliar.\nThis is a rabbit merchant camp.\nVisitors are welcome.",
};

pub const HELLO_RABBITS: Dict<&'static str> = Dict {
    ja: "ほかの仲間たちにもぜひあいさつしていってくれ。",
    en: "Please say hello to the other rabbits.",
};

pub const SINGLEPLAY: Dict<&'static str> = Dict {
    ja: "その魔法陣は地下迷宮の入り口だよ。\n行くなら気をつけてね。\nあなたは魔法使いだから\n大丈夫だと思うけど。",
    en: "This is the entrance to the underground labyrinth.\nBe careful.\nI think you'll be fine\nbecause you are a witch.",
};

pub const HUGE_SLIME: Dict<&'static str> = Dict {
    ja: "最近、地下迷宮にとても大きなスライムが現れてね。\n大暴れして困っているんだ。",
    en: "Lately, a huge slime has appeared in the labyrinth.\nIt's causing a lot of trouble.",
};

pub const HUGE_SLIME2: Dict<&'static str> = Dict {
    ja: "あなたが討伐してくれたら\nとても助かるんだけど。",
    en: "I would be very grateful if you could defeat it.",
};

pub const WITCHES_ARE: Dict<&'static str> = Dict {
    ja: "昔はこの島にも多くのヒト族がいたらしいが、今は魔法使いが時折訪れるくらいさ。君たち魔法使いはこの地底でいったい何を探しているんだい？",
    en: "There used to be many humans on this island but now only witches occasionally visit. What are you witches looking for in the depths?",
};

pub const MULTIPLAY : Dict<&'static str> = Dict {
    ja: "この先はマルチプレイ用レベルだよ。\n気軽に遊んでいってね。\n誰かいるかはわからないけど。",
    en: "It seems that this is a level for multiplayer.\nFeel free to play.\nI don't know if anyone is there.",
};

pub const TRAINING_RABBIT : Dict<&'static str> =  Dict {
    ja: "キミも強くなりたいのかい？\nここで練習していくといい。\nサンドバッグくんたちが相手になってくれる。",
    en: "Do you want to be strong?\nIt's good to practice here.\nThe sandbag guys will be your opponent.",
};

pub const SHOP_RABBIT: Dict<&'static str> = Dict {
    ja: "いらっしゃいませ！\n欲しい商品があったら\n持ってきてくださいね",
    en: "Welcome!\nBring me the item\nyou want to buy",
};

pub fn shop_rabbit(golds: u32) -> Dict<String> {
    Dict {
        ja: format!(
            "合計{}ゴールドのお買い上げです！\nありがとうございます",
            golds
        ),
        en: format!("Your total is {} Golds\nThank you", golds),
    }
}

pub fn too_few_golds(golds: u32) -> Dict<String> {
    Dict {
        ja: format!(
            "おいおい\n{}ゴールド足りませんよ\n買わない商品は\n戻しておいてくださいね",
            golds
        ),
        en: format!(
            "Hey, hey!\nYou are {} Golds short!\nPut it back that you woun't buy",
            golds
        ),
    }
}

// 地名 /////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub const LEVEL0: Dict<&'static str> = Dict {
    ja: "見捨てられた工房",
    en: "Abandoned Workshop",
};

pub const LEVEL1: Dict<&'static str> = Dict {
    ja: "図書館跡",
    en: "Library Ruins",
};

pub const LEVEL2: Dict<&'static str> = Dict {
    ja: "洞窟",
    en: "Cave",
};

pub const LEVEL3: Dict<&'static str> = Dict {
    ja: "スライムの巣窟",
    en: "Slime Nest",
};

pub const MULTIPLAY_ARENA: Dict<&'static str> = Dict {
    ja: "対決の洞窟",
    en: "Arena Cave",
};

pub const UNKNOWN_LEVEL: Dict<&'static str> = Dict {
    ja: "不明",
    en: "Unknown",
};
